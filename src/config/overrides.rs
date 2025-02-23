use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Overrides {
    /// the command that will be used to build the library instead of `bricks build`
    pub build: Option<String>,
    /// the command that will be used to run the binary instead of `bricks run`
    pub run: Option<String>,
    /// the directory where the includes (headers) are, instead of the default <lib>/build/include
    pub include_dir: Option<String>,
    /// the directory where the compiled objects are, instead of the default <lib>/build/lib
    pub lib_dir: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OverrideDatabase(HashMap<String, Overrides>);

impl OverrideDatabase {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: String, ov: Overrides) {
        self.0.insert(key, ov);
    }

    pub fn get(&self, key: &str) -> Option<&Overrides> {
        self.0.get(key)
    }
}

impl Default for OverrideDatabase {
    fn default() -> Self {
        Self::new()
    }
}
