use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    pub cflags: Option<String>,
    pub ldflags: Option<String>,
}
