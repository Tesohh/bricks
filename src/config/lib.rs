use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Result;
use home::home_dir;
use serde::{Deserialize, Serialize};

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

    #[serde(default = "latest")]
    pub version: String,
    // Compatibility
    // pub build: Option<String>,
    // pub headers: Option<Vec<String>>,
    // pub objects: Option<Vec<String>>,
}

impl Lib {
    pub fn headers(&self, name: &str) -> Result<String> {
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

    pub fn lib_links(&self, name: &str) -> Result<String> {
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

    pub fn pathify_repo(&self) -> Option<PathBuf> {
        Some(home_dir()?.join(self.directify_repo()?).join(&self.version))
    }
}

fn latest() -> String {
    "latest".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_repo_should_return_none() {
        let no_repo_lib = Lib {
            kind: LibKind::Git,
            repo: None,
            version: "2020".to_string(),
        };
        assert_eq!(no_repo_lib.normalize_repo(), None);
        assert_eq!(no_repo_lib.directify_repo(), None);
        assert_eq!(no_repo_lib.pathify_repo(), None);
    }

    #[test]
    fn repo_normalization() {
        let mut lib = Lib {
            kind: LibKind::Git,
            repo: Some("github.com/Tesohh/strings.git".to_string()),
            version: "2020".to_string(),
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
            version: "2020".to_string(),
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
            version: "2020".to_string(),
        };

        assert_eq!(
            lib.pathify_repo(),
            Some(
                Path::new(&home_dir().unwrap())
                    .join("github.com-Tesohh-strings")
                    .join("2020")
                    .to_path_buf()
            )
        );
    }
}
