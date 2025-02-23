use std::{fs, path::Path};

use anyhow::{bail, Result};

use crate::{
    cli::pretty,
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
                    "during install. {}\nresolve the problem and then run `bricks install --force`",
                    err
                )
            }
        };
    }

    let override_path = Path::new(&install_command.path)
        .join("build")
        .join("overrides.json");
    match fs::create_dir_all(override_path.parent().unwrap()) {
        Ok(_) => {}
        Err(err) => {
            dbg!(err);
        }
    };
    let override_file = fs::File::create(override_path)?;
    serde_json::to_writer(override_file, &override_db)?;

    if !install_command.silent {
        pretty::info("done!");
        pretty::info("you might need to run `bricks build` before seeing headers in your editor");
        pretty::info("if you are still having issues, try `bricks install --force`");
    }

    Ok(())
}
