use clap::{Parser, Subcommand};
use anyhow::Result;
mod command;

#[derive(Parser)]
struct Cli {
  #[command(subcommand)]
  subcli: SubCli
}

#[derive(Subcommand)]
enum SubCli {
  Init
}


fn main() -> Result<()> {
  let args = Cli::parse();

  match args.subcli {
    SubCli::Init => {
      command::cmd_init()
    }
  }
}