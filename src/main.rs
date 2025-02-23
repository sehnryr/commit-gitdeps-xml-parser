mod args;
mod model;
mod parser;

use std::fs::OpenOptions;
use std::io::Write;

use clap::Parser as ClapParser;
use flate2::write::GzDecoder;
use nom::Parser as NomParser;
use nom::character::complete::multispace0;

use crate::args::Args;
use crate::parser::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let buffer = std::fs::read_to_string(args.xml_path())?;

    let (_, (_, _, (dependency_manifest, (files, _, blobs, _, packs)))) = (
        xml_header,
        multispace0,
        dependency_manifest((files, multispace0, blobs, multispace0, packs)),
    )
        .parse(&buffer)
        .unwrap();

    let file_path_str = args.file().to_str().ok_or("File not found")?;
    let file_name = args.file().file_name().ok_or("File name not found")?;

    let file = files.get(file_path_str).ok_or("File not found")?;
    let blob = blobs.get(file.hash()).ok_or("Blob not found")?;
    let pack = packs.get(blob.pack_hash()).ok_or("Pack not found")?;

    let url = format!(
        "{base_url}/{remote_path}/{pack_hash}",
        base_url = dependency_manifest.base_url(),
        remote_path = pack.remote_path(),
        pack_hash = blob.pack_hash()
    );

    println!("{}", url);

    let response = reqwest::get(url).await?;
    let compressed = response.bytes().await?;

    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name)?;

    let mut decoder = GzDecoder::new(output_file);
    decoder.write_all(&compressed)?;
    decoder.finish()?;

    Ok(())
}
