#[macro_export]
macro_rules! blob {
    () => {
        ::nom::combinator::map(
            (
                ::nom::bytes::complete::tag::<&str, &str, ::nom::error::Error<&str>>("<Blob"),
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
                    ::nom::bytes::complete::tag("PackHash=\""),
                    crate::parser::common::hash!(),
                    ::nom::character::complete::char('"'),
                ),
                crate::parser::common::whitespace!(1),
                ::nom::sequence::delimited(
                    ::nom::bytes::complete::tag("PackOffset=\""),
                    crate::parser::common::number!(),
                    ::nom::character::complete::char('"'),
                ),
                crate::parser::common::whitespace!(),
                ::nom::bytes::complete::tag("/>"),
            ),
            |(_, _, hash, _, size, _, pack_hash, _, pack_offset, _, _)| crate::Blob {
                hash,
                size,
                pack_hash,
                pack_offset,
            },
        )
    };
}

#[macro_export]
macro_rules! blobs {
    () => {
        ::nom::sequence::delimited(
            (
                ::nom::bytes::complete::tag("<Blobs>"),
                crate::parser::common::whitespace!(),
            ),
            ::nom::multi::fold_many0(
                (crate::parser::blob!(), crate::parser::common::whitespace!()),
                Vec::new,
                |mut acc, (blob, _)| {
                    acc.push(blob);
                    acc
                },
            ),
            ::nom::bytes::complete::tag("</Blobs>"),
        )
    };
}

pub use {blob, blobs};

#[cfg(test)]
mod tests {
    use nom::Parser;

    use super::*;
    use crate::Blob;

    #[test]
    fn test_parse_blob() {
        let xml = r#"<Blob Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" Size="123456" PackHash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" PackOffset="123456" />"#;

        assert_eq!(
            blob!().parse(xml),
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
            blobs!().parse(xml),
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
