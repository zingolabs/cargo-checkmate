use std::process::Command;
use std::process::Output;

pub fn checkmate_command(argument: &str) -> Output {
    Command::new("./target/debug/cargo-checkmate")
        .arg(argument)
        .output()
        .expect("command with args failed")
}
