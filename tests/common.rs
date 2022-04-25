use std::process::Command;
use std::process::Output;

pub fn assemble_command(argument: &str) -> Output {
    Command::new("sh")
        .arg("-c")
        .arg(argument)
        .output()
        .expect("failed to execute")
}

pub fn attempt_output() -> Output {
    let arg = "echo testing";
    assemble_command(arg)
}

pub fn checkmate_help_output() -> Output {
    let arg = "./target/debug/cargo-checkmate --help";
    assemble_command(arg)
}

pub fn checkmate_output() -> Output {
    let arg = "./target/debug/cargo-checkmate";
    assemble_command(arg)
}
