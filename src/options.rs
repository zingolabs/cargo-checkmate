use crate::executable::Executable;
use crate::hook::Hook;
use crate::phase::Phase;
use crate::readme::Readme;
use crate::IOResult;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(
    about = env!("CARGO_PKG_DESCRIPTION"),
    author, 
    version,  
    long_about = None, 
)]
pub struct Options {
    #[clap(subcommand)]
    cmd: Option<Subcommand>,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    /// Run all phases.
    Everything,

    #[clap(flatten)]
    Phase(Phase),
    
    #[clap(subcommand)]
    Hook(Hook),
    
    Readme(Readme),
}

impl Options {
    pub fn parse_args() -> Options {
        let mut it = std::env::args().peekable();

        // Skip the binary name:
        it.next();

        // If executed as `cargo checkmate`, the first arg is "checkmate":
        if it.peek().map(|s| s.as_str()) == Some("checkmate") {
            // This will trip up clap parsing, so skip it:
            it.next();
        }

        Self::parse_from(it)
    }
}

impl Executable for Options {
    fn execute(&self) -> IOResult<()> {
        let default = Subcommand::Everything;
        dbg!(self).cmd.as_ref().unwrap_or(&default).execute()
    }
}

impl Executable for Subcommand {
    fn execute(&self) -> IOResult<()> {
        match dbg!(self) {
            Subcommand::Everything => Phase::execute_everything(),
            Subcommand::Phase(x) => x.execute(),
            Subcommand::Hook(x) => x.execute(),
            Subcommand::Readme(x) => x.execute(),
        }
    }
}

