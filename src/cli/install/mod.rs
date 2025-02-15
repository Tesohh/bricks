use anyhow::Result;

use crate::{config::Config, libs::install_lib::install_lib};

use super::args::InstallCommand;

// TODO: should take path??
pub fn install(config: Config, _install_command: InstallCommand) -> Result<()> {
    for (name, lib) in &config.libs {
        install_lib(name, lib)?;
    }

    Ok(())
}
