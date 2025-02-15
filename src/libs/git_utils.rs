use anyhow::{anyhow, bail, Result};
use git2::Oid;

pub trait RepositoryExt {
    fn resolve_commit(&self, version: &str) -> Result<Oid>;
}

impl RepositoryExt for git2::Repository {
    fn resolve_commit(&self, version: &str) -> Result<Oid> {
        let rev = self.revparse_single(version)?;
        println!("HEY");

        match rev.kind() {
            Some(git2::ObjectType::Commit) => Ok(rev.id()),
            Some(git2::ObjectType::Any) => bail!("`version` must resolve to a commit, got `Any`"),
            Some(git2::ObjectType::Tree) => bail!("`version` must resolve to a commit, got `Tree`"),
            Some(git2::ObjectType::Blob) => bail!("`version` must resolve to a commit, got `Blob`"),
            Some(git2::ObjectType::Tag) => bail!("`version` must resolve to a commit, got `Tag`"),
            None => bail!("no rev found for `version` {}", version),
        }
    }
}
