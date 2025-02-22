mod blob;
mod dependency_manifest;
mod file;
mod pack;
mod parse;

use nom::Parser;
use parse::whitespace0;

use crate::dependency_manifest::dependency_manifest;
use crate::parse::xml_header;

fn main() -> Result<(), std::io::Error> {
    let buffer = std::fs::read_to_string("Commit.gitdeps.xml")?;

    let (input, (_, _, dependency_manifest)) = (xml_header, whitespace0, dependency_manifest)
        .parse(&buffer)
        .unwrap();

    assert_eq!(input.len(), 0);

    println!("base url: {}", dependency_manifest.base_url);
    println!("files count: {}", dependency_manifest.files.len());
    println!("packs count: {}", dependency_manifest.packs.len());
    println!("blobs count: {}", dependency_manifest.blobs.len());

    Ok(())
}
