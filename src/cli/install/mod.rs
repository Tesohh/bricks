use anyhow::{bail, Result};

use crate::{config::Config, libs::install_lib::install_lib};

use super::args::InstallCommand;

// TODO: should take path??
pub fn install(config: Config, install_command: InstallCommand) -> Result<()> {
    for (name, lib) in &config.libs {
        match install_lib(name, lib, install_command.force) {
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
