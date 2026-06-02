use anyhow::Result;
use haki::arguments::Args;
use haki::scan_endpoints;
use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();
    let root = args.get_path();
    let endpoints = scan_endpoints(&root)?;
    println!("{}", serde_json::to_string_pretty(&endpoints)?);
    Ok(())
}
