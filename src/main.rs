mod args;
mod model;
mod parser;

use clap::Parser as ClapParser;
use nom::Parser as NomParser;
use nom::character::complete::multispace0;

use crate::args::Args;
use crate::model::FileKey;
use crate::parser::*;

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let buffer = std::fs::read_to_string(args.xml_path())?;

    let (input, (_, _, (dependency_manifest, (files, _, blobs, _, packs)))) = (
        xml_header,
        multispace0,
        dependency_manifest((files, multispace0, blobs, multispace0, packs)),
    )
        .parse(&buffer)
        .unwrap();

    let file = files.get(&FileKey::new(args.file().to_str().unwrap_or_default()));

    println!("{:?}", file);

    Ok(())
}
