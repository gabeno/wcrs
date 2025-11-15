use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

pub fn count_lines(input: impl BufRead) -> Result<usize> {
    // iterator version
    // input
    //     .lines()
    //     .try_fold(0, |count, line| { line.map(|_| count + 1)})

    let mut count = 0;
    for line in input.lines() {
        line?;
        count += 1;
    }
    Ok(count)
}

pub fn count_lines_in_path(path: &String) -> Result<usize> {
    let file = File::open(path).with_context(|| path.clone())?;
    let buf = BufReader::new(file);
    count_lines(buf).with_context(|| path.clone())
}

pub fn count_words(input: impl BufRead) -> Result<usize> {
    let mut count = 0;
    for line in input.lines() {
        count += line?.split_whitespace().count();
    }
    Ok(count)
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
        let lines = count_lines(input).unwrap();

        assert_eq!(lines, 2, "wrong line count");
    }

    #[test]
    fn count_lines_returns_error_if_present() {
        let reader = BufReader::new(ErrorReader);
        let result = count_lines(reader);

        assert!(result.is_err(), "no error returned");
    }

    #[test]
    fn count_lines_in_path_returns_number_of_lines_in_a_file() {
        let p = String::from("foo.txt");
        let mut f = File::create(&p).unwrap();
        _ = f.write(b"1\n2\n3\n4").unwrap();
        let lines = count_lines_in_path(&p).unwrap();

        assert_eq!(lines, 4, "wrong lines count");
    }

    #[test]
    fn count_words_returns_number_of_words_in_input() {
        let input = Cursor::new("word1 word2 word3");
        let word_count = count_words(input).unwrap();

        assert_eq!(word_count, 3, "wrong word count");
    }
}
