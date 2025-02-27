pub mod brick;
pub mod compiler;
pub mod lib;
pub mod overrides;
pub mod platform;
pub mod read;

use std::collections::HashMap;

use brick::Brick;
use lib::Lib;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub brick: Brick,

    #[serde(default = "libs_default")]
    pub libs: HashMap<String, Lib>,
}

fn libs_default() -> HashMap<String, Lib> {
    HashMap::new()
}
