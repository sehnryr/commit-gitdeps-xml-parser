use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, multispace0, multispace1, u32};
use nom::combinator::{map, opt};
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::{IResult, Parser};

use crate::model::Pack;

use super::hash;

pub fn pack(input: &str) -> IResult<&str, Pack> {
    map(
        (
            tag("<Pack"),
            multispace1,
            delimited(tag("Hash=\""), hash, char('"')),
            multispace1,
            delimited(tag("Size=\""), u32, char('"')),
            multispace1,
            delimited(tag("CompressedSize=\""), u32, char('"')),
            multispace1,
            delimited(tag("RemotePath=\""), is_not("\""), char('"')),
            multispace0,
            tag("/>"),
        ),
        |(_, _, hash, _, size, _, compressed_size, _, remote_path, _, _)| {
            Pack::new(hash, size, compressed_size, remote_path)
        },
    )
    .parse(input)
}

pub fn packs(input: &str) -> IResult<&str, Vec<Pack>> {
    delimited(
        (tag("<Packs>"), multispace0),
        fold_many0((pack, multispace0), Vec::new, |mut acc, (pack, _)| {
            acc.push(pack);
            acc
        }),
        tag("</Packs>"),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pack() {
        let xml = r#"<Pack Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" Size="123456" CompressedSize="123456" RemotePath="Remote-123456" />"#;

        assert_eq!(
            pack(xml),
            Ok((
                "",
                Pack::new(
                    "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                    123456,
                    123456,
                    "Remote-123456",
                )
            ))
        );
    }

    #[test]
    fn test_parse_packs() {
        let xml = r#"<Packs>
            <Pack Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" Size="123456" CompressedSize="123456" RemotePath="Remote-123456" />
            <Pack Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" Size="123456" CompressedSize="123456" RemotePath="Remote-123456" />
        </Packs>"#;

        assert_eq!(
            packs(xml),
            Ok((
                "",
                vec![
                    Pack::new(
                        "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        123456,
                        123456,
                        "Remote-123456",
                    ),
                    Pack::new(
                        "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        123456,
                        123456,
                        "Remote-123456",
                    ),
                ]
            ))
        );
    }
}
