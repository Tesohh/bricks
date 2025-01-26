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
    pub libs: HashMap<String, Lib>,
}
