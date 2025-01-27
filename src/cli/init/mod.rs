pub mod make_config;
pub mod templates;

use std::{fs, path::Path};

use anyhow::{bail, Result};
use make_config::make_config;

use super::{args::InitCommand, pretty};

pub fn init(cmd: InitCommand) -> Result<()> {
    let config = make_config(&cmd)?;

    if fs::exists(&cmd.name)? {
        bail!("directory `{}` is not empty", &cmd.name)
    }

    let project_path = Path::new(&cmd.name);

    fs::create_dir(project_path)?;
    fs::write(project_path.join("brick.toml"), config)?;

    let src_path = project_path.join("src");
    fs::create_dir(&src_path)?;

    match cmd.cpp {
        true => fs::write(src_path.join("main.cpp"), templates::main_cpp(&cmd.name)),
        false => fs::write(src_path.join("main.c"), templates::main_c(&cmd.name)),
    }?;

    pretty::msg("init", &cmd.name);
    pretty::info(format!("run `cd {}` to start developing", &cmd.name));
    if !cmd.lib {
        pretty::info("run `bricks run` to run your project");
    }
    pretty::info("run `bricks build` to build your project");

    Ok(())
}
