// nim/src/manager.rs
//
// Core package-manager logic for the Nimble toolchain.
//
// Public entry-points:
//   PackageManager::fetch_manifest_deps        – resolve nimble.toml [dependencies]
//   PackageManager::install_pkg_library        – `nim pkg install <uri>@<ver>`
//   PackageManager::uninstall_pkg_library      – `nim pkg uninstall <uri>@<ver>`
//   PackageManager::upgrade_pkg_library        – `nim pkg upgrade <uri>@<ver>`
//   PackageManager::install_standalone_binary  – `nim install <uri>@<ver>`
//   PackageManager::uninstall_binary           – `nim uninstall <name>`
//   PackageManager::upgrade_binary             – `nim upgrade <uri>@<ver>`

use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

// ── Toolchain directory layout ────────────────────────────────────────────────

/// Resolves the two canonical global storage roots:
///   `~/.nimble/bin/`                              – installed executables
///   `~/.nimble/pkgs/{domain}/{user}/{repo}@{ver}` – cached library sources
pub struct ToolchainPaths {
    pub bin_dir: PathBuf,
    pub pkgs_dir: PathBuf,
}

impl ToolchainPaths {
    pub fn resolve() -> Result<Self, String> {
        let home = dirs::home_dir().ok_or_else(|| "cannot determine home directory".to_string())?;
        let root = home.join(".nimble");
        Ok(Self {
            bin_dir: root.join("bin"),
            pkgs_dir: root.join("pkgs"),
        })
    }

    /// `~/.nimble/pkgs/{domain}/{user}/{repo}@{version}`
    pub fn pkg_path(&self, domain: &str, user: &str, repo: &str, version: &str) -> PathBuf {
        self.pkgs_dir
            .join(domain)
            .join(user)
            .join(format!("{}@{}", repo, version))
    }

    /// `~/.nimble/bin/{name}[.exe]`
    pub fn bin_path(&self, name: &str) -> PathBuf {
        let exe = if cfg!(windows) {
            format!("{}.exe", name)
        } else {
            name.to_string()
        };
        self.bin_dir.join(exe)
    }
}

// ── nimble.toml dependency manifest ──────────────────────────────────────────

/// Represents the `[dependencies]` table in a project's `nimble.toml`.
///
/// ```toml
/// [dependencies]
/// "github.com/soumalya/http-server" = "v1.2.0"
/// ```
#[derive(Debug, Deserialize)]
pub struct ProjectManifest {
    #[serde(default)]
    pub dependencies: std::collections::HashMap<String, String>,
}

impl ProjectManifest {
    pub fn load(project_dir: &Path) -> Result<Self, String> {
        let path = project_dir.join("nimble.toml");
        let raw = std::fs::read_to_string(&path)
            .map_err(|e| format!("cannot read {}: {}", path.display(), e))?;
        toml::from_str(&raw).map_err(|e| format!("invalid nimble.toml: {}", e))
    }
}

// ── URI helpers ───────────────────────────────────────────────────────────────

/// `"github.com/user/repo"` → `("github.com", "user", "repo")`
fn parse_uri(uri: &str) -> Result<(&str, &str, &str), String> {
    let parts: Vec<&str> = uri.splitn(3, '/').collect();
    if parts.len() != 3 {
        return Err(format!(
            "invalid package URI `{}` (expected domain/user/repo)",
            uri
        ));
    }
    Ok((parts[0], parts[1], parts[2]))
}

/// `"github.com/user/repo"` → `"https://github.com/user/repo.git"`
fn uri_to_git_url(uri: &str) -> String {
    format!("https://{}.git", uri)
}

// ── Git clone helper ──────────────────────────────────────────────────────────

fn git_clone(url: &str, refspec: &str, dest: &Path) -> Result<(), String> {
    let status = Command::new("git")
        .args(["clone", "--depth", "1", "--branch", refspec, url])
        .arg(dest)
        .status()
        .map_err(|e| format!("failed to invoke git: {} (is git on PATH?)", e))?;

    if !status.success() {
        return Err(format!("git clone failed for {} @ {}", url, refspec));
    }
    Ok(())
}

// ── Status label helpers ──────────────────────────────────────────────────────

