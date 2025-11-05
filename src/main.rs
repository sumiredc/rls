use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = match args.path {
        Some(p) => p,
        None => PathBuf::new().join("."),
    };

    let entries =
        fs::read_dir(&path).context(format!("Failed to read directory '{}'", path.display()))?;

    for e in entries {
        let entry = e.context("Failed to read a directory entry")?;
        println!("{}", entry.file_name().to_string_lossy());
    }

    Ok(())
}
