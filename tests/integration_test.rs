use assert_cmd::{Command, assert::OutputAssertExt};

#[test]
fn test_serialize_fucntion_with_one_argument() {
    let cmd = Command::cargo_bin("anixe_cli").unwrap().arg("input.csv").unwrap();
    
    cmd.assert().success();
}

#[test]
fn test_serialize_fucntion_with_two_argument() {
    let cmd = Command::cargo_bin("anixe_cli").unwrap().args(&["input.csv", "output.csv"]).unwrap();
    
    cmd.assert().success();
}

#[test]
#[should_panic]
fn test_serialize_fucntion_without_arguments() {
    let mut cmd = Command::cargo_bin("anixe_cli").unwrap();
    
    cmd.assert().success();
}

