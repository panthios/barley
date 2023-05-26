use clap::{Parser, Subcommand};
use anyhow::Result;

mod command;
mod config;

#[derive(Parser)]
struct Cli {
  #[command(subcommand)]
  subcli: SubCli
}

#[derive(Subcommand)]
enum SubCli {
  Init,
  Add {
    #[arg(help = "The name of the module to add")]
    name: String
  }
}


fn main() -> Result<()> {
  let args = Cli::parse();

  match args.subcli {
    SubCli::Init => {
      command::cmd_init()
    },
    SubCli::Add { name } => {
      command::cmd_add(name)
    }
  }
}