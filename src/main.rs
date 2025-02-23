mod args;
mod model;
mod parser;

use async_compression::tokio::bufread::GzipDecoder;
use clap::Parser as ClapParser;
use futures_util::TryStreamExt;
use tokio::fs::OpenOptions;
use tokio::io;
use tokio_util::io::StreamReader;

use crate::args::Args;
use crate::model::GitDeps;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let buffer = std::fs::read_to_string(args.xml_path())?;

    let git_deps = GitDeps::new(&buffer).unwrap();

    let file_path_str = args.file().to_str().ok_or("File not found")?;
    let file_name = args
        .output_dir()
        .join(args.file().file_name().ok_or("File name not found")?);

    let file = git_deps.get_file(file_path_str).unwrap();

    let url = git_deps.get_file_url(&file).ok_or("File not found")?;

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name)
        .await?;

    if file.is_executable() {
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::PermissionsExt;

            let metadata = output_file.metadata().await?;
            let mut permissions = metadata.permissions();

            permissions.set_mode(permissions.mode() | 0o100);

            output_file.set_permissions(permissions).await?;
        }
    }

    let response = reqwest::get(url).await?;
    let byte_stream = response
        .bytes_stream()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e));

    let stream_reader = StreamReader::new(byte_stream);
    let buf_reader = io::BufReader::new(stream_reader);

    let mut decoder = GzipDecoder::new(buf_reader);

    io::copy(&mut decoder, &mut output_file).await?;

    Ok(())
}
