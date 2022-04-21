mod common;
use std::path::PathBuf;
use std::process::Output;
use std::str;

#[test]
fn current_dir_is_dir() {
    assert!(std::env::current_dir().unwrap().is_dir());
}

// TODO
// are there platforms that will return non-absolute paths
// from current_dir() ? verify if absolute paths (on unix)
// can contain symlinks, hardlinks, etc
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
mod self_tests {
    // the following tests are temporary stand-ins for
    // checkmate self-evaluation. As a security tool the
    // integrity of the package should be confirmed.

    // the next test may not be compatible with git worktrees
    // or similar but may be useful for CI etc.
    /*#[test]
    fn current_dir_is_checkmate() {
        assert!(std::env::current_dir()
            .unwrap()
            .ends_with("cargo-checkmate"));
    }*/

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
    #[test]
    fn cargo_pkg_name_is_checkmate() {
        assert_eq!(super::common::get_pkg_name(), "cargo-checkmate");
    }

    #[test]
    fn attempt_generic_output() {
        assert!(super::common::attempt_output().status.success())
    }
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

// test env::args() panics with non UTF8
// env::args_os() function can handle non UTF8 arguments

// in main.rs : opts.execute() is called, look for tests here?
