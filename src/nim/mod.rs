pub mod cache;
pub mod cli;
pub mod commands;
pub mod error;
pub mod git;
pub mod manifest;
pub mod resolve;

// Re-export the key public API for backward compatibility
pub use cache::PackageCache;
pub use error::NimError;
pub use manifest::ProjectManifest;

use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nim::manifest::{DepSource, Dependency, ProjectManifest};

    #[test]
    fn copy_dir_roundtrip() {
        let src = std::env::temp_dir().join("nim_copy_src");
        let dst = std::env::temp_dir().join("nim_copy_dst");
        let _ = std::fs::create_dir_all(src.join("sub"));
        std::fs::write(src.join("a.txt"), "hello").unwrap();
        std::fs::write(src.join("sub").join("b.txt"), "world").unwrap();

        copy_dir(&src, &dst).unwrap();
        assert!(dst.join("a.txt").exists());
        assert!(dst.join("sub").join("b.txt").exists());
        assert_eq!(std::fs::read_to_string(dst.join("a.txt")).unwrap(), "hello");

        let _ = std::fs::remove_dir_all(&src);
        let _ = std::fs::remove_dir_all(&dst);
    }

    #[test]
    fn manifest_add_remove_persistence() {
        let dir = std::env::temp_dir().join("nim_integration_test");
        let _ = std::fs::create_dir_all(&dir);
        let mut m = ProjectManifest::default_for("testproj", &dir);
        m.add_dependency(Dependency {
            name: "mylib".into(),
            source: DepSource::Git {
                url: "https://github.com/user/mylib".into(),
                tag: Some("v1.0".into()),
                branch: None,
                rev: None,
            },
            features: vec![],
        });
        m.save().unwrap();

        let loaded = ProjectManifest::load(&dir).unwrap();
        assert_eq!(loaded.dependencies.len(), 1);
        assert_eq!(loaded.dependencies[0].name, "mylib");

        let _ = std::fs::remove_dir_all(&dir);
    }
}

/// Recursively copy a directory (used by commands)
pub fn copy_dir(src: &Path, dst: &Path) -> Result<(), String> {
    if src.is_file() {
        std::fs::copy(src, dst)
            .map_err(|e| format!("copy {} -> {}: {}", src.display(), dst.display(), e))?;
        return Ok(());
    }
    std::fs::create_dir_all(dst).map_err(|e| format!("cannot create {}: {}", dst.display(), e))?;
    for entry in std::fs::read_dir(src).map_err(|e| format!("read_dir {}: {}", src.display(), e))? {
        let entry = entry.map_err(|e| format!("entry: {}", e))?;
        let file_type = entry.file_type().map_err(|e| format!("file_type: {}", e))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path).map_err(|e| {
                format!(
                    "copy {} -> {}: {}",
                    src_path.display(),
                    dst_path.display(),
                    e
                )
            })?;
        }
    }
    Ok(())
}
