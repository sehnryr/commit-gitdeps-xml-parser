#[macro_export]
macro_rules! dependency_manifest {
    ($f:expr) => {
        ::nom::combinator::map(
            (
                ::nom::bytes::complete::tag::<&str, &str, ::nom::error::Error<&str>>(
                    "<DependencyManifest",
                ),
                crate::parser::common::whitespace!(1),
                ::nom::sequence::delimited(
                    ::nom::bytes::complete::tag("xmlns:xsd=\""),
                    ::nom::bytes::complete::is_not("\""),
                    ::nom::character::complete::char('"'),
                ),
                crate::parser::common::whitespace!(1),
                ::nom::sequence::delimited(
                    ::nom::bytes::complete::tag("xmlns:xsi=\""),
                    ::nom::bytes::complete::is_not("\""),
                    ::nom::character::complete::char('"'),
                ),
                crate::parser::common::whitespace!(1),
                ::nom::sequence::delimited(
                    ::nom::bytes::complete::tag("BaseUrl=\""),
                    ::nom::bytes::complete::is_not("\""),
                    ::nom::character::complete::char('"'),
                ),
                crate::parser::common::whitespace!(),
                ::nom::character::complete::char('>'),
                crate::parser::common::whitespace!(),
                $f,
                crate::parser::common::whitespace!(),
                ::nom::bytes::complete::tag("</DependencyManifest>"),
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
                f,
                _,
                _,
            )| {
                (
                    crate::DependencyManifest {
                        xml_schema_definition_namespace,
                        xml_schema_instance_namespace,
                        base_url,
                    },
                    f,
                )
            },
        )
    };
}

pub use dependency_manifest;

#[cfg(test)]
mod tests {
    use nom::Parser;

    use super::*;
    use crate::DependencyManifest;

    #[test]
    fn test_parse_dependency_manifest() {
        let xml = r#"<DependencyManifest xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" BaseUrl="https://example.com"></DependencyManifest>"#;

        assert_eq!(
            dependency_manifest!(crate::parser::common::whitespace!()).parse(xml),
            Ok((
                "",
                (
                    DependencyManifest {
                        xml_schema_definition_namespace: "http://www.w3.org/2001/XMLSchema",
                        xml_schema_instance_namespace: "http://www.w3.org/2001/XMLSchema-instance",
                        base_url: "https://example.com",
                    },
                    ""
                )
            ))
        );
    }
}
