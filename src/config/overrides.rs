use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Overrides {
    /// the command that will be used to build the library instead of `bricks build`
    pub build: Option<String>,
    /// the directory where the includes (headers) are, instead of the default <lib>/build/include
    pub include_dir: Option<String>,
    /// the directory where the compiled objects are, instead of the default <lib>/build/lib
    pub lib_dir: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct OverrideDatabase(HashMap<String, Overrides>);

impl OverrideDatabase {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn push(&mut self, key: String, ov: Overrides) {
        self.0.insert(key, ov);
    }
}

impl Default for OverrideDatabase {
    fn default() -> Self {
        Self::new()
    }
}
