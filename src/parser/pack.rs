#[macro_export]
macro_rules! pack {
    () => {
        ::nom::combinator::map(
            (
                ::nom::bytes::complete::tag::<&str, &str, ::nom::error::Error<&str>>("<Pack"),
                crate::parser::common::whitespace!(1),
                ::nom::sequence::delimited(
                    ::nom::bytes::complete::tag("Hash=\""),
                    crate::parser::common::hash!(),
                    ::nom::character::complete::char('"'),
                ),
                crate::parser::common::whitespace!(1),
                ::nom::sequence::delimited(
                    ::nom::bytes::complete::tag("Size=\""),
                    crate::parser::common::number!(),
                    ::nom::character::complete::char('"'),
                ),
                crate::parser::common::whitespace!(1),
                ::nom::sequence::delimited(
                    ::nom::bytes::complete::tag("CompressedSize=\""),
                    crate::parser::common::number!(),
                    ::nom::character::complete::char('"'),
                ),
                crate::parser::common::whitespace!(1),
                ::nom::sequence::delimited(
                    ::nom::bytes::complete::tag("RemotePath=\""),
                    ::nom::bytes::complete::is_not("\""),
                    ::nom::character::complete::char('"'),
                ),
                crate::parser::common::whitespace!(),
                ::nom::bytes::complete::tag("/>"),
            ),
            |(_, _, hash, _, size, _, compressed_size, _, remote_path, _, _)| crate::Pack {
                hash,
                size,
                compressed_size,
                remote_path,
            },
        )
    };
}

#[macro_export]
macro_rules! packs {
    () => {
        ::nom::sequence::delimited(
            (
                ::nom::bytes::complete::tag("<Packs>"),
                crate::parser::common::whitespace!(),
            ),
            ::nom::multi::fold_many0(
                (crate::parser::pack!(), crate::parser::common::whitespace!()),
                Vec::new,
                |mut acc, (pack, _)| {
                    acc.push(pack);
                    acc
                },
            ),
            ::nom::bytes::complete::tag("</Packs>"),
        )
    };
}

pub use {pack, packs};

#[cfg(test)]
mod tests {
    use nom::Parser;

    use super::*;
    use crate::Pack;

    #[test]
    fn test_parse_pack() {
        let xml = r#"<Pack Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" Size="123456" CompressedSize="123456" RemotePath="Remote-123456" />"#;

        assert_eq!(
            pack!().parse(xml),
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
            packs!().parse(xml),
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
