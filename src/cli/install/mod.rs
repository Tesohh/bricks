use std::{collections::HashMap, fs};

use anyhow::Result;

use crate::{
    config::Config,
    libs::{blueprint::BlueprintFile, install_lib::install_lib},
};

use super::args::InstallCommand;

// TODO: should take path??
pub fn install(config: Config, _install_command: InstallCommand) -> Result<()> {
    let mut blueprints = BlueprintFile { bp: HashMap::new() };

    for (name, lib) in &config.libs {
        install_lib(name, lib, &mut blueprints)?;
    }

    Ok(())
}
