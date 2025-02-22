use nom::bytes::complete::{is_not, tag, take_while, take_while_m_n, take_while1};
use nom::combinator::map_res;
use nom::sequence::delimited;
use nom::{IResult, Parser};

pub fn whitespace0(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_ascii_whitespace()).parse(input)
}

pub fn whitespace1(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_whitespace()).parse(input)
}

pub fn number(input: &str) -> IResult<&str, u32> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s| {
        u32::from_str_radix(s, 10)
    })
    .parse(input)
}

pub fn hash(input: &str) -> IResult<&str, &str> {
    take_while_m_n(40, 40, |c: char| c.is_ascii_hexdigit()).parse(input)
}

pub fn xml_header(input: &str) -> IResult<&str, &str> {
    delimited(tag("<?xml"), is_not("?>"), tag("?>")).parse(input)
}
