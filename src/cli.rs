use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    pub path: Option<PathBuf>,

    #[arg(short, long, default_value_t = false)]
    pub all: bool,

    #[arg(short = 'A', long, default_value_t = false)]
    pub almost_all: bool,

    #[arg(short, long, default_value_t = false)]
    pub l: bool,
}

pub fn parse() -> Cli {
    Cli::parse()
}
