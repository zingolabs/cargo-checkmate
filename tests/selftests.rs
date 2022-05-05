mod common;
#[cfg(test)]
mod selftests {
    use std::str;

    #[test]
    fn checkmate_help_output_contains_client_name_and_description() {
        let s_out = super::common::assemble_command("--help").stdout;
        let outstr = str::from_utf8(&s_out).unwrap();
        assert!(outstr.contains("cargo-checkmate"));
        assert!(outstr.contains(
        "checkmate checks all the things - comprehensive out-of-the-box safety & hygiene checks."
    ));
    }

    #[test]
    fn checkmate_hook_output_contains_client_name_and_description() {
        let s_out = super::common::assemble_command("hook").stdout;
        let outstr = str::from_utf8(&s_out).unwrap();
        assert!(outstr.contains("cargo-checkmate-hook"));
        assert!(outstr.contains("manage repository hooks"));
        assert!(outstr.contains("cargo-checkmate hook <SUBCOMMAND>"));
    }

    #[test]
    fn checkmate_format_output_status_successful() {
        assert!(super::common::check_status_command("format")
            .status
            .success())
    }

    #[test]
    fn checkmate_check_output_status_successful() {
        assert!(super::common::check_status_command("check")
            .status
            .success())
    }

    #[test]
    fn checkmate_clippy_output_status_successful() {
        assert!(super::common::check_status_command("clippy")
            .status
            .success())
    }

    #[test]
    fn checkmate_doc_output_status_successful() {
        assert!(super::common::check_status_command("doc").status.success())
    }

    #[test]
    fn checkmate_build_output_status_successful() {
        assert!(super::common::check_status_command("build")
            .status
            .success())
    }

    #[test]
    fn checkmate_foo_output_status_fails() {
        assert_eq!(
            false,
            super::common::check_status_command("foo").status.success()
        )
    }
}
