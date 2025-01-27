use anyhow::Result;

use crate::cli::args::InitCommand;

use super::templates;

pub fn make_config(cmd: &InitCommand) -> Result<String> {
    let kind = match cmd.lib {
        true => "library",
        false => "binary",
    };

    let lang = match cmd.cpp {
        true => "cpp",
        false => "c",
    };

    let edition = match cmd.cpp {
        true => "c++17",
        false => "c99",
    };

    Ok(templates::config(&cmd.name, kind, lang, edition))
}
