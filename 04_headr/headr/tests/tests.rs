use std::error::Error;

use assert_cmd::Command;
use rand::{distributions::Alphanumeric, Rng};

const PRG: &str = "headr";
const EMPTY: &str = "./tests/inputs/empty.txt";

type TestResult = Result<(), Box<dyn Error>>;

fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

#[test]
fn dies_bad_bytes() -> TestResult {
    let bad = random_string();
    let expected = format!("illegal byte count -- {}", &bad);
    Command::cargo_bin(PRG)?
        .args(&["-c", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicates::str::contains(expected));
    Ok(())
}

#[test]
fn dies_bad_lines() -> TestResult {
    let bad = random_string();
    let expected = format!("illegal line count -- {}", &bad);
    Command::cargo_bin(PRG)?
        .args(&["-n", &bad, EMPTY])
        .assert()
        .failure()
        .stderr(predicates::str::contains(expected));
    Ok(())
}
