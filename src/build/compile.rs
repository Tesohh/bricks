use std::{
    fs,
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{bail, Result};
use walkdir::DirEntry;

use crate::cli::pretty;

pub fn src_to_build_path(path: &Path) -> PathBuf {
    let mut new_path = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(part) if part == "src" => {
                new_path.push("build");
            }
            Component::Normal(part) => {
                new_path.push(part);
            }
            _ => new_path.push(component),
        }
    }
    new_path.set_extension("o");

    new_path
}

pub fn compile(file: walkdir::Result<DirEntry>) -> Result<()> {
    let file = file?;

    if file.file_type().is_dir() {
        return Ok(());
    }

    let src_path = file.path().to_string_lossy();
    if !src_path.to_string().ends_with(".c") {
        return Ok(());
    };

    pretty::msg("compiling", &src_path);

    let build_path = src_to_build_path(file.path());

    let Some(parent) = build_path.parent() else {
        bail!("cannot get parent from build path {}", build_path.display())
    };

    fs::create_dir_all(parent)?;

    let output = Command::new("cc")
        .arg("-c")
        .arg(file.path())
        .arg("-o")
        .arg(build_path)
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        bail!(
            "compilation failed for {}. aborted building.",
            file.path().display()
        );
    }

    Ok(())
}
