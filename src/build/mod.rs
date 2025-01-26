pub mod compile;

use anyhow::Result;

use crate::{cli::args::BuildCommand, config::Config};

pub fn build(_config: Config, _build_command: BuildCommand) -> Result<()> {
    for entry in walkdir::WalkDir::new("./src").follow_links(true) {
        compile::compile(entry)?;
    }

    Ok(())
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
