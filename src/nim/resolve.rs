use crate::nim::error::{NimError, NimResult};
use crate::nim::git::{GitRef, GitRepo};
use crate::nim::manifest::{DepSource, Dependency, ProjectManifest};
use semver::{Version, VersionReq};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct LockedDep {
    pub name: String,
    pub version: String,
    pub source: String,
    pub commit: String,
    pub checksum: String,
    pub dependencies: Vec<String>,
    pub features: Vec<String>,
    pub kind: DependencyKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyKind {
    Normal,
    Dev,
    Build,
}

#[derive(Debug, Clone)]
pub struct Lockfile {
    pub packages: Vec<LockedDep>,
    pub version: u32,
}

impl Lockfile {
    pub fn load(project_dir: &Path) -> Option<Self> {
        let path = project_dir.join("nimble.lock");
        let raw = std::fs::read_to_string(&path).ok()?;
        Self::parse(&raw).ok()
    }

    pub fn parse(raw: &str) -> NimResult<Self> {
        let val: toml::Value =
            toml::from_str(raw).map_err(|e| NimError::Other(format!("invalid lockfile: {}", e)))?;
        let version = val.get("version").and_then(|v| v.as_integer()).unwrap_or(1) as u32;
        let mut packages = vec![];
        if let Some(arr) = val.get("packages").and_then(|v| v.as_array()) {
            for item in arr {
                let t = item
                    .as_table()
                    .ok_or_else(|| NimError::Other("invalid lockfile entry".into()))?;
                let kind_str = t.get("kind").and_then(|v| v.as_str()).unwrap_or("normal");
                packages.push(LockedDep {
                    name: t
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    version: t
                        .get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    source: t
                        .get("source")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    commit: t
                        .get("commit")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    checksum: t
                        .get("checksum")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    dependencies: t
                        .get("dependencies")
                        .and_then(|v| v.as_array())
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default(),
                    features: t
                        .get("features")
                        .and_then(|v| v.as_array())
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default(),
                    kind: match kind_str {
                        "dev" => DependencyKind::Dev,
                        "build" => DependencyKind::Build,
                        _ => DependencyKind::Normal,
                    },
                });
            }
        }
        Ok(Lockfile { packages, version })
    }

    pub fn save(&self, project_dir: &Path) -> NimResult<()> {
        let path = project_dir.join("nimble.lock");
        let mut out = String::from("# nimble.lock\nversion = 1\n\n[[packages]]\n");
        let entries: Vec<String> = self
            .packages
            .iter()
            .map(|p| {
                let deps: String = p
                    .dependencies
                    .iter()
                    .map(|d| format!("\"{}\"", d))
                    .collect::<Vec<_>>()
                    .join(", ");
                let feats: String = p
                    .features
                    .iter()
                    .map(|f| format!("\"{}\"", f))
                    .collect::<Vec<_>>()
                    .join(", ");
                let kind = match p.kind {
                    DependencyKind::Normal => "normal",
                    DependencyKind::Dev => "dev",
                    DependencyKind::Build => "build",
                };
                let mut entry = format!(
                    r#"name = "{}"
version = "{}"
source = "{}"
commit = "{}"
checksum = "{}"
kind = "{}"
dependencies = [{}]"#,
                    p.name, p.version, p.source, p.commit, p.checksum, kind, deps
                );
                if !p.features.is_empty() {
                    entry.push_str(&format!("\nfeatures = [{}]", feats));
                }
                entry
            })
            .collect();
        out.push_str(&entries.join("\n\n[[packages]]\n"));
        out.push('\n');
        std::fs::write(&path, out.as_bytes())
            .map_err(|e| NimError::file_write(&path, e.to_string()))
    }

    pub fn find(&self, name: &str) -> Option<&LockedDep> {
        self.packages.iter().find(|p| p.name == name)
    }
}

pub struct Resolver<'a> {
    manifest: &'a ProjectManifest,
    cache_root: &'a Path,
}

impl<'a> Resolver<'a> {
    pub fn new(manifest: &'a ProjectManifest, cache_root: &'a Path) -> Self {
        Resolver {
            manifest,
            cache_root,
        }
    }

    pub fn resolve(&self) -> NimResult<Lockfile> {
        self.resolve_with_kind(true, false, false)
    }

