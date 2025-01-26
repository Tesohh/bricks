use std::path::PathBuf;

use clap::Parser;
use clap_derive::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// Build an executable or dynamic library
    Build(BuildCommand),
    /// Build and run the binary directly
    Run(RunCommand),
}

#[derive(Debug, Args)]
pub struct BuildCommand {}

#[derive(Debug, Args)]
pub struct RunCommand {}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[command(name = "bricks")]
#[command(bin_name = "bricks")]
pub struct Args {
    #[clap(subcommand)]
    pub sub: SubCommand,

    #[arg(
        short,
        long,
        value_name = "CONFIG PATH",
        required = false,
        default_value = "./brick.toml"
    )]
    pub config: PathBuf,
}
