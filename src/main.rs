use clap::{Parser, Subcommand};
use anyhow::Result;

mod command;
mod schema;
mod utils;

#[derive(Parser)]
struct Cli {
  #[command(subcommand)]
  subcli: SubCli
}

#[derive(Subcommand)]
enum SubCli {
  Init {
    #[arg(long)]
    lib: bool
  },
  Add {
    #[arg(help = "The name of the module to add")]
    name: String
  }
}


fn main() -> Result<()> {
  let args = Cli::parse();

  let ctx = utils::Context::new(None)?;

  match args.subcli {
    SubCli::Init { lib } => {
      command::cmd_init(ctx, lib)
    },
    SubCli::Add { name } => {
      command::cmd_add(ctx, name)
    }
  }
}