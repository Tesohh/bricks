use std::{fs, path::Path};

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::pretty;

use super::Config;

pub fn read_config(config_path: &Path) -> Result<Config> {
    let toml_str = fs::read_to_string(config_path)?;

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
