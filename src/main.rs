use clap::{Parser, arg};

use anyhow::Result;
use wcrs::count_in_path;

#[derive(Parser)]
/// Count lines or words in the specified files
struct Args {
    /// Count words instead of lines
    #[arg(short, long)]
    words: bool,

    /// Files to run count on
    #[arg(required = true)]
    files: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    for path in args.files {
        let counter = count_in_path(&path)?;

        if args.words {
            println!("{path}: {} words", counter.words);
        } else {
            println!("{path}: {} lines", counter.lines);
        }
    }
    Ok(())
}
