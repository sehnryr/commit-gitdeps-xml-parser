mod args;
mod model;
mod parser;

use clap::Parser;
use tokio::fs::OpenOptions;
use tokio::io;

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

    let mut content_reader = git_deps.get_file_content(&file).await?;

    io::copy(&mut content_reader, &mut output_file).await?;

    Ok(())
}
