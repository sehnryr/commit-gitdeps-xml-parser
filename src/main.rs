mod args;
mod model;
mod parser;

use std::fs::OpenOptions;
use std::io::Write;

use clap::Parser as ClapParser;
use flate2::write::GzDecoder;

use crate::args::Args;
use crate::model::GitDeps;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let buffer = std::fs::read_to_string(args.xml_path())?;

    let git_deps = GitDeps::new(&buffer).unwrap();

    let file_path_str = args.file().to_str().ok_or("File not found")?;
    let file_name = args.file().file_name().ok_or("File name not found")?;

    let url = git_deps
        .get_file_url(file_path_str)
        .ok_or("File not found")?;

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
