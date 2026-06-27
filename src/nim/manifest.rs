use crate::nim::error::{NimError, NimResult};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum DepSource {
    Git {
        url: String,
        tag: Option<String>,
        branch: Option<String>,
        rev: Option<String>,
    },
    Path(PathBuf),
    Version(String),
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub source: DepSource,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectSection {
    pub name: String,
    pub version: String,
    #[serde(default = "default_entry")]
    pub entry_point: String,
    #[serde(default)]
    pub edition: String,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub license: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    pub readme: Option<String>,
}

fn default_entry() -> String {
    "src/main.nbl".to_string()
}

#[derive(Debug, Clone)]
pub struct DepEntry {
    pub source: DepSource,
    pub features: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FeaturesSection {
    pub default: Vec<String>,
    pub optional: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Profile {
    pub opt_level: Option<u32>,
    pub lto: Option<bool>,
    pub strip: Option<bool>,
    pub debug: Option<bool>,
    pub panic: Option<String>,
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            opt_level: None,
            lto: None,
            strip: None,
            debug: None,
            panic: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProjectManifest {
    pub path: PathBuf,
    pub project: ProjectSection,
    pub dependencies: Vec<Dependency>,
    pub dev_dependencies: Vec<Dependency>,
    pub build_dependencies: Vec<Dependency>,
    pub features: FeaturesSection,
    pub profiles: HashMap<String, Profile>,
}

impl ProjectManifest {
    pub fn load(project_dir: &Path) -> NimResult<Self> {
        let path = project_dir.join("nimble.toml");
        let raw = std::fs::read_to_string(&path)
            .map_err(|e| NimError::file_read(&path, e.to_string()))?;
        Self::parse(&path, &raw)
    }

    pub fn parse(path: &Path, raw: &str) -> NimResult<Self> {
        let raw_val: toml::Value =
            toml::from_str(raw).map_err(|e| NimError::invalid_manifest(path, e.to_string()))?;

        let project: ProjectSection = raw_val
            .get("project")
            .ok_or_else(|| NimError::missing_field("project".to_string(), path))
            .and_then(|v| {
                ProjectSection::deserialize(v.clone())
                    .map_err(|e| NimError::invalid_manifest(path, e.to_string()))
            })?;

        let dependencies = parse_dep_table(raw_val.get("dependencies"), path)?;
        let dev_dependencies = parse_dep_table(raw_val.get("dev-dependencies"), path)?;
        let build_dependencies = parse_dep_table(raw_val.get("build-dependencies"), path)?;

        let features = parse_features(raw_val.get("features"), path);
        let profiles = parse_profiles(raw_val.get("profile"), path);

        Ok(ProjectManifest {
            path: path.to_path_buf(),
            project,
            dependencies,
            dev_dependencies,
            build_dependencies,
            features,
            profiles,
        })
    }

    pub fn default_for(name: &str, dir: &Path) -> Self {
        ProjectManifest {
            path: dir.join("nimble.toml"),
            project: ProjectSection {
                name: name.to_string(),
                version: "0.1.0".to_string(),
                entry_point: "src/main.nbl".to_string(),
                edition: "2024".to_string(),
                repository: None,
                homepage: None,
                documentation: None,
                license: None,
                authors: vec![],
                keywords: vec![],
                categories: vec![],
                readme: None,
            },
            dependencies: vec![],
            dev_dependencies: vec![],
            build_dependencies: vec![],
            features: FeaturesSection {
                default: vec![],
                optional: HashMap::new(),
            },
            profiles: HashMap::new(),
        }
    }

    pub fn save(&self) -> NimResult<()> {
        let mut out = String::new();
        out.push_str(&format!("[project]\n"));
        out.push_str(&format!("name = \"{}\"\n", self.project.name));
        out.push_str(&format!("version = \"{}\"\n", self.project.version));
        out.push_str(&format!("entry_point = \"{}\"\n", self.project.entry_point));
        if !self.project.edition.is_empty() {
            out.push_str(&format!("edition = \"{}\"\n", self.project.edition));
        }
        if let Some(ref v) = self.project.repository {
            out.push_str(&format!("repository = \"{}\"\n", v));
        }
        if let Some(ref v) = self.project.homepage {
            out.push_str(&format!("homepage = \"{}\"\n", v));
        }
        if let Some(ref v) = self.project.license {
            out.push_str(&format!("license = \"{}\"\n", v));
        }
        if !self.project.authors.is_empty() {
            let authors: String = self
                .project
                .authors
                .iter()
                .map(|a| format!("\"{}\"", a))
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("authors = [{}]\n", authors));
        }

        fn write_deps(out: &mut String, deps: &[Dependency], key: &str) {
            if deps.is_empty() {
                return;
            }
            out.push_str(&format!("\n[{}]\n", key));
            for dep in deps {
                match &dep.source {
                    DepSource::Path(p) => {
                        out.push_str(&format!("{} = {{ path = \"{}\"", dep.name, p.display()));
                        if !dep.features.is_empty() {
                            let feats: String = dep
                                .features
                                .iter()
                                .map(|f| format!("\"{}\"", f))
                                .collect::<Vec<_>>()
                                .join(", ");
                            out.push_str(&format!(", features = [{}]", feats));
                        }
                        out.push_str(" }\n");
                    }
                    DepSource::Git {
                        url,
                        tag,
                        branch,
                        rev,
                    } => {
                        out.push_str(&format!("{} = {{ git = \"{}\"", dep.name, url));
                        if let Some(t) = tag {
                            out.push_str(&format!(", tag = \"{}\"", t));
                        }
                        if let Some(b) = branch {
                            out.push_str(&format!(", branch = \"{}\"", b));
                        }
                        if let Some(r) = rev {
                            out.push_str(&format!(", rev = \"{}\"", r));
                        }
                        if !dep.features.is_empty() {
                            let feats: String = dep
                                .features
                                .iter()
                                .map(|f| format!("\"{}\"", f))
                                .collect::<Vec<_>>()
                                .join(", ");
                            out.push_str(&format!(", features = [{}]", feats));
                        }
                        out.push_str(" }\n");
                    }
                    DepSource::Version(v) => {
                        out.push_str(&format!("{} = \"{}\"\n", dep.name, v));
                    }
                }
            }
        }

        write_deps(&mut out, &self.dependencies, "dependencies");
        write_deps(&mut out, &self.dev_dependencies, "dev-dependencies");
        write_deps(&mut out, &self.build_dependencies, "build-dependencies");

        if !self.features.default.is_empty() || !self.features.optional.is_empty() {
            out.push_str("\n[features]\n");
            if !self.features.default.is_empty() {
                let defs: String = self
                    .features
                    .default
                    .iter()
                    .map(|f| format!("\"{}\"", f))
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!("default = [{}]\n", defs));
            }
            for (name, deps) in &self.features.optional {
                let dep_str: String = deps
                    .iter()
                    .map(|d| format!("\"{}\"", d))
                    .collect::<Vec<_>>()
                    .join(", ");
                out.push_str(&format!("{} = [{}]\n", name, dep_str));
            }
        }

        std::fs::write(&self.path, out.as_bytes())
            .map_err(|e| NimError::file_write(&self.path, e.to_string()))
    }

    pub fn add_dependency(&mut self, dep: Dependency) {
        self.dependencies.retain(|d| d.name != dep.name);
        self.dependencies.push(dep);
    }

    pub fn remove_dependency(&mut self, name: &str) -> NimResult<()> {
        let len = self.dependencies.len();
        self.dependencies.retain(|d| d.name != name);
        if self.dependencies.len() == len {
            return Err(NimError::DepNotFound {
                name: name.to_string(),
            });
        }
        Ok(())
    }
}

fn parse_dep_table(val: Option<&toml::Value>, path: &Path) -> NimResult<Vec<Dependency>> {
    let Some(val) = val else { return Ok(vec![]) };
    let table = val
        .as_table()
        .ok_or_else(|| NimError::invalid_manifest(path, "expected a table".to_string()))?;
    let mut deps = vec![];
    for (name, v) in table {
        match v {
            toml::Value::String(s) => {
                let source = if looks_like_git_url(s) {
                    DepSource::Git {
                        url: s.clone(),
                        tag: None,
                        branch: None,
                        rev: None,
                    }
                } else if looks_like_path(s) {
                    DepSource::Path(PathBuf::from(s.clone()))
                } else {
                    DepSource::Version(s.clone())
                };
                deps.push(Dependency {
                    name: name.clone(),
                    source,
                    features: vec![],
                });
            }
            toml::Value::Table(t) => {
                let source = if let Some(git) = t.get("git").and_then(|v| v.as_str()) {
                    DepSource::Git {
                        url: git.to_string(),
                        tag: t.get("tag").and_then(|v| v.as_str()).map(String::from),
                        branch: t.get("branch").and_then(|v| v.as_str()).map(String::from),
                        rev: t.get("rev").and_then(|v| v.as_str()).map(String::from),
                    }
                } else if let Some(p) = t.get("path").and_then(|v| v.as_str()) {
                    DepSource::Path(PathBuf::from(p))
                } else {
                    return Err(NimError::invalid_manifest(
                        path,
                        format!(
                            "dependency `{}` must have `git`, `path`, or be a version string",
                            name
                        ),
                    ));
                };
                let features = t
                    .get("features")
                    .and_then(|v| v.as_array())
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                deps.push(Dependency {
                    name: name.clone(),
                    source,
                    features,
                });
            }
            _ => {
                return Err(NimError::invalid_manifest(
                    path,
                    format!("dependency `{}` must be a string or table", name),
                ));
            }
        }
    }
    Ok(deps)
}

fn parse_features(val: Option<&toml::Value>, _path: &Path) -> FeaturesSection {
    let Some(val) = val else {
        return FeaturesSection {
            default: vec![],
            optional: HashMap::new(),
        };
    };
    let table = match val.as_table() {
        Some(t) => t,
        None => {
            return FeaturesSection {
                default: vec![],
                optional: HashMap::new(),
            };
        }
    };
    let mut optional = HashMap::new();
    let default = table
        .get("default")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    for (k, v) in table {
        if k == "default" {
            continue;
        }
        let deps = v
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        optional.insert(k.clone(), deps);
    }
    FeaturesSection { default, optional }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn parse_minimal_project() {
        let toml = r#"
[project]
name = "test"
version = "0.1.0"
"#;
        let m = ProjectManifest::parse(Path::new("test.toml"), toml).unwrap();
        assert_eq!(m.project.name, "test");
        assert_eq!(m.project.version, "0.1.0");
        assert_eq!(m.project.entry_point, "src/main.nbl");
        assert!(m.dependencies.is_empty());
    }

    #[test]
    fn parse_full_project() {
        let toml = r#"
[project]
name = "myapp"
version = "1.0.0"
entry_point = "src/main.nbl"
edition = "2024"
repository = "https://github.com/user/myapp"
license = "MIT"
authors = ["Alice"]

[dependencies]
json = { git = "https://github.com/user/json", tag = "v1.0.0" }
http = { git = "https://github.com/user/http", branch = "main" }
crypto = { git = "https://github.com/user/crypto", rev = "abc123" }
math = { path = "../math" }

[features]
default = ["std"]
std = []
full = ["json", "http"]
"#;
        let m = ProjectManifest::parse(Path::new("test.toml"), toml).unwrap();
        assert_eq!(m.project.name, "myapp");
        assert_eq!(m.dependencies.len(), 4);
        assert_eq!(m.features.default, vec!["std"]);
        assert!(m.features.optional.contains_key("full"));
    }

    #[test]
    fn parse_invalid_manifest() {
        let err = ProjectManifest::parse(Path::new("test.toml"), "not toml {{").unwrap_err();
        assert!(matches!(err, NimError::InvalidManifest { .. }));
    }

    #[test]
    fn parse_missing_project_section() {
        let err = ProjectManifest::parse(Path::new("test.toml"), "[dependencies]\nx = \"y\"")
            .unwrap_err();
        assert!(matches!(err, NimError::MissingField { .. }));
    }

    #[test]
    fn load_from_file() {
        let dir = std::env::temp_dir().join("nim_test_manifest");
        let _ = std::fs::create_dir_all(&dir);
        let mut f = std::fs::File::create(dir.join("nimble.toml")).unwrap();
        f.write_all(b"[project]\nname = \"foo\"\nversion = \"0.2.1\"\n")
            .unwrap();
        let m = ProjectManifest::load(&dir).unwrap();
        assert_eq!(m.project.name, "foo");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn load_missing_file() {
        let dir = std::env::temp_dir().join("nim_test_missing");
        let _ = std::fs::create_dir_all(&dir);
        assert!(ProjectManifest::load(&dir).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn add_and_remove_dependency() {
        let dir = std::env::temp_dir().join("nim_test_add_remove");
        let _ = std::fs::create_dir_all(&dir);
        let mut m = ProjectManifest::default_for("test", &dir);
        m.save().unwrap();

        m.add_dependency(Dependency {
            name: "json".to_string(),
            source: DepSource::Git {
                url: "https://github.com/user/json".into(),
                tag: Some("v1.0.0".into()),
                branch: None,
                rev: None,
            },
            features: vec![],
        });
        assert_eq!(m.dependencies.len(), 1);
        m.save().unwrap();

        let loaded = ProjectManifest::load(&dir).unwrap();
        assert_eq!(loaded.dependencies.len(), 1);

        m.remove_dependency("json").unwrap();
        assert!(m.dependencies.is_empty());
        m.save().unwrap();

        let loaded2 = ProjectManifest::load(&dir).unwrap();
        assert!(loaded2.dependencies.is_empty());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_version_constraint_dep() {
        let toml = r#"
[project]
name = "test"
version = "0.1.0"

[dependencies]
json = "1.2.0"
"#;
        let m = ProjectManifest::parse(Path::new("test.toml"), toml).unwrap();
        assert_eq!(m.dependencies.len(), 1);
        match &m.dependencies[0].source {
            DepSource::Version(v) => assert_eq!(v, "1.2.0"),
            _ => panic!("expected Version source"),
        }
    }

    #[test]
    fn detect_git_url_vs_path_in_string() {
        assert!(looks_like_git_url("https://github.com/user/repo"));
        assert!(looks_like_git_url("git@github.com:user/repo.git"));
        assert!(looks_like_git_url("http://example.com/repo.git"));
        assert!(!looks_like_git_url("../path/to/dep"));
        assert!(!looks_like_git_url("./local"));
        assert!(!looks_like_git_url("1.2.0"));
        assert!(looks_like_path("../path"));
        assert!(looks_like_path("./local"));
        assert!(looks_like_path("/absolute/path"));
        assert!(!looks_like_path("1.2.0"));
    }

    #[test]
    fn remove_nonexistent_dep_fails() {
        let mut m = ProjectManifest::default_for("test", Path::new("."));
        let err = m.remove_dependency("nonexistent").unwrap_err();
        assert!(matches!(err, NimError::DepNotFound { .. }));
    }
}

fn looks_like_git_url(s: &str) -> bool {
    s.starts_with("https://")
        || s.starts_with("http://")
        || s.starts_with("git@")
        || s.starts_with("git://")
        || s.starts_with("ssh://")
        || s.ends_with(".git")
}

fn looks_like_path(s: &str) -> bool {
    s.starts_with('.') || s.starts_with('/') || s.starts_with('~') || s.starts_with("..")
}

fn parse_profiles(val: Option<&toml::Value>, _path: &Path) -> HashMap<String, Profile> {
    let Some(val) = val else {
        return HashMap::new();
    };
    let table = match val.as_table() {
        Some(t) => t,
        None => return HashMap::new(),
    };
    let mut profiles = HashMap::new();
    for (k, v) in table {
        let t = match v.as_table() {
            Some(t) => t,
            None => continue,
        };
        profiles.insert(
            k.clone(),
            Profile {
                opt_level: t
                    .get("opt-level")
                    .and_then(|v| v.as_integer())
                    .map(|i| i as u32),
                lto: t.get("lto").and_then(|v| v.as_bool()),
                strip: t.get("strip").and_then(|v| v.as_bool()),
                debug: t.get("debug").and_then(|v| v.as_bool()),
                panic: t.get("panic").and_then(|v| v.as_str()).map(String::from),
            },
        );
    }
    profiles
}