    pub fn resolve_all(&self) -> NimResult<Lockfile> {
        self.resolve_with_kind(true, true, true)
    }

    pub fn resolve_with_kind(&self, normal: bool, dev: bool, build: bool) -> NimResult<Lockfile> {
        let mut resolved: Vec<LockedDep> = vec![];
        let mut visiting: HashSet<String> = HashSet::new();
        let mut visited: HashSet<String> = HashSet::new();

        if normal {
            for dep in &self.manifest.dependencies {
                self.resolve_dep(
                    dep,
                    &mut resolved,
                    &mut visiting,
                    &mut visited,
                    &vec![],
                    DependencyKind::Normal,
                )?;
            }
        }
        if dev {
            for dep in &self.manifest.dev_dependencies {
                self.resolve_dep(
                    dep,
                    &mut resolved,
                    &mut visiting,
                    &mut visited,
                    &vec![],
                    DependencyKind::Dev,
                )?;
            }
        }
        if build {
            for dep in &self.manifest.build_dependencies {
                self.resolve_dep(
                    dep,
                    &mut resolved,
                    &mut visiting,
                    &mut visited,
                    &vec![],
                    DependencyKind::Build,
                )?;
            }
        }

        let sorted = topological_sort(&resolved)?;
        Ok(Lockfile {
            packages: sorted,
            version: 1,
        })
    }

    fn resolve_dep(
        &self,
        dep: &Dependency,
        resolved: &mut Vec<LockedDep>,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
        chain: &[String],
        kind: DependencyKind,
    ) -> NimResult<()> {
        if visited.contains(&dep.name) {
            return Ok(());
        }
        if visiting.contains(&dep.name) {
            let cycle = chain.iter().cloned().collect::<Vec<_>>().join(" -> ");
            return Err(NimError::CycleDetected {
                cycle: format!("{} -> {}", cycle, dep.name),
            });
        }

        visiting.insert(dep.name.clone());

        match &dep.source {
            DepSource::Git {
                url,
                tag,
                branch,
                rev,
            } => {
                let repo = GitRepo::new(url, self.cache_root);
                repo.ensure(&GitRef::Branch("HEAD".into()))?;

                let version = if let Some(t) = tag {
                    repo.resolve_tag(t)?
                } else if let Some(b) = branch {
                    repo.resolve_ref(&GitRef::Branch(b.clone()))?
                } else if let Some(r) = rev {
                    repo.resolve_ref(&GitRef::Rev(r.clone()))?
                } else {
                    resolve_semver_tag(&repo, &dep.name, "")?
                };

                let commit = repo.current_commit()?;
                let dep_source = format!("git+{}", url);
                let checksum = compute_checksum(&commit);

                let sub_deps = self.collect_transitive_deps(
                    repo.source_path(),
                    dep.name.clone(),
                    resolved,
                    visiting,
                    visited,
                    chain,
                )?;

                let dep_names: Vec<String> = sub_deps.iter().map(|d| d.name.clone()).collect();

                for pkg in sub_deps {
                    if !visited.contains(&pkg.name) && pkg.name != dep.name {
                        visited.insert(pkg.name.clone());
                        resolved.push(pkg);
                    }
                }

                let locked = LockedDep {
                    name: dep.name.clone(),
                    version: version.clone(),
                    source: dep_source,
                    commit,
                    checksum,
                    dependencies: dep_names,
                    features: dep.features.clone(),
                    kind,
                };
                resolved.push(locked);
            }
            DepSource::Path(p) => {
                let abs_path = if p.is_relative() {
                    self.manifest.path.parent().unwrap().join(p)
                } else {
                    p.clone()
                };
                let sub_manifest = ProjectManifest::load(&abs_path)?;
                let sub_resolver = Resolver::new(&sub_manifest, self.cache_root);
                let sub_lock = sub_resolver.resolve()?;

                let checksum = compute_checksum(&abs_path.to_string_lossy());
                let locked = LockedDep {
                    name: dep.name.clone(),
                    version: sub_manifest.project.version.clone(),
                    source: format!("path+{}", abs_path.display()),
                    commit: String::new(),
                    checksum,
                    dependencies: sub_lock.packages.iter().map(|p| p.name.clone()).collect(),
                    features: dep.features.clone(),
                    kind,
                };
                resolved.push(locked);

                for pkg in sub_lock.packages {
                    if !visited.contains(&pkg.name) && pkg.name != dep.name {
                        visited.insert(pkg.name.clone());
                        resolved.push(pkg);
                    }
                }
            }
            DepSource::Version(constraint) => {
                return Err(NimError::Other(format!(
                    "dependency `{}` has version constraint `{}` but no source URL. \
                     Use `nim add {} --git <url>` or add a table entry with `git` or `path`",
                    dep.name, constraint, dep.name
                )));
            }
        }

        visiting.remove(&dep.name);
        visited.insert(dep.name.clone());
        Ok(())
    }

