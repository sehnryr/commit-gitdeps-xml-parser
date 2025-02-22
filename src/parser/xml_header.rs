use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::{map, opt};
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::{IResult, Parser};

use crate::model::XmlHeader;

use super::hash;

pub fn xml_header(input: &str) -> IResult<&str, XmlHeader> {
    map(
        (
            tag("<?xml"),
            multispace1,
            delimited(tag("version=\""), is_not("\""), char('"')),
            multispace1,
            delimited(tag("encoding=\""), is_not("\""), char('"')),
            multispace0,
            tag("?>"),
        ),
        |(_, _, version, _, encoding, _, _)| XmlHeader::new(version, encoding),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_xml_header() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?>"#;

        assert_eq!(xml_header(xml), Ok(("", XmlHeader::new("1.0", "utf-8"))));
    }
}
