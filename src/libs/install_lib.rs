use std::fs;

use anyhow::{bail, Result};

use crate::{
    build,
    cli::{
        args::{BuildCommand, InstallCommand},
        install, pretty,
    },
    config::{
        lib::{Lib, LibKind, LibPathificationError},
        overrides::Overrides,
        Config,
    },
};

use super::{copy_dir::copy_dir, git_utils::RepositoryExt};

pub fn install_lib(name: &str, lib: &Lib, force: bool, silent: bool) -> Result<Option<Overrides>> {
    if !silent {
        pretty::msg("install", name);
    }
    match lib.kind {
        LibKind::System => {
            // you don't need to do anything here.
            Ok(None)
        }
        LibKind::Git => {
            let Some(repo_uri) = lib.normalize_repo() else {
                bail!("{} is missing the `repo` property", name);
            };

            // if the library version is already there, use that (and dont do anything)
            let versioned_path = lib.pathify_repo()?;
            if !force && fs::exists(&versioned_path)? {
                // TODO: return the Overrides!
                return Ok(None);
            }

            // check if the FULL lib already exists
            // if the library isn't already installed:
            // git clone it from the provided source
            // else just open it
            let full_path = lib.pathify_repo_no_version()?.join("full");
            let repo: git2::Repository = if !fs::exists(&full_path)? {
                git2::Repository::clone(&repo_uri, &full_path)?
            } else {
                let repo = git2::Repository::open(&full_path)?;
                repo.fetch_all(&repo_uri)?;
                repo
            };

            // checkout to requested version
            let Some(ref version) = lib.version else {
                return Err(LibPathificationError::VersionMissing.into());
            };
            repo.checkout(version)?;

            // copy the required version to another directory and use that
            copy_dir(&full_path, &versioned_path, &[".git"])?;

            // in the library's directory:
            // read the config file
            let foreign_config_file = fs::read_to_string(versioned_path.join("brick.toml"))?;
            let foreign_config: Config = toml::from_str(&foreign_config_file)?;
            // run bricks install
            install::install(
                &foreign_config,
                InstallCommand {
                    path: String::from(versioned_path.to_string_lossy()),
                    force,
                    silent: true,
                },
            )?;

            // run bricks build
            // PERF: remove clone
            let overrides = match lib.overrides.clone() {
                Some(v) => Some(v),
                None => foreign_config.brick.overrides.clone(), // PERF: remove clone
            };

            let build_cmd = BuildCommand {
                force: true,
                emit_compile_commands: false,
                path: String::from(versioned_path.to_string_lossy()),
                silent: true,
            };
            let override_build = match overrides {
                Some(ref v) => &v.build,
                None => &None,
            };
            build::build(&foreign_config, build_cmd, override_build.clone())?;

            Ok(overrides)
        }
    }
}
