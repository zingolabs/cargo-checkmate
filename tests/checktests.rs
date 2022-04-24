mod common;
use std::str;

#[test]
fn current_dir_is_absolute() {
    assert!(std::env::current_dir().unwrap().is_absolute())
}

#[test]
fn cargo_pkg_name_is_checkmate() {
    assert_eq!(common::get_pkg_name(), "cargo-checkmate");
}

#[test]
fn attempt_generic_output() {
    assert!(common::attempt_output().status.success())
}

#[test]
fn checkmate_help_output_contains_client_name() {
    let out = common::checkmate_help_output().stdout;
    assert!(str::from_utf8(&out).unwrap().contains("cargo-checkmate"));
}

#[test]
fn checkmate_help_output_contains_description() {
    let out = common::checkmate_help_output().stdout;
    assert!(str::from_utf8(&out).unwrap().contains(
        "checkmate checks all the things - comprehensive out-of-the-box safety & hygiene checks."
    ));
}

#[test]
fn checkmate_output_status_successful() {
    assert!(common::checkmate_output().status.success())
}

#[test]
fn checkmate_passes_checkmate() {
    let out = common::checkmate_output().stdout;
    assert!(!str::from_utf8(&out)
        .unwrap()
        .contains("cargo-checkmate result: FAILED."));
}