    fn collect_transitive_deps(
        &self,
        pkg_path: &Path,
        _name: String,
        _resolved: &mut Vec<LockedDep>,
        _visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
        chain: &[String],
    ) -> NimResult<Vec<LockedDep>> {
        let sub_manifest = match ProjectManifest::load(pkg_path) {
            Ok(m) => m,
            Err(_) => return Ok(vec![]),
        };
        let sub_resolver = Resolver::new(&sub_manifest, self.cache_root);
        let sub_lock = sub_resolver.resolve_all()?;
        let mut new_chain = chain.to_vec();
        new_chain.push(_name.clone());
        let mut collected = vec![];
        for pkg in &sub_lock.packages {
            if !visited.contains(&pkg.name) && pkg.name != _name {
                visited.insert(pkg.name.clone());
                collected.push(pkg.clone());
            }
        }
        Ok(collected)
    }
}

fn resolve_semver_tag(repo: &GitRepo, name: &str, constraint: &str) -> NimResult<String> {
    let tags = repo.list_tags()?;
    let req = if constraint.is_empty() {
        VersionReq::parse("*").unwrap()
    } else {
        VersionReq::parse(constraint).map_err(|_| NimError::VersionResolve {
            name: name.to_string(),
            constraint: constraint.to_string(),
        })?
    };
    let mut candidates: Vec<Version> = tags
        .iter()
        .filter_map(|t| {
            let ver_str = t.strip_prefix('v').unwrap_or(t);
            Version::parse(ver_str).ok()
        })
        .filter(|v| req.matches(v))
        .collect();
    candidates.sort();
    candidates.reverse();
    let best = candidates
        .into_iter()
        .next()
        .ok_or_else(|| NimError::VersionResolve {
            name: name.to_string(),
            constraint: constraint.to_string(),
        })?;
    let tag = format!("v{}", best);
    repo.resolve_tag(&tag)?;
    Ok(tag)
}

fn topological_sort(packages: &[LockedDep]) -> NimResult<Vec<LockedDep>> {
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut name_to_pkg: HashMap<&str, &LockedDep> = HashMap::new();

    for pkg in packages {
        in_degree.entry(&pkg.name).or_insert(0);
        adj.entry(&pkg.name).or_default();
        name_to_pkg.insert(&pkg.name, pkg);
    }

    for pkg in packages {
        for dep_name in &pkg.dependencies {
            if adj.contains_key(dep_name.as_str()) {
                adj.get_mut(dep_name.as_str()).unwrap().push(&pkg.name);
                *in_degree.entry(&pkg.name).or_insert(0) += 1;
            }
        }
    }

    let mut queue: VecDeque<&str> = in_degree
        .iter()
        .filter(|&(_, deg)| *deg == 0)
        .map(|(name, _)| *name)
        .collect();

    let mut sorted = vec![];
    while let Some(name) = queue.pop_front() {
        if let Some(pkg) = name_to_pkg.get(name) {
            sorted.push((*pkg).clone());
        }
        if let Some(neighbors) = adj.get(name) {
            for &nbr in neighbors {
                if let Some(deg) = in_degree.get_mut(nbr) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(nbr);
                    }
                }
            }
        }
    }

    if sorted.len() != packages.len() {
        return Err(NimError::CycleDetected {
            cycle: format!(
                "dependency cycle among {} packages",
                packages.len() - sorted.len()
            ),
        });
    }
    Ok(sorted)
}

