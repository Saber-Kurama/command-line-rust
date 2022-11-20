// use std::process::Command;

// #[test]
// fn runs() {
//     let mut cmd = Command::new("ls");
//     let res = cmd.output();
//     assert!(res.is_ok());
// }

use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}
