use anyhow::Result;
use bricks::{
    build,
    cli::{
        self,
        args::{Args, SubCommand},
        pretty,
    },
    config::read::read_config,
};
use clap::Parser;

fn _main() -> Result<()> {
    let args = Args::parse();

    match args.sub {
        SubCommand::Build(build_command) => {
            match build::build(read_config(&args.config)?, build_command) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        }
        SubCommand::Run(run_command) => cli::run::run(read_config(&args.config)?, run_command),
        SubCommand::Install(install_command) => {
            cli::install::install(read_config(&args.config)?, install_command)
        }
        SubCommand::Clean(clean_command) => {
            cli::clean::clean(read_config(&args.config)?, clean_command)
        }
        SubCommand::Init(init_command) => cli::init::init(init_command),
    }?;

    Ok(())
}

fn main() {
    let result = _main();
    match result {
        Ok(_) => {}
        Err(err) => pretty::error(err),
    }
}
