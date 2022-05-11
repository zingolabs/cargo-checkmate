use std::process::Command;
use std::process::Output;

pub fn checkmate_command(argument: &str) -> Output {
    assert_eq!(false, argument.trim().is_empty());
    Command::new("./target/debug/cargo-checkmate")
        .arg(argument)
        .output()
        .expect("command with args failed")
}

#[should_panic]
#[test]
fn empty_checkmate_argument() {
    checkmate_command("");
}

#[should_panic]
#[test]
fn whitespace_checkmate_argument() {
    checkmate_command(" ");
}

#[should_panic]
#[test]
fn more_whitespace_checkmate_argument() {
    use std::str;
    let whitespace = vec![0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x20, 0xC2, 0x85, 0xC2, 0xA0];
    checkmate_command(str::from_utf8(&whitespace).unwrap());
}
