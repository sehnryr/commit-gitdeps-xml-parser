use nom::bytes::complete::{is_not, tag};
use nom::character::complete::char;
use nom::combinator::opt;
use nom::sequence::delimited;
use nom::{IResult, Parser};

use crate::blob::{Blob, blobs};
use crate::file::{File, files};
use crate::pack::{Pack, packs};
use crate::parse::{whitespace0, whitespace1};

#[derive(Debug)]
pub struct DependencyManifest<'a> {
    pub base_url: &'a str,
    pub files: Vec<File<'a>>,
    pub blobs: Vec<Blob<'a>>,
    pub packs: Vec<Pack<'a>>,
}

pub fn dependency_manifest(input: &str) -> IResult<&str, DependencyManifest> {
    let (input, (_, _, base_url)) = delimited(
        (tag("<DependencyManifest"), whitespace1),
        (
            opt(delimited(
                tag("xmlns:xsd=\""),
                is_not("\""),
                (char('"'), whitespace1),
            )),
            opt(delimited(
                tag("xmlns:xsi=\""),
                is_not("\""),
                (char('"'), whitespace1),
            )),
            delimited(tag("BaseUrl=\""), is_not("\""), (char('"'), whitespace0)),
        ),
        (char('>'), whitespace0),
    )
    .parse(input)?;

    let (input, (files, _)) = (files, whitespace0).parse(input)?;
    let (input, (blobs, _)) = (blobs, whitespace0).parse(input)?;
    let (input, (packs, _)) = (packs, whitespace0).parse(input)?;

    let (input, _) = tag("</DependencyManifest>").parse(input)?;

    Ok((
        input,
        DependencyManifest {
            base_url,
            files,
            blobs,
            packs,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dependency_manifest() {
        let xml = r#"<DependencyManifest xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" BaseUrl="https://example.com">
            <Files>
                <File Name="file/name.one" Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" />
                <File Name="file/name.two" Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" IsExecutable="true" />
            </Files>
            <Blobs>
                <Blob Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" Size="123456" PackHash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" PackOffset="123456" />
            </Blobs>
            <Packs>
                <Pack Hash="a3f5b7c8d9e0f1a2b3c4d5e6f7a8b9c0d1e2f3a4" Size="123456" CompressedSize="361451" RemotePath="Remote-123456" />
            </Packs>
        </DependencyManifest>"#;

        let manifest = dependency_manifest(xml).unwrap().1;
        assert_eq!(manifest.base_url, "https://example.com");
        assert_eq!(manifest.files.len(), 2);
        assert_eq!(manifest.blobs.len(), 1);
        assert_eq!(manifest.packs.len(), 1);
    }
}
