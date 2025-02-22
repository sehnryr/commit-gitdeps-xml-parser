#[macro_export]
macro_rules! whitespace {
    () => {
        ::nom::bytes::complete::take_while(|c: char| c.is_ascii_whitespace())
    };
    (1) => {
        ::nom::bytes::complete::take_while1(|c: char| c.is_ascii_whitespace())
    };
}

#[macro_export]
macro_rules! number {
    () => {
        ::nom::combinator::map_res(
            ::nom::bytes::complete::take_while1(|c: char| c.is_ascii_digit()),
            |s| u32::from_str_radix(s, 10),
        )
    };
}

#[macro_export]
macro_rules! hash {
    () => {
        ::nom::bytes::complete::take_while_m_n(40, 40, |c: char| c.is_ascii_hexdigit())
    };
}

pub use {hash, number, whitespace};
