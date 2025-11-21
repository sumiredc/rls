use std::{
    cmp::max,
    fs::{Metadata, metadata},
};

use anyhow::Result;
use chrono::{DateTime, Local};
use std::os::unix::fs::MetadataExt;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::{Width, terminal_size};
use thiserror::Error;

use crate::entry::PathEntry;

// ターミナル幅が取得できなかった場合のデフォルト値
const DEFAULT_TERMINAL_SIZE: usize = 80;

#[derive(Error, Debug)]
pub enum DisplayError {
    #[error("Metadata Error: {0} {1}")]
    Metadata(String, std::io::Error),

    #[error("Modefied Error: {0} {1}")]
    Modefied(String, std::io::Error),
}

// grid の描画
pub fn print_grid(
    entries: &Vec<PathEntry>,
    mut writer: impl std::io::Write,
) -> Result<(), DisplayError> {
    // ターミナルの幅を取得
    let terminal_width = get_terminal_size();

    // グリッド準備
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(8), // 列間のスペース
        direction: Direction::TopToBottom,
    });

    // グリッドにセルを追加
    for entry in entries {
        grid.add(Cell::from(entry.filename()));
    }

    // グリッドの計算 & 表示
    if let Some(grid_display) = grid.fit_into_width(terminal_width) {
        let _ = write!(writer, "{}", grid_display);
    } else {
        let _ = write!(writer, "{:?}", grid);
    }

    Ok(())
}

struct PrintDetail {
    nlink: u64,       // リンク数
    filesize: u64,    // ファイルサイズ
    modefied: String, // 更新日時
    filename: String, // ファイル名
}

// long list の描画
pub fn print_long_list(
    entries: &Vec<PathEntry>,
    mut writer: impl std::io::Write,
) -> Result<(), DisplayError> {
    let mut nlink_len = 0;
    let mut filesize_len = 0;

    let print_details: Vec<Result<PrintDetail, DisplayError>> = entries
        .iter()
        .map(|entry| -> Result<PrintDetail, DisplayError> {
            let meta = metadata(&entry.path())
                .map_err(|err| DisplayError::Metadata(entry.path().display().to_string(), err))?;
            let print_detail = PrintDetail {
                nlink: meta.nlink(),
                filesize: meta.len(),
                filename: entry.filename(),
                modefied: get_modefied(&meta, entry)?,
            };

            // 出力長を更新
            nlink_len = max(nlink_len, print_detail.nlink.to_string().len());
            filesize_len = max(filesize_len, print_detail.filesize.to_string().len());

            Ok(print_detail)
        })
        .collect();

    // 長さの微調整
    filesize_len += 1;

    for res in print_details {
        let print_detail = res?;
        let nlink = format!("{:>width$}", print_detail.nlink, width = nlink_len);
        let filesize = format!("{:>width$}", print_detail.filesize, width = filesize_len);

        let _ = writeln!(
            writer,
            "{nlink} {filesize} {} {}",
            print_detail.modefied, print_detail.filename
        );
    }

    Ok(())
}

// 更新日時の取得
fn get_modefied(meta: &Metadata, entry: &PathEntry) -> Result<String, DisplayError> {
    let modefiled: DateTime<Local> = meta
        .modified()
        .map_err(|err| DisplayError::Modefied(entry.path().display().to_string(), err))?
        .into();

    Ok(modefiled.format("%b %e %H:%M").to_string())
}

// ターミナルのサイズ取得
fn get_terminal_size() -> usize {
    if let Some((Width(w), _)) = terminal_size() {
        w as usize // ターミナルの幅
    } else {
        DEFAULT_TERMINAL_SIZE
    }
}
