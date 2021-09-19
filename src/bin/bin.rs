use pngme::args::Args;
use pngme::commands;

use structopt::StructOpt;

use anyhow;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> anyhow::Result<()> {
    let cli = Args::from_args();

    commands::run_cmd(cli)?;

    Ok(())
}
