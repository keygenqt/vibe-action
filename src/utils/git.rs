//! Git clone-once helper for action groups (git2, no system git required).
//! See [`crate::utils`] module-level docs for summary.

use anyhow::Result;
use anyhow::anyhow;
use git2::Repository;
use git2::build::RepoBuilder;
use std::path::Path;

/// Clone `url` into `dest` unless a repository is already there.
/// With `refspec` (branch/tag/commit) performs a full clone and checks it out;
/// without it, a shallow clone of the default branch (depth 1).
pub fn clone_once(url: &str, refspec: Option<&str>, dest: &Path) -> Result<()> {
    // Already cloned — pull-once policy: reuse as-is.
    if Repository::open(dest).is_ok() {
        return Ok(());
    }
    // Empty leftover dir (e.g. after a failed clone) — remove before cloning.
    if dest.exists() {
        let is_empty = std::fs::read_dir(dest)
            .map(|mut it| it.next().is_none())
            .unwrap_or(false);
        if !is_empty {
            anyhow::bail!(
                "Cache dir '{}' exists and is not a git repository.",
                dest.display()
            );
        }
        std::fs::remove_dir_all(dest)?;
    }

    let mut callbacks = git2::RemoteCallbacks::new();
    // ssh-agent for git@ URLs; anonymous for public https.
    callbacks.credentials(
        |_url, username_from_url, _allowed| match username_from_url {
            Some(user) => git2::Cred::ssh_key_from_agent(user),
            None => git2::Cred::default(),
        },
    );
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);
    if refspec.is_none() {
        fo.depth(1);
    }

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo);

    let repo = builder
        .clone(url, dest)
        .map_err(|e| anyhow!("Failed to clone '{}': {}", url, e))?;

    // Checkout pinned ref (branch, tag, or commit).
    if let Some(r) = refspec {
        let obj = repo
            .revparse_single(r)
            .map_err(|e| anyhow!("Failed to resolve ref '{}' in '{}': {}", r, url, e))?;
        let commit = obj
            .peel_to_commit()
            .map_err(|e| anyhow!("Ref '{}' in '{}' is not a commit: {}", r, url, e))?;
        let mut co = git2::build::CheckoutBuilder::new();
        co.force();
        repo.checkout_tree(commit.as_object(), Some(&mut co))
            .map_err(|e| anyhow!("Failed to checkout ref '{}': {}", r, e))?;
        repo.set_head_detached(commit.id())
            .map_err(|e| anyhow!("Failed to detach HEAD at '{}': {}", r, e))?;
    }

    Ok(())
}
