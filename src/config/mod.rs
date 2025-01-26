pub mod brick;
pub mod compiler;
pub mod lib;

use std::collections::HashMap;

use brick::Brick;
use compiler::Compiler;
use lib::Lib;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub brick: Brick,

    #[serde(default)]
    pub compiler: Compiler,

    #[serde(default = "libs_default")]
    pub libs: HashMap<String, Lib>,
}

fn libs_default() -> HashMap<String, Lib> {
    HashMap::new()
}
