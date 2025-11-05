use std::{fs, io};

fn main() -> io::Result<()> {
    let entries = fs::read_dir(".")?;

    for entry in entries {
        println!("{}", entry?.file_name().to_string_lossy());
    }

    Ok(())
}
