use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::{map, opt};
use nom::error::Error;
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::{IResult, Parser};

use crate::model::DependencyManifest;

use super::hash;

pub fn dependency_manifest<'a, O, F>(
    inner: F,
) -> impl Parser<&'a str, Output = (DependencyManifest<'a>, O), Error = Error<&'a str>>
where
    F: Parser<&'a str, Output = O, Error = Error<&'a str>>,
{
    map(
        (
            tag("<DependencyManifest"),
            multispace1,
            delimited(tag("xmlns:xsd=\""), is_not("\""), char('"')),
            multispace1,
            delimited(tag("xmlns:xsi=\""), is_not("\""), char('"')),
            multispace1,
            delimited(tag("BaseUrl=\""), is_not("\""), char('"')),
            multispace0,
            char('>'),
            multispace0,
            inner,
            multispace0,
            tag("</DependencyManifest>"),
        ),
        |(
            _,
            _,
            xml_schema_definition_namespace,
            _,
            xml_schema_instance_namespace,
            _,
            base_url,
            _,
            _,
            _,
            inner,
            _,
            _,
        )| {
            (
                DependencyManifest::new(
                    xml_schema_definition_namespace,
                    xml_schema_instance_namespace,
                    base_url,
                ),
                inner,
            )
        },
    )
}

#[cfg(test)]
mod tests {
    use nom::Parser;

    use super::*;

    #[test]
    fn test_parse_dependency_manifest() {
        let xml = r#"<DependencyManifest xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" BaseUrl="https://example.com"></DependencyManifest>"#;

        assert_eq!(
            dependency_manifest(multispace0).parse(xml),
            Ok((
                "",
                (
                    DependencyManifest::new(
                        "http://www.w3.org/2001/XMLSchema",
                        "http://www.w3.org/2001/XMLSchema-instance",
                        "https://example.com",
                    ),
                    ""
                )
            ))
        );
    }
}
