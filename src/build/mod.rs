pub mod compile;
pub mod compile_commands;
pub mod link;
pub mod tools;

use std::path::{Path, PathBuf};

use anyhow::Result;
use compile_commands::CompileDatabase;

use crate::{
    cli::args::BuildCommand,
    config::{brick::BrickKind, Config},
};

pub fn build(config: Config, build_command: BuildCommand) -> Result<Option<PathBuf>> {
    let mut compile_paths = vec![];

    let src_path = Path::new(&build_command.path).join("src");

    let mut compile_db = CompileDatabase::new();

    for entry in walkdir::WalkDir::new(src_path).follow_links(true) {
        let Some((path, compile_cmd)) = compile::compile(&config, entry, build_command.force)?
        else {
            continue;
        };
        compile_paths.push(path);
        compile_db.push(compile_cmd);
    }

    match config.brick.kind {
        BrickKind::Binary => link::binary(
            config.libs,
            &compile_paths,
            &Path::new(&build_command.path)
                .join("build")
                .join(&config.brick.name),
        ),
        BrickKind::Library => link::library(
            config.libs,
            &compile_paths,
            &Path::new(&build_command.path)
                .join("build")
                .join("lib")
                .join(String::from("lib") + &config.brick.name + ".a"),
        ),
    }

    // TODO: Emit compile db (if asked to)
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
