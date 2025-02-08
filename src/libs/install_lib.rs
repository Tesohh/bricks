use anyhow::{bail, Result};
use git2::Repository;

use crate::config::lib::{Lib, LibKind, LibPathificationError};

use super::git_utils::RepositoryExt;

pub fn install_lib(name: &str, lib: &Lib) -> Result<()> {
    match lib.kind {
        LibKind::System => {
            // you don't need to do anything here.
        }
        LibKind::Git => {
            // if the library isn't already installed:
            // git clone it from the provided source

            let Some(repo_url) = lib.normalize_repo() else {
                bail!("{} is missing the `repo` property", name);
            };

            let dest_path = lib.pathify_repo()?;

            let Some(ref version) = lib.version else {
                return Err(LibPathificationError::VersionMissing.into());
            };

            // init repo, add remote and try to resolve commit
            let repo = Repository::init(&dest_path)?;
            let mut remote = repo.remote_anonymous(&repo_url)?;
            remote.connect(git2::Direction::Fetch)?;

            // try to fetch the commit
            remote.fetch(&[version], None, None)?;
            for x in remote.refspecs() {
                dbg!(x.str());
            }
            // let oid = repo.refname_to_id("FETCH_HEAD")?;
            // dbg!(oid);
            // let oid = repo.resolve_commit(version)?;
            //
            // // checkout to that commi https://github.com/Tesohh/brick_testt
            // repo.set_head_detached(oid)?;
            // repo.checkout_head(None)?;
            // in the library's directory:
            // run bricks install
            // run bricks build https://github.com/Tesohh/brick_test
            //
            // if it's already installed, you don't need to do anything. https://github.com/Tesohh/brick_test https://github.com/Tesohh/brick_test
        }
    };
    Ok(())
}
