use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Packages {
    pub list: Vec<String>,
    #[serde(default = "src")]
    pub src: String,
}

fn src() -> String {
    "src".into()
}
