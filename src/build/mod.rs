pub mod compile;
pub mod compile_commands;
pub mod include;
pub mod link;
pub mod tools;

use std::{
    fs::{self},
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result};
use compile_commands::CompileDatabase;

use crate::{
    cli::{args::BuildCommand, pretty},
    config::{brick::BrickKind, overrides::OverrideDatabase, Config},
};

pub fn build(
    config: &Config,
    build_command: BuildCommand,
    override_cmd: Option<String>,
) -> Result<Option<PathBuf>> {
    if let Some(override_cmd) = override_cmd {
        let mut words = override_cmd.split_whitespace();
        let program = words.next().context("build command is an empty string")?;
        let args: Vec<_> = words.collect();

        pretty::msg("build", &override_cmd);

        let _ = Command::new(program).args(args).status()?;
        return Ok(None);
    }

    let mut compile_paths = vec![];

    let src_path = Path::new(&build_command.path).join("src");

    let override_path = Path::new(&build_command.path)
        .join("build")
        .join("overrides.json");
    let override_file = match fs::read_to_string(override_path) {
        Ok(v) => v,
        Err(_) => {
            pretty::warning(format!(
                "no `build/overrides.json` found in {}. defaulting to no overrides.",
                &build_command.path
            ));
            "{}".to_string()
        }
    };
    let override_db: OverrideDatabase = serde_json::from_str(&override_file)?;

    let mut compile_db = CompileDatabase::new();

    for entry in walkdir::WalkDir::new(&src_path).follow_links(true) {
        let Some((path, compile_cmd)) = compile::compile(
            config,
            entry,
            &override_db,
            build_command.force,
            build_command.silent,
        )?
        else {
            continue;
        };
        compile_paths.push(path);
        compile_db.push(compile_cmd);
    }

    let build_result = match config.brick.kind {
        BrickKind::Binary => link::binary(
            &config.libs,
            &compile_paths,
            &Path::new(&build_command.path)
                .join("build")
                .join(&config.brick.name),
            &override_db,
            build_command.silent,
        ),
        BrickKind::Library => link::library(
            &config.libs,
            &compile_paths,
            &Path::new(&build_command.path)
                .join("build")
                .join("lib")
                .join(String::from("lib") + &config.brick.name + ".a"),
            &override_db,
            build_command.silent,
        ),
    };

    if build_command.emit_compile_commands {
        if !build_command.silent {
            pretty::msg("emit", "build/compile_commands.json");
        }
        let comp_path = Path::new(&build_command.path)
            .join("build")
            .join("compile_commands.json");
        let comp_file = fs::File::create(comp_path)?;
        serde_json::to_writer(comp_file, &compile_db)?;
    };

    include::copy_headers(&src_path)?;

    build_result
}

#[cfg(test)]
mod tests {
    use crate::build::compile::src_to_build_path;
    use std::path::Path;

    #[test]
    fn check_src_to_build_path() {
        assert_eq!(
            src_to_build_path(Path::new("./src/main.c")),
            Path::new("./build/main.o")
        );
        assert_eq!(
            src_to_build_path(Path::new("./src/utils/crazy/main.c")),
            Path::new("./build/utils/crazy/main.o")
        );
        assert_eq!(
            src_to_build_path(Path::new("./main.c")),
            Path::new("./main.o")
        );
    }
}
