use std::{
    fs,
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{bail, Context, Result};
use walkdir::DirEntry;

use crate::{
    cli::{commandext::CommandExt, pretty},
    config::Config,
};

use super::{compile_commands::CompileCommand, tools::get_compiler};

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
pub fn compile(
    config: &Config,
    file: walkdir::Result<DirEntry>,
    force: bool,
) -> Result<Option<(PathBuf, CompileCommand)>> {
    let file = file?;

    if file.file_type().is_dir() {
        return Ok(None);
    }

    let src_path_name = file.path().to_string_lossy();
    if !src_path_name.to_string().ends_with(".c") {
        return Ok(None);
    };

    let build_path = src_to_build_path(file.path());

    let src_metadata = file.metadata();
    let build_metadata = fs::metadata(&build_path);

    let Some(build_parent) = build_path.parent() else {
        bail!("cannot get parent from build path {}", build_path.display())
    };

    fs::create_dir_all(build_parent)?;

    let mut cmd = Command::new(get_compiler());
    cmd.arg(format!("-std={}", config.brick.edition))
        .args(config.brick.cflags.split_whitespace())
        .arg("-c")
        .arg(file.path())
        .arg("-o")
        .arg(&build_path)
        .stderr(Stdio::inherit());

    for (name, lib) in &config.libs {
        cmd.args(lib.headers(name)?.split_whitespace());
    }

    // generate command for compile_cmd

    let project_path = file
        .path()
        .parent()
        .context("unable to find parent of src file")?
        .parent()
        .context("unable to find parent of src directory")?;

    let compile_cmd = CompileCommand {
        directory: std::path::absolute(project_path)?.display().to_string(),
        command: cmd.to_string(),
        file: src_path_name.to_string(),
    };

    dbg!(&compile_cmd);

    // skip if no edits
    let skip = match (src_metadata, build_metadata) {
        (Ok(src_metadata), Ok(build_metadata)) => {
            match (src_metadata.modified(), build_metadata.modified()) {
                (Ok(src_modified), Ok(build_modified)) => build_modified >= src_modified,
                _ => false,
            }
        }
        _ => false,
    };

    if !force && skip {
        pretty::msg("skip", &src_path_name);
        return Ok(Some((build_path, compile_cmd)));
    }

    pretty::msg("compile", &src_path_name);

    let output = cmd.output()?;

    if !output.status.success() {
        bail!(
            "compilation failed for {}. aborted building.",
            file.path().display()
        );
    }

    Ok(Some((build_path, compile_cmd)))
}
