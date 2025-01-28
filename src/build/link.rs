use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::Result;

use crate::{cli::pretty, config::lib::Lib};

use super::tools::{get_archiver, get_compiler};

pub fn binary(
    libs: HashMap<String, Lib>,
    compile_paths: &[PathBuf],
    target: &Path,
) -> Result<Option<PathBuf>> {
    pretty::msg("link", target.display());

    let mut cmd = &mut Command::new(get_compiler());
    cmd = cmd.stderr(Stdio::inherit());

    for path in compile_paths {
        cmd = cmd.arg(path);
    }

    for (name, lib) in &libs {
        cmd = cmd.args(lib.lib_links(name)?.split(" "));
        cmd = cmd.args(lib.headers(name)?.split(" "));
    }

    cmd = cmd.arg("-o").arg(target);

    let _status = cmd.status()?;

    Ok(Some(target.to_path_buf()))
}

pub fn library(
    libs: HashMap<String, Lib>,
    compile_paths: &[PathBuf],
    target: &Path,
) -> Result<Option<PathBuf>> {
    pretty::msg("link", target.display());

    let mut cmd = &mut Command::new(get_archiver());
    cmd = cmd.stderr(Stdio::inherit()).arg("crus").arg(target);

    for path in compile_paths {
        cmd = cmd.arg(path);
    }

    for (name, lib) in &libs {
        cmd = cmd.args(lib.lib_links(name)?.split(" "));
        cmd = cmd.args(lib.headers(name)?.split(" "));
    }

    // TODO: Do something with the status
    let _status = cmd.status()?;

    Ok(Some(target.to_path_buf()))
}
