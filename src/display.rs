use std::{fs::metadata, time::UNIX_EPOCH};

use anyhow::{Result, bail};
use chrono::DateTime;
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

        // 最終更新日時の取得
        let modified = match meta.modified() {
            Ok(m) => {
                let duration = match m.duration_since(UNIX_EPOCH) {
                    Ok(d) => d.as_secs(),
                    Err(err) => bail!(
                        "Can't get duration in modified \"{}\": {}",
                        entry.path().display(),
                        err
                    ),
                };

                match DateTime::from_timestamp_secs(duration as i64) {
                    Some(dt) => dt.to_string(),
                    None => String::new(),
                }
            }
            Err(err) => bail!(
                "Can't get modified in metadata \"{}\": {}",
                entry.path().display(),
                err
            ),
        };

        let _ = writeln!(writer, "{} {}", modified, entry.filename());
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
