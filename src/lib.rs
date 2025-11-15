use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

#[derive(Default)]
pub struct Count {
    pub lines: usize,
    pub words: usize,
}

pub fn count(mut input: impl BufRead) -> Result<Count> {
    let mut count = Count::default();
    let mut line = String::new();
    while input.read_line(&mut line)? > 0 {
        count.lines += 1;
        count.words += line.split_whitespace().count();
        line.clear();
    }
    Ok(count)
}

pub fn count_lines_in_path(path: &String) -> Result<Count> {
    let file = File::open(path).with_context(|| path.clone())?;
    let buf = BufReader::new(file);
    count(buf).with_context(|| path.clone())
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Error, Read, Result, Write};

    use super::*;

    struct ErrorReader;

    impl Read for ErrorReader {
        fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
            Err(Error::other("oh no!"))
        }
    }

    #[test]
    fn count_lines_returns_number_of_lines_in_input() {
        let input = Cursor::new("line 1\nline 2\n");
        let counter = count(input).unwrap();

        assert_eq!(counter.lines, 2, "wrong line count");
    }

    #[test]
    fn count_lines_returns_error_if_present() {
        let reader = BufReader::new(ErrorReader);
        let counter = count(reader);

        assert!(counter.is_err(), "no error returned");
    }

    #[test]
    fn count_lines_in_path_returns_number_of_lines_in_a_file() {
        let p = String::from("foo.txt");
        let mut f = File::create(&p).unwrap();
        _ = f.write(b"1\n2\n3\n4").unwrap();
        let counter = count_lines_in_path(&p).unwrap();

        assert_eq!(counter.lines, 4, "wrong lines count");
    }

    #[test]
    fn count_words_returns_number_of_words_in_input() {
        let input = Cursor::new("word1 word2 word3\nword4 word5\n");
        let counter = count(input).unwrap();

        assert_eq!(counter.words, 5, "wrong word count");
    }
}
