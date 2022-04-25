mod common;
use std::str;

#[test]
fn attempt_generic_output() {
    assert!(common::attempt_output().status.success())
}

#[test]
fn checkmate_help_output_contains_client_name_and_description() {
    let out = common::checkmate_help_output().stdout;
    assert!(str::from_utf8(&out).unwrap().contains("cargo-checkmate"));
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
