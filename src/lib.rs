use std::io::{BufRead, Result};

pub fn count_lines(input: impl BufRead) -> Result<usize> {
    let mut count = 0;
    for line in input.lines() {
        line?;
        count += 1;
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor, Error, Read, Result};

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
}
