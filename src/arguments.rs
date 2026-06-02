use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the file to check
    #[arg(short, long)]
    path: PathBuf,
}

impl Args {
    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }
}
