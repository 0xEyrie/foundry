use clap::{Parser, Subcommand};
use foundry_cli::opts::GlobalArgs;

use crate::generate::GenerateArgs;

const VERSION_MESSAGE: &str = concat!(env!("CARGO_PKG_VERSION"), " (",);

/// Build, test, fuzz, debug and deploy Solidity contracts.
#[derive(Parser)]
#[command(
    name = "mold",
    version = VERSION_MESSAGE,
    after_help = "Find more information in the book: http://book.getfoundry.sh/reference/mold/mold.html",
    next_display_order = None,
)]
pub struct Mold {
    /// Include the global arguments.
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub cmd: MoldSubcommand,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum MoldSubcommand {
    /// Generate solidity types
    #[command(visible_alias = "g")]
    Generate(GenerateArgs),
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Mold::command().debug_assert();
    }
}
