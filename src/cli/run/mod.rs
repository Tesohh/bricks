use std::process::Command;

use anyhow::{bail, Result};

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

    let build_path = match build::build(
        &config,
        BuildCommand {
            force: run_command.force,
            path: run_command.path,
            emit_compile_commands: true,
            silent: false,
        },
    )? {
        Some(p) => p,
        None => bail!("build path was not returned"),
    };

    pretty::msg("run", build_path.display());

    Command::new(build_path).status()?;

    Ok(())
}
