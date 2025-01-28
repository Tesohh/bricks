use std::{io::BufRead, process::Command};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum LibKind {
    #[serde(alias = "system")]
    System,
    #[serde(alias = "git")]
    Git,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lib {
    pub kind: LibKind,
    pub repo: Option<String>,

    #[serde(default = "latest")]
    pub version: String,
    // Compatibility
    // pub build: Option<String>,
    // pub headers: Option<Vec<String>>,
    // pub objects: Option<Vec<String>>,
}

impl Lib {
    pub fn headers(&self, name: &str) -> Result<String> {
        match self.kind {
            LibKind::System => {
                let output = Command::new("pkg-config")
                    .arg(name)
                    .arg("--cflags")
                    .output()?;

                Ok(String::from_utf8(output.stdout)?.trim_end().to_string())
            }
            LibKind::Git => todo!(),
        }
    }
    pub fn objects(&self, name: &str) -> Result<String> {
        match self.kind {
            LibKind::System => {
                let output = Command::new("pkg-config")
                    .arg(name)
                    .args(["--libs", "--static"])
                    .output()?;

                Ok(String::from_utf8(output.stdout)?.trim_end().to_string())
            }
            LibKind::Git => todo!(),
        }
    }
}

fn latest() -> String {
    "latest".to_string()
}
