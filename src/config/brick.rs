use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BrickKind {
    #[serde(alias = "binary")]
    #[serde(alias = "bin")]
    Binary,
    #[serde(alias = "library")]
    #[serde(alias = "lib")]
    Library,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BrickLang {
    #[serde(alias = "c")]
    C,
    #[serde(alias = "cpp")]
    #[serde(alias = "c++")]
    CPP,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Brick {
    pub name: String,
    pub kind: BrickKind,
    pub lang: BrickLang,
    pub edition: String,
}
