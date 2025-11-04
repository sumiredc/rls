// use clap::Parser;

// #[derive(Parser)]
// struct Cli {
//     pattern: String,
//     path: std::path::PathBuf,
// }

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    // let args = Cli::parse();
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }

    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;
    println!("file content: {}", content);
    Ok(())
}
