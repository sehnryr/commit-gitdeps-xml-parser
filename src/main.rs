mod parser;

use nom::Parser;
use nom::character::complete::multispace0;

use parser::*;

#[derive(Debug, PartialEq)]
struct XmlHeader<'a> {
    version: &'a str,
    encoding: &'a str,
}

#[derive(Debug, PartialEq)]
struct DependencyManifest<'a> {
    xml_schema_definition_namespace: &'a str,
    xml_schema_instance_namespace: &'a str,
    base_url: &'a str,
}

#[derive(Debug, PartialEq)]
struct File<'a> {
    name: &'a str,
    hash: &'a str,
    is_executable: bool,
}

#[derive(Debug, PartialEq)]
struct Blob<'a> {
    hash: &'a str,
    size: u32,
    pack_hash: &'a str,
    pack_offset: u32,
}

#[derive(Debug, PartialEq)]
struct Pack<'a> {
    hash: &'a str,
    size: u32,
    compressed_size: u32,
    remote_path: &'a str,
}

fn main() -> Result<(), std::io::Error> {
    let buffer = std::fs::read_to_string("Commit.gitdeps.xml")?;

    let (input, (_, _, (dependency_manifest, (files, _, blobs, _, packs)))) = (
        xml_header,
        multispace0,
        dependency_manifest((files, multispace0, blobs, multispace0, packs)),
    )
        .parse(&buffer)
        .unwrap();

    assert_eq!(input.len(), 0);

    println!("base url: {}", dependency_manifest.base_url);
    println!("files count: {}", files.len());
    println!("packs count: {}", packs.len());
    println!("blobs count: {}", blobs.len());

    Ok(())
}
