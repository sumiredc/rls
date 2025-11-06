use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    pub path: Option<PathBuf>,

    #[arg(short, long, default_value_t = false)]
    pub all: bool,
}

pub fn parse() -> Cli {
    Cli::parse()
}
