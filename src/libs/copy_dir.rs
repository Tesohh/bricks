use std::{fs, path::Path};

use anyhow::Result;
use walkdir::WalkDir;

pub fn copy_dir(src: impl AsRef<Path>, dest: impl AsRef<Path>, ignore: &[&str]) -> Result<()> {
    for entry in WalkDir::new(&src) {
        let entry = entry?;
        let relative = entry.path().strip_prefix(&src)?;

        if ignore.iter().any(|i| relative.starts_with(i)) {
            continue;
        }

        let dest_path = dest.as_ref().join(relative);
        if entry.path().is_dir() {
            fs::create_dir_all(dest_path)?;
        } else {
            fs::copy(entry.path(), dest_path)?;
        }
    }

    Ok(())
}
