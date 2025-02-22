mod model;
mod parser;

use nom::Parser;
use nom::character::complete::multispace0;

use parser::*;

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

    println!("base url: {}", dependency_manifest.base_url());
    println!("files count: {}", files.len());
    println!("packs count: {}", packs.len());
    println!("blobs count: {}", blobs.len());

    Ok(())
}