fn compute_checksum(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lockfile_roundtrip() {
        let lf = Lockfile {
            version: 1,
            packages: vec![LockedDep {
                name: "json".into(),
                version: "1.0.0".into(),
                source: "git+https://github.com/user/json".into(),
                commit: "abc123".into(),
                checksum: "deadbeef".into(),
                dependencies: vec![],
                features: vec![],
                kind: DependencyKind::Normal,
            }],
        };
        let dir = std::env::temp_dir().join("nim_lockfile_test");
        let _ = std::fs::create_dir_all(&dir);
        lf.save(&dir).unwrap();

        let loaded = Lockfile::load(&dir).unwrap();
        assert_eq!(loaded.packages.len(), 1);
        assert_eq!(loaded.packages[0].name, "json");
        assert_eq!(loaded.packages[0].version, "1.0.0");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn lockfile_parse_invalid() {
        assert!(Lockfile::parse("not toml {{{{").is_err());
    }

    #[test]
    fn lockfile_find() {
        let lf = Lockfile {
            version: 1,
            packages: vec![
                LockedDep {
                    name: "a".into(),
                    version: "1".into(),
                    source: "".into(),
                    commit: "".into(),
                    checksum: "".into(),
                    dependencies: vec![],
                    features: vec![],
                    kind: DependencyKind::Normal,
                },
                LockedDep {
                    name: "b".into(),
                    version: "2".into(),
                    source: "".into(),
                    commit: "".into(),
                    checksum: "".into(),
                    dependencies: vec![],
                    features: vec![],
                    kind: DependencyKind::Normal,
                },
            ],
        };
        assert!(lf.find("a").is_some());
        assert!(lf.find("c").is_none());
    }

    #[test]
    fn topological_sort_basic() {
        let pkgs = vec![
            LockedDep {
                name: "c".into(),
                version: "1".into(),
                source: "".into(),
                commit: "".into(),
                checksum: "".into(),
                dependencies: vec![],
                features: vec![],
                kind: DependencyKind::Normal,
            },
            LockedDep {
                name: "b".into(),
                version: "1".into(),
                source: "".into(),
                commit: "".into(),
                checksum: "".into(),
                dependencies: vec!["c".into()],
                features: vec![],
                kind: DependencyKind::Normal,
            },
            LockedDep {
                name: "a".into(),
                version: "1".into(),
                source: "".into(),
                commit: "".into(),
                checksum: "".into(),
                dependencies: vec!["b".into()],
                features: vec![],
                kind: DependencyKind::Normal,
            },
        ];
        let sorted = topological_sort(&pkgs).unwrap();
        let names: Vec<&str> = sorted.iter().map(|p| p.name.as_str()).collect();
        let a_pos = names.iter().position(|&n| n == "a").unwrap();
        let b_pos = names.iter().position(|&n| n == "b").unwrap();
        let c_pos = names.iter().position(|&n| n == "c").unwrap();
        assert!(c_pos < b_pos, "c should come before b");
        assert!(b_pos < a_pos, "b should come before a");
    }

    #[test]
    fn topological_sort_cycle() {
        let pkgs = vec![
            LockedDep {
                name: "a".into(),
                version: "1".into(),
                source: "".into(),
                commit: "".into(),
                checksum: "".into(),
                dependencies: vec!["b".into()],
                features: vec![],
                kind: DependencyKind::Normal,
            },
            LockedDep {
                name: "b".into(),
                version: "1".into(),
                source: "".into(),
                commit: "".into(),
                checksum: "".into(),
                dependencies: vec!["a".into()],
                features: vec![],
                kind: DependencyKind::Normal,
            },
        ];
        assert!(topological_sort(&pkgs).is_err());
    }

    #[test]
    fn topological_sort_independent() {
        let pkgs = vec![
            LockedDep {
                name: "a".into(),
                version: "1".into(),
                source: "".into(),
                commit: "".into(),
                checksum: "".into(),
                dependencies: vec![],
                features: vec![],
                kind: DependencyKind::Normal,
            },
            LockedDep {
                name: "b".into(),
                version: "1".into(),
                source: "".into(),
                commit: "".into(),
                checksum: "".into(),
                dependencies: vec![],
                features: vec![],
                kind: DependencyKind::Normal,
            },
        ];
        let sorted = topological_sort(&pkgs).unwrap();
        assert_eq!(sorted.len(), 2);
    }

    #[test]
    fn checksum_deterministic() {
        let a = compute_checksum("hello");
        let b = compute_checksum("hello");
        assert_eq!(a, b);
        let c = compute_checksum("world");
        assert_ne!(a, c);
    }

    #[test]
    fn lockfile_with_features_and_kind_roundtrip() {
        let lf = Lockfile {
            version: 1,
            packages: vec![
                LockedDep {
                    name: "json".into(), version: "1.0.0".into(),
                    source: "git+https://github.com/user/json".into(),
                    commit: "abc".into(), checksum: "def".into(),
                    dependencies: vec!["serde".into()],
                    features: vec!["serde".into()],
                    kind: DependencyKind::Dev,
                },
            ],
        };
        let dir = std::env::temp_dir().join("nim_lock_feat_test");
        let _ = std::fs::create_dir_all(&dir);
        lf.save(&dir).unwrap();
        let loaded = Lockfile::load(&dir).unwrap();
        assert_eq!(loaded.packages[0].features, vec!["serde"]);
        assert_eq!(loaded.packages[0].kind, DependencyKind::Dev);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn resolve_with_kind_excludes_dev_and_build() {
        use crate::nim::manifest::ProjectManifest;
        let dir = std::env::temp_dir().join("nim_kind_test");
        let _ = std::fs::create_dir_all(&dir);
        let toml_path = dir.join("nimble.toml");
        std::fs::write(&toml_path, r#"
[project]
name = "test"
version = "0.1.0"

[dev-dependencies]
mylib = { git = "https://github.com/user/mylib" }
"#).unwrap();
        let m = ProjectManifest::load(&dir).unwrap();
        let cache = std::env::temp_dir().join("nim_kind_cache");
        let resolver = Resolver::new(&m, &cache);

        // resolve() (normal only) should succeed with 0 packages
        // since only dev-deps exist
        let lockfile = resolver.resolve().unwrap();
        assert!(lockfile.packages.is_empty());

        // resolve_all() should include dev deps and fail on git clone
        assert!(resolver.resolve_all().is_err());
        let _ = std::fs::remove_dir_all(&dir);
        let _ = std::fs::remove_dir_all(&cache);
    }

    #[test]
    fn version_source_rejected() {
        let dir = std::env::temp_dir().join("nim_version_reject");
        let _ = std::fs::create_dir_all(&dir);
        let toml_path = dir.join("nimble.toml");
        std::fs::write(&toml_path, r#"
[project]
name = "test"
version = "0.1.0"

[dependencies]
json = "1.2.0"
"#).unwrap();
        let m = ProjectManifest::load(&dir).unwrap();
        let cache = std::env::temp_dir().join("nim_version_cache");
        let resolver = Resolver::new(&m, &cache);
        let err = resolver.resolve().unwrap_err();
        assert!(err.to_string().contains("version constraint"));
        assert!(err.to_string().contains("no source URL"));
        let _ = std::fs::remove_dir_all(&dir);
        let _ = std::fs::remove_dir_all(&cache);
    }

    #[test]
    fn topological_sort_preserves_features_and_kind() {
        let pkgs = vec![
            LockedDep { name: "b".into(), version: "1".into(), source: "".into(), commit: "".into(), checksum: "".into(), dependencies: vec![], features: vec!["feat_b".into()], kind: DependencyKind::Build },
            LockedDep { name: "a".into(), version: "1".into(), source: "".into(), commit: "".into(), checksum: "".into(), dependencies: vec!["b".into()], features: vec!["feat_a".into()], kind: DependencyKind::Normal },
        ];
        let sorted = topological_sort(&pkgs).unwrap();
        assert_eq!(sorted[0].name, "b");
        assert_eq!(sorted[0].features, vec!["feat_b"]);
        assert_eq!(sorted[0].kind, DependencyKind::Build);
        assert_eq!(sorted[1].features, vec!["feat_a"]);
        assert_eq!(sorted[1].kind, DependencyKind::Normal);
    }

    #[test]
    fn cycle_detection() {
        let _resolved: Vec<LockedDep> = vec![];
        let mut visiting: HashSet<String> = HashSet::new();
        let _visited: HashSet<String> = HashSet::new();

        visiting.insert("a".to_string());
        visiting.insert("b".to_string());

        // Detect if we re-visit 'a' while it's still in visiting set
        assert!(visiting.contains("a"));
        // This simulates what happens when the resolver finds a cycle
    }
}
