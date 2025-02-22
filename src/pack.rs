use nom::bytes::complete::{is_not, tag};
use nom::character::complete::char;
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::{IResult, Parser};

use crate::parse::{hash, number, whitespace0, whitespace1};

#[derive(Debug, PartialEq)]
pub struct Pack<'a> {
    pub hash: &'a str,
    pub size: u32,
    pub compressed_size: u32,
    pub remote_path: &'a str,
}

pub fn pack(input: &str) -> IResult<&str, Pack> {
    let (input, (hash, size, compressed_size, remote_path)) = delimited(
        (tag("<Pack"), whitespace1),
        (
            delimited(tag("Hash=\""), hash, (char('"'), whitespace1)),
            delimited(tag("Size=\""), number, (char('"'), whitespace1)),
            delimited(tag("CompressedSize=\""), number, (char('"'), whitespace1)),
            delimited(tag("RemotePath=\""), is_not("\""), char('"')),
        ),
        (whitespace0, tag("/>")),
    )
    .parse(input)?;

    Ok((
        input,
        Pack {
            hash,
            size,
            compressed_size,
            remote_path,
        },
    ))
}

pub fn packs(input: &str) -> IResult<&str, Vec<Pack>> {
    delimited(
        (tag("<Packs>"), whitespace0),
        fold_many0((pack, whitespace0), Vec::new, |mut acc, (pack, _)| {
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
                Pack {
                    hash: "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                    size: 123456,
                    compressed_size: 123456,
                    remote_path: "Remote-123456",
                }
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
                    Pack {
                        hash: "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        size: 123456,
                        compressed_size: 123456,
                        remote_path: "Remote-123456",
                    },
                    Pack {
                        hash: "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        size: 123456,
                        compressed_size: 123456,
                        remote_path: "Remote-123456",
                    },
                ]
            ))
        );
    }
}
