use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, multispace0, multispace1, u32};
use nom::combinator::{map, opt};
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::{IResult, Parser};

use crate::model::Blob;

use super::hash;

pub fn blob(input: &str) -> IResult<&str, Blob> {
    map(
        (
            tag("<Blob"),
            multispace1,
            delimited(tag("Hash=\""), hash, char('"')),
            multispace1,
            delimited(tag("Size=\""), u32, char('"')),
            multispace1,
            delimited(tag("PackHash=\""), hash, char('"')),
            multispace1,
            delimited(tag("PackOffset=\""), u32, char('"')),
            multispace0,
            tag("/>"),
        ),
        |(_, _, hash, _, size, _, pack_hash, _, pack_offset, _, _)| {
            Blob::new(hash, size, pack_hash, pack_offset)
        },
    )
    .parse(input)
}

pub fn blobs(input: &str) -> IResult<&str, Vec<Blob>> {
    delimited(
        (tag("<Blobs>"), multispace0),
        ::nom::multi::fold_many0(
            (crate::parser::blob, multispace0),
            Vec::new,
            |mut acc, (blob, _)| {
                acc.push(blob);
                acc
            },
        ),
        (tag("</Blobs>"), multispace0),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_blob() {
        let xml = r#"<Blob Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" Size="123456" PackHash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" PackOffset="123456" />"#;

        assert_eq!(
            blob.parse(xml),
            Ok((
                "",
                Blob::new(
                    "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                    123456,
                    "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                    123456,
                )
            ))
        );
    }

    #[test]
    fn test_parse_blobs() {
        let xml = r#"<Blobs>
            <Blob Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" Size="123456" PackHash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" PackOffset="123456" />
            <Blob Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" Size="123456" PackHash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" PackOffset="123456" />
        </Blobs>"#;

        assert_eq!(
            blobs.parse(xml),
            Ok((
                "",
                vec![
                    Blob::new(
                        "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        123456,
                        "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        123456,
                    ),
                    Blob::new(
                        "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        123456,
                        "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        123456,
                    ),
                ]
            ))
        );
    }
}
