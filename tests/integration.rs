use assert_cmd::{Command, cargo};
use predicates::prelude::predicate;

#[test]
fn binary_with_no_args_prints_usage() {
    Command::new(cargo::cargo_bin!("wcrs"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}
