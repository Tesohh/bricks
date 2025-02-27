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

    let override_build = match config.brick.overrides {
        Some(ref v) => &v.build,
        None => &None,
    };

    let build_path = match build::build(
        &config,
        BuildCommand {
            force: run_command.force,
            path: run_command.path.clone(),
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
                Some(override_run_cmd) => {
                    pretty::msg("run", &override_run_cmd);

                    let mut cmd = match std::env::consts::OS {
                        "windows" => {
                            let mut cmd = Command::new("cmd");
                            cmd.arg("/C");
                            cmd
                        }
                        _ => {
                            let mut cmd = Command::new("sh");
                            cmd.arg("-c");
                            cmd
                        }
                    };
                    cmd.current_dir(&run_command.path);
                    cmd.arg(override_run_cmd);
                    cmd.status()?;
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
