use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::Result;

use crate::{
    cli::pretty,
    config::{brick::BrickLang, lib::Lib, overrides::OverrideDatabase},
};

use super::tools::{get_archiver, get_compiler};

pub fn binary(
    libs: &HashMap<String, Lib>,
    compile_paths: &[PathBuf],
    target: &Path,
    override_db: &OverrideDatabase,
    ldflags: &str,
    lang: BrickLang,
    silent: bool,
) -> Result<Option<PathBuf>> {
    if !silent {
        pretty::msg("link", target.display());
    }

    let mut cmd = &mut Command::new(get_compiler(lang));
    cmd = cmd.stderr(Stdio::inherit());

    for path in compile_paths {
        cmd = cmd.arg(path);
    }

    for (name, lib) in libs {
        cmd = cmd.args(lib.lib_links(name, override_db)?.split(" "));
        cmd = cmd.args(lib.headers(name, override_db)?.split(" "));
    }

    cmd.args(ldflags.split_whitespace().collect::<Vec<&str>>());

    cmd = cmd.arg("-o").arg(target);

    let _status = cmd.status()?;

    Ok(Some(target.to_path_buf()))
}

pub fn library(
    libs: &HashMap<String, Lib>,
    compile_paths: &[PathBuf],
    target: &Path,
    override_db: &OverrideDatabase,
    ldflags: &str,
    _lang: BrickLang,
    silent: bool,
) -> Result<Option<PathBuf>> {
    if !silent {
        pretty::msg("link", target.display());
    }

    let mut cmd = &mut Command::new(get_archiver());
    cmd = cmd.stderr(Stdio::inherit()).arg("crus").arg(target);

    for path in compile_paths {
        cmd = cmd.arg(path);
    }

    for (name, lib) in libs {
        cmd = cmd.args(lib.lib_links(name, override_db)?.split(" "));
        cmd = cmd.args(lib.headers(name, override_db)?.split(" "));
    }

    cmd.args(ldflags.split_whitespace().collect::<Vec<&str>>());

    let _status = cmd.status()?;

    Ok(Some(target.to_path_buf()))
}
