mod cli;
mod display;

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn run() -> Result<()> {
    let args = cli::parse();

    // パスの決定
    let path = match args.path {
        Some(p) => p,
        None => PathBuf::new().join("."),
    };

    // 対象ディレクトリの一覧取得
    let entries =
        fs::read_dir(&path).context(format!("Failed to read directory '{}'", path.display()))?;

    let mut file_names: Vec<String> = Vec::new();
    for e in entries {
        let entry = e.context("Failed to read a directory entry")?;
        let file_name = entry.file_name().to_string_lossy().to_string();

        file_names.push(file_name);
    }

    // アルファベット順ソート
    file_names.sort_unstable();

    // グリッドの表示
    display::print_grid(file_names, &mut std::io::stdout())?;

    Ok(())
}
