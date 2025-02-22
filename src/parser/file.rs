use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::{map, opt};
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::{IResult, Parser};

use crate::model::File;

use super::hash;

pub fn file(input: &str) -> IResult<&str, File> {
    map(
        (
            tag("<File"),
            multispace1,
            delimited(tag("Name=\""), is_not("\""), char('"')),
            multispace1,
            delimited(tag("Hash=\""), hash, char('"')),
            opt((multispace1, tag("IsExecutable=\"true\""))),
            multispace0,
            tag("/>"),
        ),
        |(_, _, name, _, hash, is_executable, _, _)| File::new(name, hash, is_executable.is_some()),
    )
    .parse(input)
}

pub fn files(input: &str) -> IResult<&str, Vec<File>> {
    delimited(
        (tag("<Files>"), multispace0),
        fold_many0((file, multispace0), Vec::new, |mut acc, (file, _)| {
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
                File::new(
                    "file/name.one",
                    "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                    false
                )
            ))
        );

        let xml = r#"<File Name="file/name.two" Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" IsExecutable="true" />"#;
        assert_eq!(
            file(xml),
            Ok((
                "",
                File::new(
                    "file/name.two",
                    "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                    true,
                )
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
                    File::new(
                        "file/name.one",
                        "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        false,
                    ),
                    File::new(
                        "file/name.two",
                        "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                        true,
                    ),
                ]
            ))
        );
    }
}
