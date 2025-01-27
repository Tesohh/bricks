use std::{
    fs,
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{bail, Result};
use walkdir::DirEntry;

use crate::{cli::pretty, config::Config};

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

/// Compiles a file through `cc` and returns where it was compiled to
pub fn compile(config: &Config, file: walkdir::Result<DirEntry>) -> Result<Option<PathBuf>> {
    let file = file?;

    if file.file_type().is_dir() {
        return Ok(None);
    }

    let src_path_name = file.path().to_string_lossy();
    if !src_path_name.to_string().ends_with(".c") {
        return Ok(None);
    };

    let build_path = src_to_build_path(file.path());

    let src_metadata = fs::metadata(file.path());
    let build_metadata = fs::metadata(&build_path);

    let skip = match (src_metadata, build_metadata) {
        (Ok(src_metadata), Ok(build_metadata)) => {
            match (src_metadata.modified(), build_metadata.modified()) {
                (Ok(src_modified), Ok(build_modified)) => build_modified >= src_modified,
                _ => false,
            }
        }
        _ => true,
    };

    if skip {
        pretty::msg("skipping", &src_path_name);
        return Ok(Some(build_path));
    }

    pretty::msg("compiling", &src_path_name);

    let Some(parent) = build_path.parent() else {
        bail!("cannot get parent from build path {}", build_path.display())
    };

    fs::create_dir_all(parent)?;

    // TODO: Use platform specific compiler or switch to using env vars
    let output = Command::new(&config.compiler.mac)
        .arg(String::from("-std=") + &config.brick.edition)
        .arg("-c")
        .arg(file.path())
        .arg("-o")
        .arg(&build_path)
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        bail!(
            "compilation failed for {}. aborted building.",
            file.path().display()
        );
    }

    Ok(Some(build_path))
}
