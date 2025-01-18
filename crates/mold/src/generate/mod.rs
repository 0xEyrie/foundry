use clap::{command, Parser};
use eyre::Result;
use foundry_cli::opts::GlobalArgs;

/// CLI arguments for `forge test`.
#[derive(Clone, Debug, Parser)]
#[command(next_help_heading = "Test options")]
pub struct GenerateArgs {
    // Include global options for users of this struct.
    #[command(flatten)]
    pub global: GlobalArgs,
}

impl GenerateArgs {
    pub fn run(self) -> Result<()> {
        Ok(())
    }
}
