use nom::bytes::complete::{take_while, take_while_m_n, take_while1};
use nom::combinator::map_res;
use nom::{IResult, Parser};

pub fn hash(input: &str) -> IResult<&str, &str> {
    take_while_m_n(40, 40, |c: char| c.is_ascii_hexdigit())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(
            hash("a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4"),
            Ok(("", "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4"))
        );
    }
}
