use std::io::stdin;

use wcrs::count_lines;

fn main() {
    let lines = count_lines(stdin().lock()).expect("found an error");
    println!("{lines} lines");
}
