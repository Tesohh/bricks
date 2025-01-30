use std::path::PathBuf;

use clap::Parser;
use clap_derive::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// Build an executable or library
    Build(BuildCommand),

    /// Build and run the binary directly
    Run(RunCommand),

    /// Initialize new project
    Init(InitCommand),

    /// Install required libraries
    Install(InstallCommand),

    /// Cleans the build directory
    Clean(CleanCommand),
}

#[derive(Debug, Args)]
pub struct BuildCommand {
    #[arg(long, required = false)]
    pub force: bool,

    #[clap(default_value = ".")]
    pub path: String,
}

#[derive(Debug, Args)]
pub struct RunCommand {
    #[arg(long, required = false)]
    pub force: bool,

    #[clap(default_value = ".")]
    pub path: String,
}

#[derive(Debug, Args)]
pub struct InitCommand {
    pub name: String,

    #[arg(long, required = false)]
    pub cpp: bool,

    #[arg(long, required = false)]
    /// Sets this project as lib
    pub lib: bool,
}

#[derive(Debug, Args)]
pub struct CleanCommand {
    #[clap(default_value = ".")]
    pub path: String,
}

#[derive(Debug, Args)]
pub struct InstallCommand {
    #[clap(default_value = ".")]
    pub path: String,
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[command(name = "bricks")]
#[command(bin_name = "bricks")]
#[command(styles = CLAP_STYLING)]
/// Gissy
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

pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(
        anstyle::Style::new()
            .bold()
            .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Magenta))),
    )
    .usage(
        anstyle::Style::new()
            .bold()
            .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Magenta))),
    )
    .literal(
        anstyle::Style::new()
            .bold()
            .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Blue))),
    );
// .placeholder(clap_cargo::style::PLACEHOLDER)
// .error(clap_cargo::style::ERROR)
// .valid(clap_cargo::style::VALID)
// .invalid(clap_cargo::style::INVALID);
