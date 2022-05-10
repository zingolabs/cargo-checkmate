use std::process::Command;
use std::process::Output;

pub fn checkmate_command(argument: &str) -> Output {
    assert!(
        "" != argument,
        "Don't pass an empty string to this function."
    );
    Command::new("./target/debug/cargo-checkmate")
        .arg(argument)
        .output()
        .expect("command with args failed")
}

#[should_panic]
#[test]
fn empty_checkmate_command() {
    checkmate_command("");
}
