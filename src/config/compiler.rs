use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Compiler {
    #[serde(default)]
    pub mac: String,
    #[serde(default)]
    pub windows: String,
    #[serde(default)]
    pub linux: String,
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            mac: "clang".into(),
            windows: "gcc".into(),
            linux: "clang".into(),
        }
    }
}
