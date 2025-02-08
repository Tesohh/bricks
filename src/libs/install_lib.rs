use std::process::Command;

use anyhow::{bail, Result};

use crate::{
    cli::commandext::CommandExt,
    config::lib::{Lib, LibKind, LibPathificationError},
};

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

            let Some(ref version) = lib.version else {
                return Err(LibPathificationError::VersionMissing.into());
            };

            dbg!(Command::new("git")
                .arg("clone")
                .args(["--depth", "1"])
                .arg("--branch")
                .arg(version)
                .arg(repo_url)
                .arg(dest_path)
                .to_string());

            // in the library's directory:
            // run bricks install
            // run bricks build
            //
            // if it's already installed, you don't need to do anything.
        }
    };

    Ok(())
}
