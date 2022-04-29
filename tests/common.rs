use std::process::Command;
use std::process::Output;

pub fn clean_build_checkmate() {
    Command::new("cargo")
        .arg("clean")
        .arg("-p")
        .arg("cargo-checkmate")
        .status()
        .expect("cargo clean failed");
    Command::new("cargo")
        .arg("build")
        .arg("-q")
        .status()
        .expect("cargo build failed");
}

pub fn attempt_output() -> Output {
    let cli_arg = "echo testing";
    assemble_command(cli_arg)
}
pub fn checkmate_help_output() -> Output {
    let cli_arg = "./target/debug/cargo-checkmate --help";
    assemble_command(cli_arg)
}
pub fn checkmate_output() -> Output {
    let cli_arg = "./target/debug/cargo-checkmate";
    assemble_command(cli_arg)
}

pub fn assemble_command(argument: &str) -> Output {
    Command::new("script")
        .arg("-O")
        .arg("selftest_stdout")
        .arg("-c")
        .arg(argument)
        .output()
        .expect("failed to execute")
}
