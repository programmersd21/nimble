use std::path::{Path, PathBuf};
use crate::nim::error::{NimError, NimResult};

pub struct PackageCache {
    root: PathBuf,
}

impl PackageCache {
    pub fn new() -> NimResult<Self> {
        let home = dirs::home_dir().ok_or(NimError::NoHomeDir)?;
        let root = home.join(".nimble");
        Ok(PackageCache { root })
    }

    pub fn with_root(root: PathBuf) -> Self {
        PackageCache { root }
    }

    pub fn cache_dir(&self) -> &Path { &self.root }
    pub fn bin_dir(&self) -> PathBuf { self.root.join("bin") }
    pub fn repos_dir(&self) -> PathBuf { self.root.join("cache").join("repos") }
    pub fn pkgs_dir(&self) -> PathBuf { self.root.join("cache").join("pkgs") }

    pub fn ensure_dirs(&self) -> NimResult<()> {
        for d in &[self.bin_dir(), self.repos_dir(), self.pkgs_dir()] {
            std::fs::create_dir_all(d)
                .map_err(|e| NimError::cache(format!("cannot create {}: {}", d.display(), e)))?;
        }
        Ok(())
    }

    pub fn repo_cache(&self, url: &str) -> PathBuf {
        let dirname = sanitize(url);
        self.repos_dir().join(dirname)
    }

    pub fn pkg_cache(&self, name: &str, version: &str) -> PathBuf {
        self.pkgs_dir().join(format!("{}@{}", name, version))
    }

    pub fn bin_path(&self, name: &str) -> PathBuf {
        let exe = if cfg!(windows) { format!("{}.exe", name) } else { name.to_string() };
        self.bin_dir().join(exe)
    }

    pub fn is_pkg_cached(&self, name: &str, version: &str) -> bool {
        self.pkg_cache(name, version).exists()
    }
}

fn sanitize(s: &str) -> String {
    s.replace("://", "_")
        .replace("@", "_")
        .replace("/", "_")
        .replace(":", "_")
        .replace(".", "_")
        .replace("-", "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_creates_dirs() {
        let dir = std::env::temp_dir().join("nim_cache_test");
        let _ = std::fs::remove_dir_all(&dir);
        let cache = PackageCache::with_root(dir.clone());
        cache.ensure_dirs().unwrap();
        assert!(cache.bin_dir().exists());
        assert!(cache.repos_dir().exists());
        assert!(cache.pkgs_dir().exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn cache_paths() {
        let cache = PackageCache::with_root(PathBuf::from("/tmp/.nimble"));
        assert!(cache.repo_cache("https://github.com/user/repo").to_string_lossy().contains("user_repo"));
        assert!(cache.pkg_cache("mylib", "1.0.0").to_string_lossy().contains("mylib@1.0.0"));
        let bin = cache.bin_path("mybin");
        #[cfg(windows)]
        assert!(bin.to_string_lossy().ends_with("mybin.exe"));
        #[cfg(not(windows))]
        assert!(bin.to_string_lossy().ends_with("mybin"));
    }

    #[test]
    fn is_pkg_cached() {
        let dir = std::env::temp_dir().join("nim_cache_check");
        let _ = std::fs::create_dir_all(dir.join("cache").join("pkgs").join("testlib@1.0.0"));
        let cache = PackageCache::with_root(dir.clone());
        assert!(cache.is_pkg_cached("testlib", "1.0.0"));
        assert!(!cache.is_pkg_cached("testlib", "2.0.0"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
