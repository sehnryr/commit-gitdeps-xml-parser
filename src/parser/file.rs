#[macro_export]
macro_rules! file {
    () => {
        ::nom::combinator::map(
            (
                ::nom::bytes::complete::tag::<&str, &str, ::nom::error::Error<&str>>("<File"),
                crate::parser::common::whitespace!(1),
                ::nom::sequence::delimited(
                    ::nom::bytes::complete::tag("Name=\""),
                    ::nom::bytes::complete::is_not("\""),
                    ::nom::character::complete::char('"'),
                ),
                crate::parser::common::whitespace!(1),
                ::nom::sequence::delimited(
                    ::nom::bytes::complete::tag("Hash=\""),
                    crate::parser::common::hash!(),
                    ::nom::character::complete::char('"'),
                ),
                ::nom::combinator::opt((
                    crate::parser::common::whitespace!(1),
                    ::nom::bytes::complete::tag("IsExecutable=\"true\""),
                )),
                crate::parser::common::whitespace!(),
                ::nom::bytes::complete::tag("/>"),
            ),
            |(_, _, name, _, hash, is_executable, _, _)| crate::File {
                name,
                hash,
                is_executable: is_executable.is_some(),
            },
        )
    };
}

#[macro_export]
macro_rules! files {
    () => {
        ::nom::sequence::delimited(
            (
                ::nom::bytes::complete::tag("<Files>"),
                crate::parser::common::whitespace!(),
            ),
            ::nom::multi::fold_many0(
                (crate::parser::file!(), crate::parser::common::whitespace!()),
                Vec::new,
                |mut acc, (file, _)| {
                    acc.push(file);
                    acc
                },
            ),
            ::nom::bytes::complete::tag("</Files>"),
        )
    };
}

pub use {file, files};

#[cfg(test)]
mod tests {
    use nom::Parser;

    use super::*;
    use crate::File;

    #[test]
    fn test_parse_file() {
        let xml =
            r#"<File Name="file/name.one" Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" />"#;

        assert_eq!(
            file!().parse(xml),
            Ok((
                "",
                File {
                    name: "file/name.one",
                    hash: "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                    is_executable: false,
                }
            ))
        );

        let xml = r#"<File Name="file/name.two" Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" IsExecutable="true" />"#;
        assert_eq!(
            file!().parse(xml),
            Ok((
                "",
                File {
                    name: "file/name.two",
                    hash: "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                    is_executable: true,
                }
            ))
        );
    }

    #[test]
    fn test_parse_files() {
        let xml = r#"<Files>
            <File Name="file/name.one" Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" />
            <File Name="file/name.two" Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" IsExecutable="true" />
        </Files>"#;

        assert_eq!(
            files!().parse(xml),
            Ok((
                "",
                vec![
                    File {
                        name: "file/name.one",
                        hash: "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        is_executable: false,
                    },
                    File {
                        name: "file/name.two",
                        hash: "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        is_executable: true,
                    },
                ]
            ))
        );
    }
}
