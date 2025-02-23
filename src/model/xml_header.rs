#[derive(Debug, PartialEq)]
pub struct XmlHeader<'a> {
    version: &'a str,
    encoding: &'a str,
}

impl<'a> XmlHeader<'a> {
    pub fn new(version: &'a str, encoding: &'a str) -> Self {
        Self { version, encoding }
    }
}
