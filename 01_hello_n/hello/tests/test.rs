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

#[test]
fn test_true() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}
#[test]
fn test_false() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

#[test]
fn test_output() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}
