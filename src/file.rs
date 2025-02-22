use nom::bytes::complete::{is_not, tag};
use nom::character::complete::char;
use nom::combinator::opt;
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::{IResult, Parser};

use crate::parse::{hash, whitespace0, whitespace1};

#[derive(Debug, PartialEq)]
pub struct File<'a> {
    pub name: &'a str,
    pub hash: &'a str,
    pub is_executable: bool,
}

pub fn file(input: &str) -> IResult<&str, File> {
    let (input, (name, hash, is_executable)) = delimited(
        (tag("<File"), whitespace1),
        (
            delimited(tag("Name=\""), is_not("\""), (char('"'), whitespace1)),
            delimited(tag("Hash=\""), hash, char('"')),
            opt((whitespace1, tag("IsExecutable=\"true\""))),
        ),
        (whitespace0, tag("/>")),
    )
    .parse(input)?;

    Ok((
        input,
        File {
            name,
            hash,
            is_executable: is_executable.is_some(),
        },
    ))
}

pub fn files(input: &str) -> IResult<&str, Vec<File>> {
    delimited(
        (tag("<Files>"), whitespace0),
        fold_many0((file, whitespace0), Vec::new, |mut acc, (file, _)| {
            acc.push(file);
            acc
        }),
        tag("</Files>"),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let xml =
            r#"<File Name="file/name.one" Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" />"#;

        assert_eq!(
            file(xml),
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
            file(xml),
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
            files(xml),
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
