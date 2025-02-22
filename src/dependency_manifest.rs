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
        let xml = r#"<DependencyManifest xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" BaseUrl="https://cdn.unrealengine.com/dependencies">
            <Files>
                <File Name=".tgitconfig" Hash="8fa70533aa3bc6aee606776237b8d1b3ed16d315" />
                <File Name="Engine/Binaries/DotNET/Android/UnrealAndroidFileTool/linux-x64/UnrealAndroidFileTool" Hash="c8214157f458f132d684e80eaa824e94a2c12a4d" IsExecutable="true" />
            </Files>
            <Blobs>
                <Blob Hash="0000171f05076eae0997dd93d592ec19e9fa4827" Size="589615" PackHash="12de9b5f80b252f1a94457c523985aeb4e15df51" PackOffset="1148624" />
            </Blobs>
            <Packs>
                <Pack Hash="0007e862705001fbab3fcf58eb2b059da1e9f872" Size="2005632" CompressedSize="361451" RemotePath="UnrealEngine-25328963" />
            </Packs>
        </DependencyManifest>"#;

        let manifest = dependency_manifest(xml).unwrap().1;
        assert_eq!(
            manifest.base_url,
            "https://cdn.unrealengine.com/dependencies"
        );
        assert_eq!(manifest.files.len(), 2);
        assert_eq!(manifest.blobs.len(), 1);
        assert_eq!(manifest.packs.len(), 1);
    }
}
