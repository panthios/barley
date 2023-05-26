use clap::{Parser, Subcommand};
use anyhow::Result;

mod command;
mod schema;
mod utils;

#[derive(Parser, Clone)]
struct Cli {
  #[command(subcommand)]
  subcli: SubCli
}

#[derive(Subcommand, Clone)]
enum SubCli {
  Init {
    #[arg(long)]
    lib: bool
  },
  Add {
    #[arg(help = "The name of the module to add")]
    name: String
  },
  Remove {
    #[arg(help = "The name of the module to remove")]
    name: String
  },
  Build {
    #[arg(long, help = "Set a custom build target")]
    target: Option<String>
  }
}


fn main() -> Result<()> {
  let args = Cli::parse();

  let ctx = match args.subcli.clone() {
    SubCli::Init { .. } => utils::Context::new(None)?,
    SubCli::Add { .. } => utils::Context::new(None)?,
    SubCli::Remove { .. } => utils::Context::new(None)?,
    SubCli::Build { .. } => utils::Context::new(None)?
  };

  match args.subcli {
    SubCli::Init { lib } => command::cmd_init(ctx, lib),
    SubCli::Add { name } => command::cmd_add(ctx, name),
    SubCli::Remove { name } => command::cmd_remove(ctx, name),
    SubCli::Build { target } => command::cmd_build(ctx, target)
  }
}