use std::env;

use anyhow::{Result, bail};
use wcrs::count_lines_in_path;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    let mut show_word_count = false;
    if args.is_empty() {
        bail!("Usage: wcrs [-w] <FILE>...");
    }
    for path in args {
        if path == "-w" {
            show_word_count = true;
            continue;
        }
        let counter = count_lines_in_path(&path)?;

        if show_word_count {
            println!("{path}: {} words", counter.words);
        } else {
            println!("{path}: {} lines", counter.lines);
        }
    }
    Ok(())
}
