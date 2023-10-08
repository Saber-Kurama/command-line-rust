use assert_cmd::Command;
use predicates::{prelude::predicate, Predicate};

type TestResult = Result<(), Box<dyn std::error::Error>>;
#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echorn")?;
    let predicate_fn = predicate::str::is_empty();
    assert_eq!(true, predicate_fn.eval(""));
    cmd.assert()
        .failure()
        .stderr(predicate::str::is_empty());
    Ok(())
}
