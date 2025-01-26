use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::Result;

use crate::{cli::pretty, config::Config};

pub fn binary(
    config: &Config,
    compile_paths: &[PathBuf],
    target: &Path,
) -> Result<Option<PathBuf>> {
    pretty::msg("linking", target.display());

    // TODO: Use platform specific compiler or switch to using env vars
    let mut cmd = &mut Command::new(&config.compiler.mac);
    cmd = cmd.stderr(Stdio::inherit()).arg("-o").arg(target);

    for path in compile_paths {
        cmd = cmd.arg(path);
    }

    // TODO: Do something with the status
    let _status = cmd.status()?;

    Ok(Some(target.to_path_buf()))
}

pub fn library(
    _config: &Config,
    compile_paths: &[PathBuf],
    target: &Path,
) -> Result<Option<PathBuf>> {
    pretty::msg("linking", target.display());

    let mut cmd = &mut Command::new("ar");
    cmd = cmd.stderr(Stdio::inherit()).arg("crus").arg(target);

    for path in compile_paths {
        cmd = cmd.arg(path);
    }

    // TODO: Do something with the status
    let _status = cmd.status()?;

    Ok(Some(target.to_path_buf()))
}
