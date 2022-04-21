use std::process::Command;
use std::process::Output;

pub fn attempt_output() -> Output {
    if cfg!(target_os = "windows") {
        return Command::new("cmd")
            .args(["/C", "echo windows-test"])
            .output()
            .expect("failed to exectute, windows OS");
    } else {
        return Command::new("sh")
            .arg("-c")
            .arg("echo testing")
            .output()
            .expect("failed to execute");
    };
}

pub fn checkmate_output() -> Output {
    if cfg!(target_os = "windows") {
        return Command::new("cmd")
            .args(["/C", "cargo checkmate"])
            .output()
            .expect("failed to exectute, windows OS");
    } else {
        return Command::new("sh")
            .arg("-c")
            .arg("cargo checkmate")
            .output()
            .expect("failed to execute");
    };
}

pub fn get_pkg_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
