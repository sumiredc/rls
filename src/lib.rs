mod cli;
mod display;
mod entry;

use anyhow::Result;
use std::path::PathBuf;

pub fn run() -> Result<()> {
    let args = cli::parse();

    // パスの決定
    let path = match args.path {
        Some(p) => p,
        None => PathBuf::new().join("."),
    };

    // 対象ディレクトリの一覧取得
    let entries = if args.all {
        entry::get_all(&path)?
    } else {
        entry::get_exclude_hidden(&path)?
    };

    // グリッドの表示
    display::print_grid(entries, &mut std::io::stdout())?;

    Ok(())
}
