use crate::check::Check;
use crate::CMDNAME;
use std::io::Result;
use std::path::PathBuf;

mod phaseresult;
use phaseresult::PhaseResult;

pub struct Runner {
    logdir: PathBuf,
    passes: Vec<PhaseResult>,
    fails: Vec<PhaseResult>,
}

impl Runner {
    pub fn new() -> Result<Runner> {
        let logdir = crate::results_dir().join("logs");

        {
            if logdir.exists() {
                println!("Removing prior log directory: {}", &logdir.display());
                std::fs::remove_dir_all(&logdir)?;
            }
            std::fs::create_dir_all(&logdir)?;
        }

        Ok(Runner {
            logdir: logdir,
            passes: vec![],
            fails: vec![],
        })
    }

    pub fn run_check(&mut self, check: &Check) -> Result<()> {
        use std::process::Command;

        let checkname = &check.to_string();
        let exec = std::env::current_exe()?;

        print!("{} {}... ", CMDNAME, checkname);

        {
            use std::io::Write;
            std::io::stdout().flush()?;
        }

        let output = Command::new(exec).arg(checkname).output()?;

        let reloutlog = self.rellog_path(checkname, "stdout");
        let relerrlog = self.rellog_path(checkname, "stderr");

        {
            use std::fs::File;
            use std::io::Write;

            File::create(&relerrlog)?.write_all(&output.stderr)?;
            File::create(&reloutlog)?.write_all(&output.stdout)?;
        }

        let results = if output.status.success() {
            if output.stdout.starts_with(b"skipped:\n") {
                println!("skipped.");
            } else {
                println!("ok.");
            }
            &mut self.passes
        } else {
            println!("FAILED.");
            &mut self.fails
        };

        results.push(PhaseResult::new(checkname, reloutlog, relerrlog));

        Ok(())
    }

    pub fn exit(self) -> Result<()> {
        let passcount = self.passes.len();
        let failcount = self.fails.len();

        let (exitstatus, label) = if failcount == 0 {
            (0, "ok")
        } else {
            println!("\nfailures:\n");

            for fres in self.fails {
                fres.display()?;
            }

            (1, "FAILED")
        };

        println!(
            "\n{} result: {}. {} passed; {} failed",
            CMDNAME, label, passcount, failcount,
        );

        std::process::exit(exitstatus);
    }

    fn rellog_path(&self, checkname: &str, outkind: &str) -> PathBuf {
        self.logdir.join(&format!("{}.{}", checkname, outkind))
    }
}
