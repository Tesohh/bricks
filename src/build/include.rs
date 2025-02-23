use std::{
    fs,
    path::{Component, Path, PathBuf},
};

use anyhow::Result;

pub fn src_to_include_path(path: impl AsRef<Path>, name: &str) -> PathBuf {
    let mut new_path = PathBuf::new();
    for component in path.as_ref().components() {
        match component {
            Component::Normal(part) if part == "src" => {
                new_path.push(format!("build/include/{}", name))
            }
            Component::Normal(part) => new_path.push(part),
            _ => new_path.push(component),
        }
    }

    new_path
}

pub fn copy_headers(src_path: impl AsRef<Path>, name: &str) -> Result<()> {
    fs::create_dir_all(src_to_include_path(&src_path, name))?;

    for entry in walkdir::WalkDir::new(&src_path).follow_links(true) {
        let entry = entry?;

        let path = entry.path();
        let path_str = path.to_string_lossy();
        if !path_str.ends_with(".h") && !path_str.ends_with(".hpp") {
            continue;
        }

        fs::copy(path, src_to_include_path(path, name))?;
    }
    Ok(())
}
