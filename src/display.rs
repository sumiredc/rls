use std::fs::metadata;

use anyhow::{Result, bail};
use chrono::{DateTime, Local};
use std::os::unix::fs::MetadataExt;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::{Width, terminal_size};

use crate::entry::PathEntry;

// ターミナル幅が取得できなかった場合のデフォルト値
const DEFAULT_TERMINAL_SIZE: usize = 80;

// grid の描画
pub fn print_grid(entries: &Vec<PathEntry>, mut writer: impl std::io::Write) -> Result<()> {
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

// long list の描画
pub fn print_long_list(entries: &Vec<PathEntry>, mut writer: impl std::io::Write) -> Result<()> {
    for entry in entries {
        // metadata の取得
        let meta = match metadata(&entry.path()) {
            Ok(m) => m,
            Err(err) => bail!("Can't get metadata \"{}\": {}", entry.path().display(), err),
        };

        // リンク数の取得
        let nlink = meta.nlink();

        // サイズの取得
        let file_size = meta.len();

        // 最終更新日時の取得
        let modefied_datetime: DateTime<Local> = meta.modified()?.into();
        let formatted_modefiled = modefied_datetime.format("%b %e %H:%M").to_string();

        // ファイル名の取得
        let filename = entry.filename();

        let _ = writeln!(
            writer,
            "{nlink} {file_size} {formatted_modefiled} {filename}",
        );
    }

    Ok(())
}

// ターミナルのサイズ取得
fn get_terminal_size() -> usize {
    if let Some((Width(w), _)) = terminal_size() {
        w as usize // ターミナルの幅
    } else {
        DEFAULT_TERMINAL_SIZE
    }
}
