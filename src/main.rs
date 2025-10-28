use std::io::stdin;

use wcrs::count_lines;

fn main() {
    let lines = count_lines(stdin().lock());
    println!("{lines} lines");
}
