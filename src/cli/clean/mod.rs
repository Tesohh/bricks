use std::fs;

use anyhow::Result;

use crate::{cli::pretty, config::Config};

pub fn clean(_config: Config) -> Result<()> {
    pretty::msg("rm -rf", "./build");
    Ok(fs::remove_dir_all("./build")?)
}
