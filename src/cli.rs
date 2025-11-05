use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    pub path: Option<PathBuf>,
}

pub fn parse() -> Cli {
    Cli::parse()
}
