use anyhow::Result;
use rayon::iter::{ParallelBridge, ParallelIterator};
use walkdir::DirEntry;

use crate::{
    cli::{args::BuildCommand, pretty},
    config::Config,
};

pub fn compile(file: walkdir::Result<DirEntry>) -> Result<()> {
    let file = file?;

    if file.file_type().is_dir() {
        return Ok(());
    }

    let file_name = file.path().to_string_lossy();
    if !file_name.to_string().ends_with(".c") {
        return Ok(());
    };

    pretty::msg("compiling", &file_name);
    // compile through the CC...

    Ok(())
}

pub fn build(_config: Config, _build_command: BuildCommand) -> Result<()> {
    walkdir::WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .par_bridge()
        .for_each(|file| compile(file).unwrap());

    Ok(())
}
