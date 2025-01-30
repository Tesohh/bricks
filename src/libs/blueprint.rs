use std::{collections::HashMap, fs};

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blueprint {
    /// where to -L
    pub libs: String,
    /// where to -I and add to .clangd
    pub headers: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlueprintFile {
    pub bp: HashMap<String, Blueprint>,
}

pub fn read_blueprints() -> Result<BlueprintFile> {
    let toml_str = match fs::read_to_string("build/blueprints.toml") {
        Ok(str) => str,
        Err(_) => {
            bail!("Failed to read build/blueprints.toml.\nPerhaps run `bricks install` first?")
        }
    };
    Ok(toml::from_str(&toml_str)?)
}
