use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Lib {
    pub src: String,
    #[serde(default = "latest")]
    pub version: String,

    // Compatibility
    pub build: Option<String>,
    pub headers: Option<Vec<String>>,
    pub objects: Option<Vec<String>>,
}

fn latest() -> String {
    "latest".to_string()
}
