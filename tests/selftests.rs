mod common;
#[cfg(test)]
mod selftests {
    use std::str;
    #[test]
    #[cfg_attr(not(feature = "selftest"), ignore)]
    fn checkmate_help_output_contains_client_name_and_description() {
        super::common::clean_build_checkmate();
        let s_out = super::common::assemble_command(Some("--help")).stdout;
        let outstr = str::from_utf8(&s_out).unwrap();
        assert!(outstr.contains("cargo-checkmate"));
        assert!(outstr.contains(
        "checkmate checks all the things - comprehensive out-of-the-box safety & hygiene checks."
    ));
    }

    #[test]
    #[cfg_attr(not(feature = "selftest"), ignore)]
    fn checkmate_output_status_successful() {
        super::common::clean_build_checkmate();
        assert!(super::common::check_status_command(None).status.success())
    }

    #[test]
    #[cfg_attr(not(feature = "selftest"), ignore)]
    fn checkmate_passes_checkmate() {
        super::common::clean_build_checkmate();
        let s_out = super::common::assemble_command(None).stdout;
        let outstr = str::from_utf8(&s_out).unwrap();
        assert_eq!(false, outstr.contains("cargo-checkmate result: FAILED."));
    }
}
