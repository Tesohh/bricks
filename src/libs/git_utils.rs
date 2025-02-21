use anyhow::{bail, Result};
use git2::Oid;

pub trait RepositoryExt {
    fn resolve_commit(&self, version: &str) -> Result<Oid>;
    fn checkout(&self, version: &str) -> Result<()>;
    fn fetch_all(&self, repo_uri: &str) -> Result<()>;
}

impl RepositoryExt for git2::Repository {
    fn resolve_commit(&self, version: &str) -> Result<Oid> {
        let rev = self.revparse_single(version)?;

        match rev.kind() {
            Some(git2::ObjectType::Commit) => Ok(rev.id()),
            Some(git2::ObjectType::Any) => bail!("`version` must resolve to a commit, got `Any`"),
            Some(git2::ObjectType::Tree) => bail!("`version` must resolve to a commit, got `Tree`"),
            Some(git2::ObjectType::Blob) => bail!("`version` must resolve to a commit, got `Blob`"),
            Some(git2::ObjectType::Tag) => bail!("`version` must resolve to a commit, got `Tag`"),
            None => bail!("no rev found for `version` {}", version),
        }
    }
    fn checkout(&self, version: &str) -> Result<()> {
        let obj = self.revparse_single(version)?;
        let commit = obj.peel_to_commit()?;
        self.checkout_tree(
            commit.as_object(),
            Some(git2::build::CheckoutBuilder::new().force()),
        )?;
        self.set_head_detached(commit.id())?;
        Ok(())
    }

    fn fetch_all(&self, repo_uri: &str) -> Result<()> {
        let mut remote = self.remote_anonymous(repo_uri)?;
        Ok(remote.fetch(&["+refs/heads/*:refs/remotes/temp/*"], None, None)?)
    }
}
