use std::env;

use anyhow::{Result, bail};
use wcrs::count_lines_in_path;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        bail!("Usage: wcrs <FILE>...");
    }
    for path in args {
        let counter = count_lines_in_path(&path)?;
        println!("{path}: {} lines", counter.lines);
        println!("{path}: {} words", counter.words);
    }
    Ok(())
}
