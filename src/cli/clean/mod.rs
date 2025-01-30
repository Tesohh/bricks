use std::{fs, path::Path};

use anyhow::Result;

use crate::{cli::pretty, config::Config};

use super::args::CleanCommand;

pub fn clean(_config: Config, clean_command: CleanCommand) -> Result<()> {
    let build_path = Path::new(&clean_command.path).join("build");
    pretty::msg("rm -rf", build_path.display());
    Ok(fs::remove_dir_all(build_path)?)
}
