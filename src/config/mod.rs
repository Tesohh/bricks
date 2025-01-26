pub mod brick;
pub mod lib;
pub mod packages;

use std::collections::HashMap;

use brick::Brick;
use lib::Lib;
use packages::Packages;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub brick: Brick,
    pub packages: Option<Packages>,

    #[serde(default = "libs_default")]
    pub libs: HashMap<String, Lib>,
}

fn libs_default() -> HashMap<String, Lib> {
    HashMap::new()
}
