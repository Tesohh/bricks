use anyhow::{bail, Result};
use git2::Repository;

use crate::config::lib::{Lib, LibKind, LibPathificationError};

use super::git_utils::RepositoryExt;

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

            // TODO: if the library version is already there, use that
            //

            // if it's not already there...
            // check if the full repo is already there
            // if not: clone
            // if it is there: pull to update the library

            // copy the required version to another directory and use that

            // in the library's directory:
            // run bricks install
            // run bricks build https://github.com/Tesohh/brick_test
            //
            // if it's already installed, you don't need to do anything. https://github.com/Tesohh/brick_test https://github.com/Tesohh/brick_test
        }
    };
    Ok(())
}
