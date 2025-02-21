use anyhow::{bail, Result};

use crate::{config::Config, libs::install_lib::install_lib};

use super::args::InstallCommand;

pub fn install(config: Config, install_command: InstallCommand) -> Result<()> {
    for (name, lib) in &config.libs {
        match install_lib(name, lib, install_command.force, install_command.silent) {
            Ok(_) => {}
            Err(err) => {
                bail!(
                    "during install.{}\nresolve the problem and then run `bricks install --force`",
                    err
                )
            }
        };
    }

    Ok(())
}
