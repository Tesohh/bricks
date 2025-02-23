use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::{
    build,
    cli::{
        args::{BuildCommand, RunCommand},
        pretty,
    },
    config::{brick::BrickKind, Config},
};

pub fn run(config: Config, run_command: RunCommand) -> Result<()> {
    if let BrickKind::Library = config.brick.kind {
        bail!("cannot run a library")
    }

    let override_build = match config.brick.overrides {
        Some(ref v) => &v.build,
        None => &None,
    };

    let build_path = match build::build(
        &config,
        BuildCommand {
            force: run_command.force,
            path: run_command.path,
            emit_compile_commands: true,
            silent: false,
        },
        override_build.clone(),
    )? {
        Some(p) => p,
        None => {
            let override_run = match config.brick.overrides {
                Some(v) => v.run,
                None => None,
            };

            match override_run {
                Some(cmd) => {
                    let mut words = cmd.split_whitespace();
                    let program = words.next().context("run command is an empty string")?;
                    let args: Vec<_> = words.collect();

                    pretty::msg("run", &cmd);

                    let _ = Command::new(program).args(args).status()?;
                    return Ok(());
                }
                None => {
                    bail!("build path was not returned, and a `run` override was not specified")
                }
            }
        }
    };

    pretty::msg("run", build_path.display());

    Command::new(build_path).status()?;

    Ok(())
}
