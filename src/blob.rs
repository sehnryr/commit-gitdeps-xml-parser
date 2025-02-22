use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::{IResult, Parser};

use crate::parse::{hash, number, whitespace0, whitespace1};

#[derive(Debug, PartialEq)]
pub struct Blob<'a> {
    pub hash: &'a str,
    pub size: u32,
    pub pack_hash: &'a str,
    pub pack_offset: u32,
}

pub fn blob(input: &str) -> IResult<&str, Blob> {
    let (input, (hash, size, pack_hash, pack_offset)) = delimited(
        (tag("<Blob"), whitespace1),
        (
            delimited(tag("Hash=\""), hash, (char('"'), whitespace1)),
            delimited(tag("Size=\""), number, (char('"'), whitespace1)),
            delimited(tag("PackHash=\""), hash, (char('"'), whitespace1)),
            delimited(tag("PackOffset=\""), number, char('"')),
        ),
        (whitespace0, tag("/>")),
    )
    .parse(input)?;

    Ok((
        input,
        Blob {
            hash,
            size,
            pack_hash,
            pack_offset,
        },
    ))
}

pub fn blobs(input: &str) -> IResult<&str, Vec<Blob>> {
    delimited(
        (tag("<Blobs>"), whitespace0),
        fold_many0((blob, whitespace0), Vec::new, |mut acc, (blob, _)| {
            acc.push(blob);
            acc
        }),
        tag("</Blobs>"),
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
            blob(xml),
            Ok((
                "",
                Blob {
                    hash: "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                    size: 123456,
                    pack_hash: "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                    pack_offset: 123456,
                }
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
            blobs(xml),
            Ok((
                "",
                vec![
                    Blob {
                        hash: "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        size: 123456,
                        pack_hash: "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        pack_offset: 123456,
                    },
                    Blob {
                        hash: "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        size: 123456,
                        pack_hash: "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        pack_offset: 123456,
                    },
                ]
            ))
        );
    }
}
