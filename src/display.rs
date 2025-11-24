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
    filetype: char,     // ファイルタイプ (d, l, -)
    permission: String, // パーミッション (rwxrwxrwx)
    xattr: char,        // 拡張属性
    nlink: u64,         // リンク数
    filesize: u64,      // ファイルサイズ
    modefied: String,   // 更新日時
    filename: String,   // ファイル名
}

// long list の描画
pub fn print_long_list(
    entries: &Vec<PathEntry>,
    mut writer: impl std::io::Write,
) -> Result<(), DisplayError> {
    let mut nlink_len = 0;
    let mut filesize_len = 0;
    let mut block_total = 0;

    let print_details: Vec<Result<PrintDetail, DisplayError>> = entries
        .iter()
        .map(|entry| -> Result<PrintDetail, DisplayError> {
            let meta = metadata(&entry.path())
                .map_err(|err| DisplayError::Metadata(entry.path().display().to_string(), err))?;

            let print_detail = PrintDetail {
                filetype: get_filetype(&meta),
                permission: get_permission(&meta),
                xattr: get_xattr(entry),
                nlink: meta.nlink(),
                filesize: meta.len(),
                filename: entry.filename(),
                modefied: get_modefied(&meta, entry)?,
            };

            // 出力長を更新
            nlink_len = max(nlink_len, print_detail.nlink.to_string().len());
            filesize_len = max(filesize_len, print_detail.filesize.to_string().len());

            // 合計ブロック数の計算
            block_total += meta.blocks();

            Ok(print_detail)
        })
        .collect();

    // 長さの微調整
    filesize_len += 1;

    // ブロック合計数の出力
    let _ = writeln!(writer, "total {}", block_total);

    for res in print_details {
        let print_detail = res?;

        let filetype = &print_detail.filetype;
        let permission = &print_detail.permission;
        let xattr = &print_detail.xattr;
        let nlink = format!("{:>width$}", print_detail.nlink, width = nlink_len);
        let filesize = format!("{:>width$}", print_detail.filesize, width = filesize_len);
        let modefied = &print_detail.modefied;
        let filename = &print_detail.filename;

        let _ = writeln!(
            writer,
            "{filetype}{permission}{xattr} {nlink} {filesize} {modefied} {filename}",
        );
    }

    Ok(())
}

fn get_filetype(meta: &Metadata) -> char {
    const DIRECTORY: char = 'd';
    const SYMLINK: char = 'l';
    const OTHER_FILE: char = '-';

    let filetype = meta.file_type();
    let symbol = if filetype.is_dir() {
        DIRECTORY
    } else if filetype.is_symlink() {
        SYMLINK
    } else {
        OTHER_FILE
    };

    symbol
}

// パーミッション情報の取得
fn get_permission(meta: &Metadata) -> String {
    const READ: char = 'r';
    const WRITE: char = 'w';
    const EXECUTE: char = 'x';
    const NONE: char = '-';

    const USER_READ: u32 = 0o400; // 256
    const USER_WRITE: u32 = 0o200; // 128
    const USER_EXECUTE: u32 = 0o100; // 64
    const GROUP_READ: u32 = 0o040; // 32
    const GROUP_WRITE: u32 = 0o020; // 16
    const GROUP_EXECUTE: u32 = 0o010; // 8
    const OTHER_READ: u32 = 0o004; // 4
    const OTHER_WRITE: u32 = 0o002; // 2
    const OTHER_EXECUTE: u32 = 0o001; //1

    let mode = meta.mode();
    let mut permission = String::new();

    for (p, s) in [
        (USER_READ, READ),
        (USER_WRITE, WRITE),
        (USER_EXECUTE, EXECUTE),
        (GROUP_READ, READ),
        (GROUP_WRITE, WRITE),
        (GROUP_EXECUTE, EXECUTE),
        (OTHER_READ, READ),
        (OTHER_WRITE, WRITE),
        (OTHER_EXECUTE, EXECUTE),
    ] {
        let symbol = if mode & p != 0 { s } else { NONE };
        permission.push(symbol);
    }

    permission
}

fn get_xattr(entry: &PathEntry) -> char {
    match xattr::list(entry.path()) {
        Ok(attrs) => {
            if attrs.count() > 0 {
                '@'
            } else {
                ' '
            }
        }
        Err(_) => ' ',
    }
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
    // ターミナル幅が取得できなかった場合のデフォルト値
    const DEFAULT_TERMINAL_SIZE: usize = 80;

    if let Some((Width(w), _)) = terminal_size() {
        w as usize // ターミナルの幅
    } else {
        DEFAULT_TERMINAL_SIZE
    }
}
