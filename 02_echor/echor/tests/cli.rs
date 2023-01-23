use std::fs;

use assert_cmd::Command;
use predicates::prelude::predicate;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn works() {
    assert!(true)
}

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

// let mut cmd = Command::cargo_bin("echor").unwrap();
// cmd.arg("hello").assert().success();
#[test]
fn hello1() -> TestResult {
    // let outfile = "tests/expected/hello1.txt";
    // let expected = fs::read_to_string(outfile).unwrap();
    // let mut cmd = Command::cargo_bin("echor").unwrap();
    // cmd.arg("Hello there").assert().success().stdout(expected);
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    // let outfile = "tests/expected/hello2.txt";
    // let expected = fs::read_to_string(outfile)?;
    // let mut cmd = Command::cargo_bin("echor")?;
    // cmd.args(vec!["Hello", "there"])
    //     .assert()
    //     .success()
    //     .stdout(expected);
    // Ok(())
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["Hello", "there", "-n"], "tests/expected/hello2.n.txt")
}
