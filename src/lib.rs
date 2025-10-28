use std::io::{BufRead, Cursor};

pub fn count_lines(input: impl BufRead) -> usize {
    input.lines().count()
}

#[test]
fn count_lines_returns_number_of_lines_in_input() {
    let input = Cursor::new("line 1\nline 2\n");
    let lines = count_lines(input);

    assert_eq!(lines, 2, "wrong line count");
}
