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
    let whitespace = vec![
        9, 10, 11, 12, 13, 32, 133, 160, 0xE2, 0x80, 0x8E, 0xE2, 0x80, 0x8F, 0xE2, 0x80, 0xA8,
        0xE2, 0x80, 0xA9,
    ];
    checkmate_command(str::from_utf8(&whitespace).unwrap());
}
