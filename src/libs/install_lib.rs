use std::process::Command;

use anyhow::{bail, Context, Result};

use crate::config::lib::{Lib, LibKind};

pub fn install_lib(name: &str, lib: &Lib) -> Result<()> {
    match lib.kind {
        LibKind::System => {
            // you don't need to do anything here.
        }
        LibKind::Git => {
            // if the library isn't already installed:
            // git clone it from the provided source

            let Some(repo_url) = lib.normalize_repo() else {
                bail!("{} is missing the `repo` property", name);
            };

            let dest_path = lib.pathify_repo()?;

            dbg!(Command::new("git")
                .arg("clone")
                .arg(repo_url)
                .arg(dest_path));

            // in the library's directory:
            // run bricks install
            // run bricks build
            //
            // if it's already installed, you don't need to do anything.
        }
    };

    Ok(())
}
