use assert_cmd::Command;

#[test]
fn works() {
    assert!(true)
}

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert().failure()
}
