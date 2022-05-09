mod common;
#[cfg(test)]
mod selftests {
    use std::str;

    #[test]
    fn checkmate_dashdash_help_output_contains_client_name_and_description() {
        let s_out = super::common::checkmate_command("--help").stdout;
        let outstr = dbg!(str::from_utf8(&s_out).unwrap());
        assert!(outstr.contains("cargo-checkmate"));
        assert!(outstr.contains(
        "checkmate checks all the things - comprehensive out-of-the-box safety & hygiene checks."
    ));
    }

    #[test]
    fn checkmate_help_output_contains_client_name_and_description() {
        let s_out = super::common::checkmate_command("help").stdout;
        let outstr = dbg!(str::from_utf8(&s_out).unwrap());
        assert!(outstr.contains("cargo-checkmate"));
        assert!(outstr.contains(
        "checkmate checks all the things - comprehensive out-of-the-box safety & hygiene checks."
    ));
    }

    #[test]
    fn checkmate_format_status_successful() {
        assert!(super::common::checkmate_command("format").status.success())
    }

    #[test]
    fn checkmate_format_stdout_empty() {
        let s_out = super::common::checkmate_command("format").stdout;
        let outstr = str::from_utf8(&s_out).unwrap();
        assert!(outstr.is_empty());
    }

    #[test]
    fn checkmate_check_status_successful() {
        assert!(super::common::checkmate_command("check").status.success())
    }

    #[test]
    fn checkmate_clippy_status_successful() {
        assert!(super::common::checkmate_command("clippy").status.success())
    }

    #[test]
    fn checkmate_doc_status_successful() {
        assert!(super::common::checkmate_command("doc").status.success())
    }

    #[test]
    fn checkmate_build_status_successful() {
        assert!(super::common::checkmate_command("build").status.success())
    }

    #[test]
    fn checkmate_foo_status_fails() {
        assert_eq!(
            false,
            super::common::checkmate_command("foo").status.success()
        )
    }

    #[test]
    fn checkmate_foo_stderr_contains_specific_response() {
        let s_err = super::common::checkmate_command("foo").stderr;
        let err = str::from_utf8(&s_err).unwrap();
        assert_eq!(
            true,
            err.contains(
                "error: Found argument 'foo' which wasn't expected, or isn't valid in this context"
            )
        );
        assert_eq!(true, err.contains("USAGE:"));
        assert_eq!(true, err.contains("cargo-checkmate [SUBCOMMAND]"));
    }

    #[test]
    fn checkmate_hook_invocation_with_no_arguments_fails() {
        let out = super::common::checkmate_command("hook");
        // debug shows out.status = ExitStatus(unix_wait_status(256,),)
        assert_eq!(false, out.status.success());
    }

    // using `hook` the help output is printed to TTY with stderr
    #[test]
    fn checkmate_hook_invocation_with_no_arguments_stderr_contains_client_name_and_description() {
        let s_err = super::common::checkmate_command("hook").stderr;
        let outerr = str::from_utf8(&s_err).unwrap();
        assert!(outerr.contains("cargo-checkmate-hook"));
        assert!(outerr.contains("manage repository hooks"));
        assert!(outerr.contains("cargo-checkmate hook <SUBCOMMAND>"));
    }
}
