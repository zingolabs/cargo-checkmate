use std::process::Command;
use std::process::Output;

pub fn check_status_command(argument: &str) -> Output {
    Command::new("cargo-checkmate")
        .arg(argument)
        .output()
        .expect("command status with args failed")
}

pub fn assemble_command(argument: &str) -> Output {
    let mut exe: String = String::from("./target/debug/cargo-checkmate ");
    exe.push_str(argument);
    Command::new("script")
        .arg("-c")
        .arg(&exe)
        .output()
        .expect("failed to execute")
}
