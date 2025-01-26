use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::Result;

use crate::cli::pretty;

pub fn binary(compile_paths: &[PathBuf], target: &Path) -> Result<Option<PathBuf>> {
    pretty::msg("linking", target.display());

    let mut cmd = &mut Command::new("cc");
    cmd = cmd.stderr(Stdio::inherit()).arg("-o").arg(target);

    for path in compile_paths {
        cmd = cmd.arg(path);
    }

    // TODO: Do something with the status
    let _status = cmd.status()?;

    Ok(Some(target.to_path_buf()))
}

pub fn library(compile_paths: &[PathBuf], target: &Path) -> Result<Option<PathBuf>> {
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
