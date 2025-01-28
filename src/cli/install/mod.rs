use anyhow::Result;

use crate::{config::Config, libs::install_lib::install_lib};

pub fn install(config: Config) -> Result<()> {
    for (name, lib) in &config.libs {
        install_lib(name, lib)?;
    }

    Ok(())
}
