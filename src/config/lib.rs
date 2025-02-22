use std::{path::PathBuf, process::Command};

use anyhow::Result;
use home::home_dir;
use serde::{Deserialize, Serialize};

use super::overrides::Overrides;

#[derive(Debug, Serialize, Deserialize)]
pub enum LibKind {
    #[serde(alias = "system")]
    System,
    #[serde(alias = "git")]
    Git,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lib {
    pub kind: LibKind,
    pub repo: Option<String>,
    pub version: Option<String>,

    pub overrides: Option<Overrides>,
}

impl Lib {
    pub fn headers(&self, name: &str) -> Result<String> {
        let include_dir = if let Some(overrides) = &self.overrides {
            overrides
                .include_dir
                .as_ref()
                .map(|include_dir| include_dir.to_string())
        } else {
            None
        };
        if let Some(include_dir) = include_dir {
            Ok(include_dir)
        } else {
            match self.kind {
                LibKind::System => {
                    let output = Command::new("pkg-config")
                        .arg(name)
                        .arg("--cflags")
                        .output()?;

                    Ok(String::from_utf8(output.stdout)?.trim_end().to_string())
                }
                LibKind::Git => todo!(),
            }
        }
    }

    pub fn lib_links(&self, name: &str) -> Result<String> {
        let lib_dir = if let Some(overrides) = &self.overrides {
            overrides
                .lib_dir
                .as_ref()
                .map(|include_dir| include_dir.to_string())
        } else {
            None
        };
        if let Some(lib_dir) = lib_dir {
            Ok(lib_dir)
        } else {
            match self.kind {
                LibKind::System => {
                    let output = Command::new("pkg-config")
                        .arg(name)
                        .args(["--libs", "--static"])
                        .output()?;

                    Ok(String::from_utf8(output.stdout)?.trim_end().to_string())
                }
                LibKind::Git => todo!(),
            }
        }
    }

    pub fn normalize_repo(&self) -> Option<String> {
        match &self.repo {
            Some(repo) => {
                let mut new_repo = repo.clone();
                if !repo.starts_with("https://") {
                    new_repo = format!("https://{}", new_repo);
                }
                if !new_repo.ends_with(".git") {
                    new_repo = format!("{}.git", new_repo)
                }
                Some(new_repo)
            }
            None => None,
        }
    }

    pub fn directify_repo(&self) -> Option<String> {
        match &self.repo {
            Some(repo) => {
                let new_repo = repo
                    .trim_start_matches("https://")
                    .trim_end_matches(".git")
                    .replace("/", "-")
                    .to_string();
                Some(new_repo)
            }
            None => None,
        }
    }

    pub fn pathify_repo_no_version(&self) -> Result<PathBuf, LibPathificationError> {
        let home = home_dir().ok_or(LibPathificationError::HomeDirMissing)?;
        let lib_dir = self
            .directify_repo()
            .ok_or(LibPathificationError::RepoUriMissing)?;

        Ok(home.join(".bricks").join("libs").join(lib_dir))
    }

    pub fn pathify_repo(&self) -> Result<PathBuf, LibPathificationError> {
        let home = home_dir().ok_or(LibPathificationError::HomeDirMissing)?;
        let lib_dir = self
            .directify_repo()
            .ok_or(LibPathificationError::RepoUriMissing)?;
        let version = self
            .version
            .as_ref()
            .ok_or(LibPathificationError::VersionMissing)?;

        Ok(home
            .join(".bricks")
            .join("libs")
            .join(lib_dir)
            .join(version))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LibPathificationError {
    #[error("unable to get the home directory")]
    HomeDirMissing,
    #[error("lib is missing the `repo` property")]
    RepoUriMissing,
    #[error("lib is missing the `version` property")]
    VersionMissing,
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn no_repo_should_return_none() {
        let no_repo_lib = Lib {
            kind: LibKind::Git,
            repo: None,
            version: Some("2020".to_string()),
            overrides: None,
        };
        assert_eq!(no_repo_lib.normalize_repo(), None);
        assert_eq!(no_repo_lib.directify_repo(), None);
        assert!(no_repo_lib.pathify_repo().is_err());
    }

    #[test]
    fn repo_normalization() {
        let mut lib = Lib {
            kind: LibKind::Git,
            repo: Some("github.com/Tesohh/strings.git".to_string()),
            version: Some("2020".to_string()),
            overrides: None,
        };
        assert_eq!(
            lib.normalize_repo(),
            Some("https://github.com/Tesohh/strings.git".into())
        );

        lib.repo = Some("https://github.com/Tesohh/strings".into());
        assert_eq!(
            lib.normalize_repo(),
            Some("https://github.com/Tesohh/strings.git".into())
        );

        lib.repo = Some("github.com/Tesohh/strings".into());
        assert_eq!(
            lib.normalize_repo(),
            Some("https://github.com/Tesohh/strings.git".into())
        );
    }

    #[test]
    fn repo_directification() {
        let lib = Lib {
            kind: LibKind::Git,
            repo: Some("github.com/Tesohh/strings.git".to_string()),
            version: Some("2020".to_string()),
            overrides: None,
        };

        assert_eq!(
            lib.directify_repo(),
            Some("github.com-Tesohh-strings".to_string())
        );
    }

    #[test]
    fn repo_pathification() {
        let lib = Lib {
            kind: LibKind::Git,
            repo: Some("github.com/Tesohh/strings.git".to_string()),
            version: Some("2020".to_string()),
            overrides: None,
        };

        let path = lib.pathify_repo().unwrap();

        assert_eq!(
            path,
            Path::new(&home_dir().unwrap())
                .join(".bricks")
                .join("libs")
                .join("github.com-Tesohh-strings")
                .join("2020")
                .to_path_buf()
        );
    }
}
