use anyhow::{Context, Ok, Result};
use std::{
    fs::{self, ReadDir},
    path::PathBuf,
};

pub fn get_all(path: &PathBuf) -> Result<Vec<String>> {
    let entries = read_dir(path)?;
    let mut file_names = filter_file_names(entries, false)?;

    // カレントディレクトリ、上位ディレクトリへのパスを追加
    file_names.push(String::from("."));
    file_names.push(String::from(".."));

    // アルファベット順ソート
    file_names.sort_unstable();

    Ok(file_names)
}

pub fn get_exclude_hidden(path: &PathBuf) -> Result<Vec<String>> {
    let entries = read_dir(path)?;
    let mut file_names = filter_file_names(entries, true)?;

    // アルファベット順ソート
    file_names.sort_unstable();

    Ok(file_names)
}

// 対象ディレクトリの一覧取得
fn read_dir(path: &PathBuf) -> Result<ReadDir> {
    fs::read_dir(path).context(format!("Failed to read directory '{}'", path.display()))
}

// ファイル名の一覧取得
fn filter_file_names(entries: ReadDir, is_skip_hidden: bool) -> Result<Vec<String>> {
    let mut file_names: Vec<String> = Vec::new();
    for e in entries {
        let entry = e.context("Failed to read a directory entry")?;
        let file_name = entry.file_name().to_string_lossy().to_string();

        // 隠しファイルの判定
        if is_skip_hidden && file_name.starts_with(".") {
            continue;
        }

        file_names.push(file_name);
    }

    Ok(file_names)
}
