mod common;
use std::str;

#[test]
#[cfg_attr(not(feature = "selftest"), ignore)]
fn attempt_generic_output() {
    let out = common::attempt_output();
    assert!(out.status.success());
    let outstr = str::from_utf8(&out.stdout).unwrap();
    assert!(outstr.contains("testing"));
    assert!(outstr.contains("Script done."));
}

#[test]
#[cfg_attr(not(feature = "selftest"), ignore)]
fn checkmate_help_output_contains_client_name_and_description() {
    common::clean_build_checkmate();
    let s_out = common::checkmate_help_output().stdout;
    let outstr = str::from_utf8(&s_out).unwrap();
    assert!(outstr.contains("cargo-checkmate"));
    assert!(outstr.contains(
        "checkmate checks all the things - comprehensive out-of-the-box safety & hygiene checks."
    ));
}

#[test]
#[cfg_attr(not(feature = "selftest"), ignore)]
fn checkmate_output_status_successful() {
    common::clean_build_checkmate();
    assert!(common::checkmate_output().status.success())
}

#[test]
#[cfg_attr(not(feature = "selftest"), ignore)]
fn checkmate_passes_checkmate() {
    common::clean_build_checkmate();
    let s_out = common::checkmate_output().stdout;
    let outstr = str::from_utf8(&s_out).unwrap();
    assert_eq!(false, outstr.contains("cargo-checkmate result: FAILED."));
}