fn log_installing(what: &str) {
    println!("  \x1b[32mInstalling\x1b[0m {}", what);
}

fn log_compiling(what: &str) {
    println!("   \x1b[34mCompiling\x1b[0m {}", what);
}

fn log_removing(what: &str) {
    println!("   \x1b[31mRemoving\x1b[0m {}", what);
}

fn log_upgrading(what: &str) {
    println!("  \x1b[33mUpgrading\x1b[0m {}", what);
}

fn log_finished(what: &str) {
    println!("    \x1b[1mFinished\x1b[0m {}", what);
}

// ── PackageManager ────────────────────────────────────────────────────────────

pub struct PackageManager {
    pub paths: ToolchainPaths,
}

impl PackageManager {
    pub fn new() -> Result<Self, String> {
        let paths = ToolchainPaths::resolve()?;
        std::fs::create_dir_all(&paths.bin_dir)
            .map_err(|e| format!("cannot create bin dir: {}", e))?;
        std::fs::create_dir_all(&paths.pkgs_dir)
            .map_err(|e| format!("cannot create pkgs dir: {}", e))?;
        Ok(Self { paths })
    }

    // ── Requirement A: local manifest deps ───────────────────────────────────

    /// Resolve all `[dependencies]` from a local `nimble.toml`, cloning any
    /// packages not yet present in `~/.nimble/pkgs/`.
    ///
    /// Returns cached source paths ready to be injected into the compiler's
    /// module search path.
    pub fn fetch_manifest_deps(&self, manifest: &ProjectManifest) -> Result<Vec<PathBuf>, String> {
        manifest
            .dependencies
            .iter()
            .map(|(uri, version)| self.ensure_pkg_cached(uri, version))
            .collect()
    }

    // ── Requirement B: global library install ─────────────────────────────────

    /// `nim pkg install github.com/user/repo@v1.2.0`
    pub fn install_pkg_library(&self, uri: &str, version: &str) -> Result<(), String> {
        let dest = self.ensure_pkg_cached(uri, version)?;
        log_finished(&format!(
            "{} @ {} cached at {}",
            uri,
            version,
            dest.display()
        ));
        Ok(())
    }

    // ── Requirement C: standalone binary install ──────────────────────────────

    /// `nim install github.com/user/repo@v1.0.5`
    pub fn install_standalone_binary(&self, uri: &str, version: &str) -> Result<PathBuf, String> {
        let (_, _, repo) = parse_uri(uri)?;
        let url = uri_to_git_url(uri);

        // Step 1 – clone into an isolated temp directory
        let tmp = tempfile::tempdir().map_err(|e| format!("cannot create temp dir: {}", e))?;
        let src_dir = tmp.path().join(repo);

        log_installing(&format!("{} @ {}", uri, version));
        git_clone(&url, version, &src_dir)?;

        // Step 2 – verify this is a Nimble executable project
        let manifest_path = src_dir.join("nimble.toml");
        if !manifest_path.exists() {
            return Err(format!(
                "no nimble.toml found in {} – not a Nimble project",
                src_dir.display()
            ));
        }

        // Step 3 – resolve entry point from manifest, compile via smelt
        let raw = std::fs::read_to_string(&manifest_path)
            .map_err(|e| format!("cannot read nimble.toml: {}", e))?;
        let toml_val: toml::Value =
            toml::from_str(&raw).map_err(|e| format!("invalid nimble.toml: {}", e))?;
        let entry_point = toml_val
            .get("project")
            .and_then(|p| p.get("entry_point"))
            .and_then(|v| v.as_str())
            .unwrap_or("src/main.nbl");

        let source_file = src_dir.join(entry_point);
        let bin_dest = self.paths.bin_path(repo);
        std::fs::create_dir_all(bin_dest.parent().unwrap())
            .map_err(|e| format!("cannot create bin dir: {}", e))?;

        log_compiling(&format!(
            "{} → {}",
            source_file.display(),
            bin_dest.display()
        ));

        let status = Command::new("smelt")
            .arg(&source_file)
            .arg("-o")
            .arg(&bin_dest)
            .status()
            .map_err(|e| format!("failed to invoke smelt: {} (is it on PATH?)", e))?;

        if !status.success() {
            return Err(format!("compilation failed for {}", uri));
        }

        // Step 4 – tmp dir auto-cleaned on drop
        log_finished(&format!("{} installed at {}", repo, bin_dest.display()));
        Ok(bin_dest)
    }

