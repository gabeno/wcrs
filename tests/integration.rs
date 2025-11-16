use assert_cmd::{Command, cargo};

use predicates::prelude::predicate;

#[test]
fn binary_with_no_args_prints_usage() {
    Command::new(cargo::cargo_bin!("wcrs"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn binary_counts_lines_in_named_files() {
    Command::new(cargo::cargo_bin!("wcrs"))
        .args(&["tests/data/lines.txt", "tests/data/words.txt"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "tests/data/lines.txt: 2 lines\ntests/data/words.txt: 2 lines",
        ));
}

#[test]
fn binary_counts_words_with_flag() {
    Command::new(cargo::cargo_bin!("wcrs"))
        .args(&["-w", "tests/data/lines.txt", "tests/data/words.txt"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "tests/data/lines.txt: 2 words\ntests/data/words.txt: 14 words",
        ));
}
