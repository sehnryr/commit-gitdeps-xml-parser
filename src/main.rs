mod args;
mod model;
mod parser;

use clap::Parser as ClapParser;
use nom::Parser as NomParser;
use nom::character::complete::multispace0;

use crate::args::Args;
use crate::parser::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let buffer = std::fs::read_to_string(args.xml_path())?;

    let (_, (_, _, (dependency_manifest, (files, _, blobs, _, packs)))) = (
        xml_header,
        multispace0,
        dependency_manifest((files, multispace0, blobs, multispace0, packs)),
    )
        .parse(&buffer)
        .unwrap();

    let file = files.get(args.file()).ok_or("File not found")?;
    let blob = blobs.get(file.hash()).ok_or("Blob not found")?;
    let pack = packs.get(blob.pack_hash()).ok_or("Pack not found")?;

    let url = format!(
        "{base_url}/{remote_path}/{pack_hash}",
        base_url = dependency_manifest.base_url(),
        remote_path = pack.remote_path(),
        pack_hash = blob.pack_hash()
    );

    println!("{}", url);

    Ok(())
}
