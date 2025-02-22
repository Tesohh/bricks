use std::{fs, path::Path};

use anyhow::{bail, Result};
use owo_colors::OwoColorize;

use crate::cli::pretty;

use super::Config;

pub fn read_config(config_path: &Path) -> Result<Config> {
    let toml_str = match fs::read_to_string(config_path) {
        Ok(v) => v,
        Err(err) => bail!(
            "while reading config. Are sure you are in a bricks project?\n{}",
            err
        ),
    };

    let config: Config = toml::from_str(&toml_str)?;

    pretty::msg(
        "brick",
        format!(
            "{} {}",
            config.brick.name,
            format!(
                "({}, {}, {})",
                config.brick.kind, config.brick.lang, config.brick.edition
            )
            .dimmed()
        ),
    );

    Ok(config)
}
