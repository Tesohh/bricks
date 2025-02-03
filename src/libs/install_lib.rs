use std::process::Command;

use anyhow::Result;

use crate::config::lib::{Lib, LibKind};

use super::blueprint::{Blueprint, BlueprintFile};

pub fn install_lib(name: &str, lib: &Lib, blueprints: &mut BlueprintFile) -> Result<()> {
    match lib.kind {
        LibKind::System => {
            // you don't need to do anything here.
        }
        LibKind::Git => {
            // if the library isn't already installed:
            // git clone it from the provided source

            Command::new("git").arg("clone");
            // in the library's directory:
            // run bricks install
            // run bricks build
            //
            // if it's already installed, you don't need to do anything.
        }
    };

    // Get lib and header directory from Lib::lib_links and Lib::header
    let libs = lib.lib_links(name)?;
    let headers = lib.headers(name)?;

    // Add blueprint entry to blueprintfile
    blueprints
        .bp
        .insert(name.to_string(), Blueprint { libs, headers });

    Ok(())
}
