#[macro_export]
macro_rules! xml_header {
    () => {
        ::nom::combinator::map(
            (
                ::nom::bytes::complete::tag::<&str, &str, ::nom::error::Error<&str>>("<?xml"),
                crate::parser::common::whitespace!(1),
                ::nom::sequence::delimited(
                    ::nom::bytes::complete::tag("version=\""),
                    ::nom::bytes::complete::is_not("\""),
                    ::nom::character::complete::char('"'),
                ),
                crate::parser::common::whitespace!(1),
                ::nom::sequence::delimited(
                    ::nom::bytes::complete::tag("encoding=\""),
                    ::nom::bytes::complete::is_not("\""),
                    ::nom::character::complete::char('"'),
                ),
                crate::parser::common::whitespace!(),
                ::nom::bytes::complete::tag("?>"),
            ),
            |(_, _, version, _, encoding, _, _)| crate::XmlHeader { version, encoding },
        )
    };
}

pub use xml_header;

#[cfg(test)]
mod tests {
    use nom::Parser;

    use super::*;
    use crate::XmlHeader;

    #[test]
    fn test_parse_xml_header() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?>"#;

        assert_eq!(
            xml_header!().parse(xml),
            Ok((
                "",
                XmlHeader {
                    version: "1.0",
                    encoding: "utf-8"
                }
            ))
        );
    }
}
