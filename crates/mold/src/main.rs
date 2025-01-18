use clap::Parser;
use eyre::Result;
use foundry_cli::{handler, utils};
use opts::{Mold, MoldSubcommand};

mod db;
mod generate;
mod opts;

fn main() {
    if let Err(err) = run() {
        let _ = foundry_common::sh_err!("{err:?}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    handler::install();
    utils::load_dotenv();
    utils::subscriber();
    utils::enable_paint();

    let args = Mold::parse();
    args.global.init()?;

    match args.cmd {
        MoldSubcommand::Generate(cmd) => utils::block_on(cmd.run()),
    }
}
