// anvil - Project manifest (Toml) configuration model

use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Top-level project manifest read from `nimble.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct ProjectManifest {
    pub project: ProjectSection,
}

/// The `[project]` section of `nimble.toml`.
#[derive(Debug, Clone, Deserialize)]
pub struct ProjectSection {
    /// Project name.
    pub name: String,
    /// Semantic version.
    pub version: String,
    /// Entry point file (relative to project root).
    #[serde(default = "default_entry")]
    pub entry_point: String,
}

fn default_entry() -> String {
    "src/main.nbl".to_string()
}

impl ProjectManifest {
    /// Load and parse a `nimble.toml` manifest from a project root directory.
    pub fn load(project_dir: &Path) -> Result<Self, String> {
        let path = project_dir.join("nimble.toml");
        let content = fs::read_to_string(&path)
            .map_err(|e| format!("cannot read {}: {}", path.display(), e))?;
        toml::from_str(&content)
            .map_err(|e| format!("invalid nimble.toml: {}", e))
    }

    /// Create a default manifest for `anvil init`.
    pub fn default_for(name: &str) -> Self {
        ProjectManifest {
            project: ProjectSection {
                name: name.to_string(),
                version: "0.1.0".to_string(),
                entry_point: "src/main.nbl".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn parse_valid_manifest() {
        let toml_str = r#"
[project]
name = "myapp"
version = "1.0.0"
entry_point = "src/main.nbl"
"#;
        let manifest: ProjectManifest = toml::from_str(toml_str).unwrap();
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
        let manifest: ProjectManifest = toml::from_str(toml_str).unwrap();
        assert_eq!(manifest.project.entry_point, "src/main.nbl");
    }

    #[test]
    fn load_from_file() {
        let dir = std::env::temp_dir().join("anvil_test_manifest");
        let _ = std::fs::create_dir_all(&dir);
        let mut f = std::fs::File::create(dir.join("nimble.toml")).unwrap();
        f.write_all(b"[project]\nname = \"foo\"\nversion = \"0.2.0\"\n")
            .unwrap();

        let manifest = ProjectManifest::load(&dir).unwrap();
        assert_eq!(manifest.project.name, "foo");
        assert_eq!(manifest.project.version, "0.2.0");

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
        let m = ProjectManifest::default_for("hello");
        assert_eq!(m.project.name, "hello");
        assert_eq!(m.project.version, "0.1.0");
        assert_eq!(m.project.entry_point, "src/main.nbl");
    }
}
