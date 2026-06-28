use crate::nim::error::{NimError, NimResult};
use std::path::{Path, PathBuf};
use std::process::Command;

pub enum GitRef {
    Tag(String),
    Branch(String),
    Rev(String),
}

pub struct GitRepo {
    pub url: String,
    pub cache_dir: PathBuf,
}

impl GitRepo {
    pub fn new(url: &str, cache_root: &Path) -> Self {
        let dirname = sanitize_url(url);
        GitRepo {
            url: url.to_string(),
            cache_dir: cache_root.join(&dirname),
        }
    }

    pub fn clone_repo(&self) -> NimResult<()> {
        if self.cache_dir.join(".git").exists() || self.cache_dir.exists() {
            return Ok(());
        }
        let parent = self.cache_dir.parent().unwrap();
        std::fs::create_dir_all(parent).map_err(|e| {
            NimError::cache(format!(
                "cannot create cache dir {}: {}",
                parent.display(),
                e
            ))
        })?;
        run_git(
            &["clone", "--depth", "1", &self.url],
            Some(&self.cache_dir),
            &self.url,
            "HEAD",
        )
    }

    pub fn fetch(&self) -> NimResult<()> {
        if !self.cache_dir.join(".git").exists() {
            return self.clone_repo();
        }
        run_git(
            &["fetch", "--all", "--tags"],
            Some(&self.cache_dir),
            &self.url,
            "",
        )
    }

    pub fn checkout(&self, git_ref: &GitRef) -> NimResult<()> {
        let refspec = match git_ref {
            GitRef::Tag(t) => format!("tags/{}", t),
            GitRef::Branch(b) => b.clone(),
            GitRef::Rev(r) => r.clone(),
        };
        run_git(
            &["checkout", &refspec],
            Some(&self.cache_dir),
            &self.url,
            &refspec,
        )
    }

    pub fn list_tags(&self) -> NimResult<Vec<String>> {
        let output = Command::new("git")
            .arg("tag")
            .arg("--list")
            .current_dir(&self.cache_dir)
            .output()
            .map_err(|e| NimError::git(format!("cannot list tags: {}", e)))?;
        if !output.status.success() {
            return Ok(vec![]);
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.lines().map(|l| l.to_string()).collect())
    }

    pub fn resolve_ref(&self, git_ref: &GitRef) -> NimResult<String> {
        let ref_str = match git_ref {
            GitRef::Tag(t) => format!("refs/tags/{}", t),
            GitRef::Branch(b) => format!("refs/heads/{}", b),
            GitRef::Rev(r) => r.clone(),
        };
        let output = Command::new("git")
            .args(["rev-parse", "--verify", &ref_str])
            .current_dir(&self.cache_dir)
            .output()
            .map_err(|e| NimError::git(format!("cannot resolve ref: {}", e)))?;
        if !output.status.success() {
            return Err(NimError::GitRefNotFound {
                url: self.url.clone(),
                refspec: ref_str,
            });
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.trim().to_string())
    }

    pub fn resolve_tag(&self, tag: &str) -> NimResult<String> {
        self.resolve_ref(&GitRef::Tag(tag.to_string()))
    }

    pub fn current_commit(&self) -> NimResult<String> {
        let output = Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(&self.cache_dir)
            .output()
            .map_err(|e| NimError::git(format!("cannot get HEAD: {}", e)))?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.trim().to_string())
    }

    pub fn ensure(&self, git_ref: &GitRef) -> NimResult<()> {
        if !self.cache_dir.join(".git").exists() {
            self.clone_repo()?;
        }
        self.checkout(git_ref)
    }

    pub fn source_path(&self) -> &Path {
        &self.cache_dir
    }
}

fn sanitize_url(url: &str) -> String {
    url.replace("://", "_")
        .replace("@", "_")
        .replace("/", "_")
        .replace(":", "_")
        .replace(".", "_")
        .replace("-", "_")
}

fn run_git(args: &[&str], cwd: Option<&Path>, url: &str, refspec: &str) -> NimResult<()> {
    let mut cmd = Command::new("git");
    cmd.args(args);
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    let output = cmd
        .output()
        .map_err(|e| NimError::git(format!("failed to run git: {} (is git on PATH?)", e)))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if refspec.is_empty() || refspec == "HEAD" {
            return Err(NimError::git_clone(
                url.to_string(),
                refspec.to_string(),
                stderr.trim().to_string(),
            ));
        }
        return Err(NimError::git(format!(
            "{}: {}",
            args.join(" "),
            stderr.trim()
        )));
    }
    Ok(())
}
