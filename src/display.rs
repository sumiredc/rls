use anyhow::Result;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use terminal_size::{Width, terminal_size};

// ターミナル幅が取得できなかった場合のデフォルト値
const DEFAULT_TERMINAL_SIZE: usize = 80;

// grid の描画
pub fn print_grid(items: Vec<String>, mut writer: impl std::io::Write) -> Result<()> {
    // ターミナルの幅を取得
    let terminal_width = get_terminal_size();

    // グリッド準備
    let mut grid = Grid::new(GridOptions {
        filling: Filling::Spaces(8), // 列間のスペース
        direction: Direction::TopToBottom,
    });

    // グリッドにセルを追加
    for item in items {
        grid.add(Cell::from(item));
    }

    // グリッドの計算 & 表示
    if let Some(grid_display) = grid.fit_into_width(terminal_width) {
        let _ = write!(writer, "{}", grid_display);
    } else {
        let _ = write!(writer, "{:?}", grid);
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
