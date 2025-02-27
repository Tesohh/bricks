use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{overrides::Overrides, platform::Platform};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrickKind {
    #[serde(alias = "binary")]
    #[serde(alias = "bin")]
    Binary,
    #[serde(alias = "library")]
    #[serde(alias = "lib")]
    Library,
}

impl Display for BrickKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrickKind::Binary => write!(f, "bin"),
            BrickKind::Library => write!(f, "lib"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BrickLang {
    #[serde(alias = "c")]
    C,
    #[serde(alias = "cpp")]
    #[serde(alias = "c++")]
    CPP,
}

impl Display for BrickLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BrickLang::C => write!(f, "c"),
            BrickLang::CPP => write!(f, "c++"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Brick {
    pub name: String,
    pub kind: BrickKind,
    pub lang: BrickLang,
    pub edition: String,
    #[serde(default = "default_cflags")]
    pub cflags: String,
    #[serde(default = "default_ldflags")]
    pub ldflags: String,
    pub overrides: Option<Overrides>,

    macos: Option<Platform>,
    windows: Option<Platform>,
    linux: Option<Platform>,
}

impl Brick {
    pub fn platform(&self) -> Option<&Platform> {
        if cfg!(target_os = "linux") {
            self.linux.as_ref()
        } else if cfg!(target_os = "macos") {
            self.macos.as_ref()
        } else if cfg!(target_os = "windows") {
            self.windows.as_ref()
        } else {
            None
        }
    }
}

fn default_cflags() -> String {
    "".into()
}

fn default_ldflags() -> String {
    "".into()
}
