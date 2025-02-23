use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to Commit.gitdeps.xml
    xml_path: PathBuf,

    /// File to download
    #[arg(short, long)]
    file: String,
}

impl Args {
    pub fn xml_path(&self) -> &PathBuf {
        &self.xml_path
    }

    pub fn file(&self) -> &str {
        &self.file
    }
}
