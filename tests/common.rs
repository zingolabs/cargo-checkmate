use std::process::Command;
use std::process::Output;

pub fn clean_build_checkmate() {
    Command::new("cargo")
        .arg("clean")
        .status()
        .expect("cargo clean failed");
    Command::new("cargo")
        .arg("build")
        .arg("-q")
        .status()
        .expect("cargo build failed");
}

pub fn check_status_command(argument: Option<&str>) -> Output {
    match argument {
        Some(arg) => Command::new("./target/debug/cargo-checkmate ")
            .arg(arg)
            .output()
            .expect("command status with args failed"),
        None => Command::new("./target/debug/cargo-checkmate")
            .output()
            .expect("command status with no args failed"),
    }
}

pub fn assemble_command(argument: Option<&str>) -> Output {
    match argument {
        Some(arg) => {
            let mut exe: String = String::from("./target/debug/cargo-checkmate ");
            exe.push_str(arg);
            Command::new("script")
                .arg("-c")
                .arg(&exe)
                .output()
                .expect("failed to execute")
        }
        None => Command::new("script")
            .arg("-c")
            .arg("./target/debug/cargo-checkmate")
            .output()
            .expect("failed to execute"),
    }
}
