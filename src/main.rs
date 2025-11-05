use clap::Parser;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Parser)]
struct Cli {
    path: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let path = match args.path {
        Some(p) => p,
        None => PathBuf::new().join("."),
    };

    let entries = fs::read_dir(path)?;

    for entry in entries {
        println!("{}", entry?.file_name().to_string_lossy());
    }

    Ok(())
}
