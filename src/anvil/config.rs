use std::io::Write;
use std::path::Path;
use crate::nim::manifest::ProjectManifest;

/// Create a default `nimble.toml` for `anvil init`.
pub fn default_manifest(name: &str) -> ProjectManifest {
    ProjectManifest::default_for(name, Path::new("."))
}

/// Write a minimal `nimble.toml` for a new project.
pub fn write_init_manifest(dir: &Path, manifest: &ProjectManifest) -> Result<(), String> {
    let toml_str = format!(
        r#"[project]
name = "{}"
version = "{}"
entry_point = "{}"
"#,
        manifest.project.name, manifest.project.version, manifest.project.entry_point,
    );
    let manifest_path = dir.join("nimble.toml");
    let mut f = std::fs::File::create(&manifest_path)
        .map_err(|e| format!("failed to create {}: {}", manifest_path.display(), e))?;
    f.write_all(toml_str.as_bytes())
        .map_err(|e| format!("failed to write {}: {}", manifest_path.display(), e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nim::manifest::ProjectManifest;

    #[test]
    fn parse_valid_manifest() {
        let toml_str = r#"
[project]
name = "myapp"
version = "1.0.0"
entry_point = "src/main.nbl"
"#;
        let manifest = ProjectManifest::parse(Path::new("test.toml"), toml_str).unwrap();
        assert_eq!(manifest.project.name, "myapp");
        assert_eq!(manifest.project.version, "1.0.0");
        assert_eq!(manifest.project.entry_point, "src/main.nbl");
    }

    #[test]
    fn parse_manifest_default_entry() {
        let toml_str = r#"
[project]
name = "test"
version = "0.1.0"
"#;
        let manifest = ProjectManifest::parse(Path::new("test.toml"), toml_str).unwrap();
        assert_eq!(manifest.project.entry_point, "src/main.nbl");
    }

    #[test]
    fn load_from_file() {
        let dir = std::env::temp_dir().join("anvil_test_manifest");
        let _ = std::fs::create_dir_all(&dir);
        let mut f = std::fs::File::create(dir.join("nimble.toml")).unwrap();
        f.write_all(b"[project]\nname = \"foo\"\nversion = \"0.2.1\"\n").unwrap();

        let manifest = ProjectManifest::load(&dir).unwrap();
        assert_eq!(manifest.project.name, "foo");
        assert_eq!(manifest.project.version, "0.2.1");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_missing_file() {
        let dir = std::env::temp_dir().join("anvil_test_missing");
        let _ = std::fs::create_dir_all(&dir);
        assert!(ProjectManifest::load(&dir).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn default_for_creates_valid() {
        let m = default_manifest("hello");
        assert_eq!(m.project.name, "hello");
        assert_eq!(m.project.version, "0.1.0");
        assert_eq!(m.project.entry_point, "src/main.nbl");
    }
}
