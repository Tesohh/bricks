use std::fs;

use anyhow::{bail, Result};

use crate::{
    cli::pretty,
    config::lib::{Lib, LibKind, LibPathificationError},
};

use super::{copy_dir::copy_dir, git_utils::RepositoryExt};

pub fn install_lib(name: &str, lib: &Lib) -> Result<()> {
    pretty::msg("install", name);
    match lib.kind {
        LibKind::System => {
            // you don't need to do anything here.
        }
        LibKind::Git => {
            let Some(repo_uri) = lib.normalize_repo() else {
                bail!("{} is missing the `repo` property", name);
            };

            // if the library version is already there, use that (and dont do anything)
            let versioned_path = lib.pathify_repo()?;
            if fs::exists(&versioned_path)? {
                return Ok(());
            }

            // check if the FULL lib already exists
            // if the library isn't already installed:
            // git clone it from the provided source
            // else just open it
            let full_path = lib.pathify_repo_no_version()?.join("full");
            let repo: git2::Repository = if !fs::exists(&full_path)? {
                git2::Repository::clone(&repo_uri, &full_path)?
            } else {
                // TODO: this shoould pull also!
                git2::Repository::open(&full_path)?
            };

            // checkout to requested version
            let Some(ref version) = lib.version else {
                return Err(LibPathificationError::VersionMissing.into());
            };
            repo.checkout(version)?;
            copy_dir(&full_path, &versioned_path, &[".git"])?;

            // copy the required version to another directory and use that
            // -

            // in the library's directory:
            // run bricks install
            // run bricks build
            //
            // if it's already installed, you don't need to do anything. https://github.com/Tesohh/brick_test https://github.com/Tesohh/brick_test
        }
    };
    Ok(())
}
