use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to Commit.gitdeps.xml
    xml_path: PathBuf,

    /// File to download
    #[arg(short, long)]
    file: PathBuf,

    /// Output directory
    #[arg(short, long, default_value = ".")]
    output_dir: PathBuf,
}

impl Args {
    pub fn xml_path(&self) -> &PathBuf {
        &self.xml_path
    }

    pub fn file(&self) -> &PathBuf {
        &self.file
    }

    pub fn output_dir(&self) -> &PathBuf {
        &self.output_dir
    }
}
