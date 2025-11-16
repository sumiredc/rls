mod cli;
mod display;
mod entry;

use anyhow::{Result, bail};
use std::env;

pub fn run() -> Result<()> {
    let args = cli::parse();

    let current_dir = match env::current_dir() {
        Ok(d) => d,
        Err(err) => bail!("{}", err),
    };

    // 検索するパスの決定
    let path = match args.path {
        Some(p) => current_dir.join(p),
        None => current_dir,
    };

    // 対象ディレクトリの一覧取得
    let entries = if args.all || args.almost_all {
        entry::get_all(&path, args.all)?
    } else {
        entry::get_exclude_hidden(&path)?
    };

    let mut w = std::io::stdout();

    // ロングリストモードで表示
    if args.l {
        display::print_long_list(&entries, &mut w)?;
    } else {
        // グリッドの表示
        display::print_grid(&entries, &mut w)?;
    }

    Ok(())
}
