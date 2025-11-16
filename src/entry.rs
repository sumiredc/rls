use anyhow::{Context, Result};
use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct PathEntry {
    path: PathBuf,
    filename: String,
}

impl PathEntry {
    fn new(path: PathBuf, filename: String) -> Self {
        Self { path, filename }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }
}

// Arugments
//
// * `include_cp` - Include list current(.) and parent(..)
pub fn get_all(path: &PathBuf, include_cp: bool) -> Result<Vec<PathEntry>> {
    let paths = if path.is_dir() {
        // ディレクトリ
        // 対象ディレクトリを読み込み、Vector を生成
        read_dir(path)?
    } else {
        // ファイル
        // 対象パスを Vector へ入れて返却
        vec![path.clone()]
    };

    let mut entries = filter_and_sort_paths(paths, false)?;

    if include_cp {
        // カレントディレクトリを追加
        entries.insert(0, PathEntry::new(path.clone(), String::from(".")));

        // 親ディレクトリを追加
        let mut parent_dir = path.clone();
        parent_dir.push("..");
        entries.insert(1, PathEntry::new(parent_dir, String::from("..")));
    }

    Ok(entries)
}

pub fn get_exclude_hidden(path: &PathBuf) -> Result<Vec<PathEntry>> {
    let paths = read_dir(path)?;

    filter_and_sort_paths(paths, true)
}

// 対象ディレクトリの一覧取得
fn read_dir(path: &PathBuf) -> Result<Vec<PathBuf>> {
    // 対象ディレクトリを読み込み、Vector を生成
    let rd =
        fs::read_dir(path).context(format!("Failed to read directory '{}'", path.display()))?;
    let paths_result: Result<Vec<PathBuf>> = rd
        .map(|result_entry| {
            let res = result_entry.map(|e| e.path())?;
            // std::io::Result から anyhow::Result へ変換
            Ok(res)
        })
        .collect();

    return paths_result;
}

// ファイル名の一覧取得
fn filter_and_sort_paths(paths: Vec<PathBuf>, is_skip_hidden: bool) -> Result<Vec<PathEntry>> {
    let mut entries: Vec<PathEntry> = vec![];
    for p in paths {
        let filename = match p.file_name() {
            Some(n) => n.to_string_lossy().to_string(),
            None => String::new(),
        };

        // 隠しファイルの判定
        if is_skip_hidden && filename.starts_with(".") {
            continue;
        }

        entries.push(PathEntry::new(p, filename));
    }

    // アルファベット順ソート
    entries.sort_unstable_by(|a, b| a.filename.cmp(&b.filename));

    Ok(entries)
}
