use std::path::PathBuf;
use std::process::Command;
use std::process::Output;

pub fn attempt_output() -> Output {
    // TODO: windows workflow untested
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo windows-test"])
            .output()
            .expect("failed to exectute, windows OS")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo testing")
            .output()
            .expect("failed to execute")
    }
}

pub fn checkmate_help_output() -> Output {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg("./target/debug/cargo-checkmate --help")
            .output()
            .expect("failed to exectute, windows OS")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("./target/debug/cargo-checkmate --help")
            .output()
            .expect("failed to execute")
    }
}

pub fn checkmate_output() -> Output {
    let exec: PathBuf = PathBuf::from("target/debug/cargo-checkmate");
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(exec)
            .output()
            .expect("failed to exectute, windows OS")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(exec)
            .output()
            .expect("failed to execute")
    }
}

pub fn get_pkg_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
