use assert_cmd::Command;
use std::process::Command as SCommand;

#[test]
fn test1() {
    assert_eq!(1 + 2, 3);
}

#[test]
fn test_cli() {
    let mut cmd = SCommand::new("ls");
    let res = cmd.output();
    assert!(res.is_ok())
}

#[test]
fn test_cli2() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}
