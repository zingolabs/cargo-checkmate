mod common;
use std::path::PathBuf;
use std::process::Output;
use std::str;

#[test]
fn current_dir_is_ok() {
    assert!(std::env::current_dir().is_ok());
}

#[test]
fn current_dir_is_dir() {
    assert!(std::env::current_dir().unwrap().is_dir());
}

#[test]
fn current_dir_exists() {
    assert!(std::env::current_dir().unwrap().exists());
}

#[test]
#[should_panic]
fn current_dir_is_not_the_footgun() {
    assert_eq!(
        std::env::current_dir().unwrap(),
        PathBuf::from("/the/footgun")
    );
}

#[test]
fn current_dir_is_absolute() {
    assert!(std::env::current_dir().unwrap().is_absolute())
}

#[test]
fn current_dir_canonical_is_ok() {
    assert!(std::env::current_dir().unwrap().canonicalize().is_ok())
}

#[test]
fn current_dir_is_canonical() {
    assert_eq!(
        std::env::current_dir().unwrap(),
        std::env::current_dir().unwrap().canonicalize().unwrap()
    )
}

#[test]
fn current_dir_is_checkmate() {
    assert!(std::env::current_dir()
        .unwrap()
        .ends_with("cargo-checkmate"));
}

//should work across various OS paths
#[test]
fn src_main_exists() {
    let mut path = std::env::current_dir().unwrap();
    path.push("src");
    path.push("main.rs");
    assert!(path.exists());
}
#[test]
fn src_cdcrate_exists() {
    let mut path = std::env::current_dir().unwrap();
    path.push("src");
    path.push("cdcrate.rs");
    assert!(path.exists());
}
#[test]
fn src_phase_exists() {
    let mut path = std::env::current_dir().unwrap();
    path.push("src");
    path.push("phase.rs");
    assert!(path.exists());
}
#[test]
fn src_runner_exists() {
    let mut path = std::env::current_dir().unwrap();
    path.push("src");
    path.push("runner.rs");
    assert!(path.exists());
}
#[test]
fn src_subcommands_exists() {
    let mut path = std::env::current_dir().unwrap();
    path.push("src");
    path.push("subcommands.rs");
    assert!(path.exists());
}
#[test]
fn src_phaseresult_exists() {
    let mut path = std::env::current_dir().unwrap();
    path.push("src");
    path.push("runner");
    path.push("phaseresult.rs");
    assert!(path.exists());
}

// ... the above could go on.

#[test]
fn cargo_pkg_name_is_checkmate() {
    assert_eq!(common::get_pkg_name(), "cargo-checkmate");
}

#[test]
fn attempt_generic_output() {
    assert!(common::attempt_output().status.success())
}

#[test]
#[should_panic]
fn checkmate_is_cargo_installed() {
    let err = common::checkmate_output().stderr;
    assert_eq!(
        str::from_utf8(&err).unwrap(),
        "error: no such subcommand: `checkmate`\n"
    );
}

#[test]
fn checkmate_output_status_successful() {
    assert!(common::checkmate_output().status.success())
}

// TODO work more with out
#[test]
#[should_panic]
fn checkmate_is_right_out() {
    use std::str;
    let out = common::checkmate_output().stdout;
    assert!(str::from_utf8(&out)
        .unwrap()
        .contains("cargo-checkmate result: FAILED."));
}
