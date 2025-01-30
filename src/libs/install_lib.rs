use anyhow::Result;

use crate::config::lib::{Lib, LibKind};

use super::blueprint::{Blueprint, BlueprintFile};

pub fn install_lib(name: &str, lib: &Lib, blueprints: &mut BlueprintFile) -> Result<()> {
    match lib.kind {
        LibKind::System => {
            // you don't need to do anything here.
        }
        LibKind::Git => {
            // if the library isn't already installed:
            // git clone it from the provided source
            // in the library's directory:
            // run bricks install
            // run bricks build
            //
            // if it's already installed, you don't need to do anything.
        }
    };

    // Get lib and header directory from Lib::lib_links and Lib::header
    let libs = lib.lib_links(name)?;
    let headers = lib.headers(name)?;

    // Add blueprint entry to blueprintfile
    blueprints
        .bp
        .insert(name.to_string(), Blueprint { libs, headers });

    // Add to clangd
    // let clangd_path = Path::new(".clangd");
    // pretty::msg(
    //     "add to .clangd",
    //     format!("{} version {}", name, lib.version),
    // );

    // match fs::exists(clangd_path) {
    //     Ok(false) => {
    //         fs::write(clangd_path, templates::clangd())?;
    //     }
    //     Err(_) => {
    //         fs::write(clangd_path, templates::clangd())?;
    //     }
    //     _ => {}
    // }
    //
    // let mut append_file = fs::OpenOptions::new().append(true).open(clangd_path)?;
    //
    // let new_entry = format!("    - \"{}\"\n", lib.headers(name)?);
    // append_file.write_all(new_entry.as_bytes())?;
    Ok(())
}
