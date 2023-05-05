use assert_cmd::Command;
use predicates::prelude::predicate;

type TestResult = Result<(), Box<dyn std::error::Error>>;
#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echorn")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("you"));
    Ok(())
}
