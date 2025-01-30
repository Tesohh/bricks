use std::{
    fs::{self},
    io::Write,
    path::Path,
};

use anyhow::Result;

use crate::{
    cli::{init::templates, pretty},
    config::lib::{Lib, LibKind},
};

pub fn install_lib(name: &str, lib: &Lib) -> Result<()> {
    match lib.kind {
        LibKind::System => {
            let clangd_path = Path::new(".clangd");
            pretty::msg(
                "add to .clangd",
                format!("{} version {}", name, lib.version),
            );

            match fs::exists(clangd_path) {
                Ok(false) => {
                    fs::write(clangd_path, templates::clangd())?;
                }
                Err(_) => {
                    fs::write(clangd_path, templates::clangd())?;
                }
                _ => {}
            }

            let mut append_file = fs::OpenOptions::new().append(true).open(clangd_path)?;

            let new_entry = format!("    - \"{}\"\n", lib.headers(name)?);
            append_file.write_all(new_entry.as_bytes())?;

            Ok(())
        }
        LibKind::Git => todo!(),
    }
}