    // ── Uninstall binary ──────────────────────────────────────────────────────

    /// `nim uninstall <name>` – remove a binary from `~/.nimble/bin/`.
    pub fn uninstall_binary(&self, name: &str) -> Result<(), String> {
        let path = self.paths.bin_path(name);
        if !path.exists() {
            return Err(format!("`{}` is not installed", name));
        }
        log_removing(&path.display().to_string());
        std::fs::remove_file(&path)
            .map_err(|e| format!("failed to remove {}: {}", path.display(), e))?;
        log_finished(&format!("{} uninstalled", name));
        Ok(())
    }

    // ── Upgrade binary ────────────────────────────────────────────────────────

    /// `nim upgrade <uri>@<version>` – recompile and replace an installed binary.
    pub fn upgrade_binary(&self, uri: &str, version: &str) -> Result<PathBuf, String> {
        let (_, _, repo) = parse_uri(uri)?;
        log_upgrading(&format!("{} @ {}", uri, version));
        // Remove existing binary if present, then reinstall.
        let bin = self.paths.bin_path(repo);
        if bin.exists() {
            std::fs::remove_file(&bin)
                .map_err(|e| format!("failed to remove old binary: {}", e))?;
        }
        self.install_standalone_binary(uri, version)
    }

    // ── Uninstall pkg library ─────────────────────────────────────────────────

    /// `nim pkg uninstall <uri>@<version>` – remove a cached library package.
    pub fn uninstall_pkg_library(&self, uri: &str, version: &str) -> Result<(), String> {
        let (domain, user, repo) = parse_uri(uri)?;
        let path = self.paths.pkg_path(domain, user, repo, version);
        if !path.exists() {
            return Err(format!("`{}@{}` is not cached", uri, version));
        }
        log_removing(&path.display().to_string());
        std::fs::remove_dir_all(&path)
            .map_err(|e| format!("failed to remove {}: {}", path.display(), e))?;
        log_finished(&format!("{} @ {} removed", uri, version));
        Ok(())
    }

    // ── Upgrade pkg library ───────────────────────────────────────────────────

    /// `nim pkg upgrade <uri>@<version>` – re-clone a cached library package.
    pub fn upgrade_pkg_library(&self, uri: &str, version: &str) -> Result<(), String> {
        log_upgrading(&format!("{} @ {}", uri, version));
        // Drop the old cache entry so ensure_pkg_cached re-clones it.
        let (domain, user, repo) = parse_uri(uri)?;
        let path = self.paths.pkg_path(domain, user, repo, version);
        if path.exists() {
            std::fs::remove_dir_all(&path)
                .map_err(|e| format!("failed to remove old cache: {}", e))?;
        }
        self.install_pkg_library(uri, version)
    }

    // ── Internal ──────────────────────────────────────────────────────────────

    /// Return the cached path for a package, cloning it first if absent.
    fn ensure_pkg_cached(&self, uri: &str, version: &str) -> Result<PathBuf, String> {
        let (domain, user, repo) = parse_uri(uri)?;
        let dest = self.paths.pkg_path(domain, user, repo, version);

        if dest.exists() {
            return Ok(dest);
        }

        let url = uri_to_git_url(uri);
        log_installing(&format!("{} @ {}", uri, version));

        // Clone into a sibling temp path, then rename atomically
        let tmp_dest = dest.with_extension("__tmp");
        if tmp_dest.exists() {
            std::fs::remove_dir_all(&tmp_dest)
                .map_err(|e| format!("cannot clean tmp dir: {}", e))?;
        }
        std::fs::create_dir_all(dest.parent().unwrap())
            .map_err(|e| format!("cannot create pkg parent dir: {}", e))?;

        git_clone(&url, version, &tmp_dest)?;

        std::fs::rename(&tmp_dest, &dest).map_err(|e| format!("atomic move failed: {}", e))?;

        log_finished(&format!("{} @ {} → {}", uri, version, dest.display()));
        Ok(dest)
    }
}
