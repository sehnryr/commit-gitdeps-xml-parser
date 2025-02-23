use std::collections::HashMap;

use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, multispace0, multispace1, u32};
use nom::combinator::{map, opt};
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::{IResult, Parser};

use crate::model::{Blob, BlobMap};

use super::hash;

pub fn blob(input: &str) -> IResult<&str, Blob> {
    map(
        (
            tag("<Blob"),
            multispace1,
            delimited(tag("Hash=\""), hash, char('"')),
            multispace1,
            delimited(tag("Size=\""), u32, char('"')),
            multispace1,
            delimited(tag("PackHash=\""), hash, char('"')),
            multispace1,
            delimited(tag("PackOffset=\""), u32, char('"')),
            multispace0,
            tag("/>"),
        ),
        |(_, _, hash, _, size, _, pack_hash, _, pack_offset, _, _)| {
            Blob::new(hash, size, pack_hash, pack_offset)
        },
    )
    .parse(input)
}

pub fn blobs(input: &str) -> IResult<&str, BlobMap> {
    delimited(
        (tag("<Blobs>"), multispace0),
        fold_many0((blob, multispace0), BlobMap::new, |mut acc, (blob, _)| {
            acc.insert(blob);
            acc
        }),
        (tag("</Blobs>"), multispace0),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_blob() {
        let xml = r#"<Blob Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" Size="123456" PackHash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" PackOffset="123456" />"#;

        assert_eq!(
            blob.parse(xml),
            Ok((
                "",
                Blob::new(
                    "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                    123456,
                    "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                    123456,
                )
            ))
        );
    }

    #[test]
    fn test_parse_blobs() {
        let xml = r#"<Blobs>
            <Blob Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" Size="123456" PackHash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" PackOffset="123456" />
            <Blob Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a5" Size="123456" PackHash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a5" PackOffset="123456" />
        </Blobs>"#;

        assert_eq!(
            blobs.parse(xml),
            Ok(("", {
                let mut blob_map = BlobMap::new();
                blob_map.insert(Blob::new(
                    "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                    123456,
                    "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4",
                    123456,
                ));
                blob_map.insert(Blob::new(
                    "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a5",
                    123456,
                    "a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a5",
                    123456,
                ));
                blob_map
            }))
        );
    }
}
