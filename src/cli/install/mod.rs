use std::{fs, path::Path};

use anyhow::{bail, Result};

use crate::{
    config::{overrides::OverrideDatabase, Config},
    libs::install_lib::install_lib,
};

use super::args::InstallCommand;

pub fn install(config: &Config, install_command: InstallCommand) -> Result<()> {
    let mut override_db = OverrideDatabase::new();

    for (name, lib) in &config.libs {
        match install_lib(name, lib, install_command.force, install_command.silent) {
            Ok(Some(overrides)) => {
                override_db.insert(name.to_string(), overrides);
            }
            Ok(_) => {}
            Err(err) => {
                bail!(
                    "during install.{}\nresolve the problem and then run `bricks install --force`",
                    err
                )
            }
        };
    }

    let override_path = Path::new(&install_command.path)
        .join("build")
        .join("overrides.json");
    let override_file = fs::File::create(override_path)?;
    serde_json::to_writer(override_file, &override_db)?;

    Ok(())
}
