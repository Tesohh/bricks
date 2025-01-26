use std::{env, fs};

use anyhow::{bail, Result};
use bricks::{
    cli::{args::Args, pretty},
    config::Config,
};
use clap::Parser;
use owo_colors::OwoColorize;

// TODO: get the file from clap args

fn _main() -> Result<()> {
    let args = Args::parse();

    let toml_str = fs::read_to_string(args.config.as_path())?;

    let config: Config = toml::from_str(&toml_str)?;

    pretty::msg(
        "brick",
        format!(
            "{} {}",
            config.brick.name,
            format!(
                "({}, {}, {})",
                config.brick.kind, config.brick.lang, config.brick.edition
            )
            .dimmed()
        ),
    );
    Ok(())
}

fn main() {
    let result = _main();
    match result {
        Ok(_) => {}
        Err(err) => pretty::error(err),
    }
}

/*
    let path = env::current_dir()?;
    let mut dir = fs::read_dir(path)?;

    let config_file = dir.find(|entry| {
        let file = match entry {
            Ok(f) => f,
            Err(_) => return false,
        };

        let Ok(ft) = file.file_type() else {
            return false;
        };

        ft.is_file() && file.file_name() == "brick.toml"
    });

    let config_file = match config_file {
        Some(f) => f,
        None => bail!("`brick.toml` file not found"),
    };
*/
