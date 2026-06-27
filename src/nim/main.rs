// nim/src/main.rs – CLI router for the Nimble package manager

use crate::nim::manager;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use manager::{PackageManager, ProjectManifest};

#[derive(Parser)]
#[command(
    name    = "nim",
    version,
    about   = "Nimble package manager - install libraries and standalone binaries"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a standalone executable binary from a remote repository.
    ///
    /// Example: nim install github.com/soumalya/kairo@v1.0.5
    Install {
        #[arg(value_name = "URI@VERSION")]
        target: String,
    },

    /// Uninstall a previously installed binary from ~/.nimble/bin/.
    ///
    /// Example: nim uninstall kairo
    Uninstall {
        #[arg(value_name = "NAME")]
        name: String,
    },

    /// Upgrade an installed binary to a new version.
    ///
    /// Example: nim upgrade github.com/soumalya/kairo@v1.1.0
    Upgrade {
        #[arg(value_name = "URI@VERSION")]
        target: String,
    },

    /// Library package sub-commands.
    Pkg {
        #[command(subcommand)]
        action: PkgAction,
    },

    /// Fetch all dependencies declared in the local `nimble.toml`.
    Fetch {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

#[derive(Subcommand)]
enum PkgAction {
    /// Cache a library package globally.
    ///
    /// Example: nim pkg install github.com/soumalya/http-server@v1.2.0
    Install {
        #[arg(value_name = "URI@VERSION")]
        target: String,
    },

    /// Remove a cached library package.
    ///
    /// Example: nim pkg uninstall github.com/soumalya/http-server@v1.2.0
    Uninstall {
        #[arg(value_name = "URI@VERSION")]
        target: String,
    },

    /// Re-clone a cached library package at a new (or same) version.
    ///
    /// Example: nim pkg upgrade github.com/soumalya/http-server@v1.3.0
    Upgrade {
        #[arg(value_name = "URI@VERSION")]
        target: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Install   { target } => cmd_install_binary(target),
        Commands::Uninstall { name }   => PackageManager::new().and_then(|pm| pm.uninstall_binary(name)),
        Commands::Upgrade   { target } => cmd_upgrade_binary(target),
        Commands::Pkg { action } => match action {
            PkgAction::Install   { target } => cmd_install_pkg(target),
            PkgAction::Uninstall { target } => cmd_uninstall_pkg(target),
            PkgAction::Upgrade   { target } => cmd_upgrade_pkg(target),
        },
        Commands::Fetch { path } => cmd_fetch(path),
    };

    if let Err(e) = result {
        eprintln!("nim: error: {}", e);
        std::process::exit(1);
    }
}

// ── Command handlers ──────────────────────────────────────────────────────────

fn cmd_install_binary(target: &str) -> Result<(), String> {
    let (uri, version) = split_target(target)?;
    PackageManager::new()?.install_standalone_binary(uri, version).map(|_| ())
}

fn cmd_upgrade_binary(target: &str) -> Result<(), String> {
    let (uri, version) = split_target(target)?;
    PackageManager::new()?.upgrade_binary(uri, version).map(|_| ())
}

fn cmd_install_pkg(target: &str) -> Result<(), String> {
    let (uri, version) = split_target(target)?;
    PackageManager::new()?.install_pkg_library(uri, version)
}

fn cmd_uninstall_pkg(target: &str) -> Result<(), String> {
    let (uri, version) = split_target(target)?;
    PackageManager::new()?.uninstall_pkg_library(uri, version)
}

fn cmd_upgrade_pkg(target: &str) -> Result<(), String> {
    let (uri, version) = split_target(target)?;
    PackageManager::new()?.upgrade_pkg_library(uri, version)
}

fn cmd_fetch(project_dir: &PathBuf) -> Result<(), String> {
    let manifest = ProjectManifest::load(project_dir)?;
    let pm = PackageManager::new()?;
    let cached = pm.fetch_manifest_deps(&manifest)?;
    if cached.is_empty() {
        println!("    Finished no dependencies declared");
    } else {
        println!("    \x1b[1mFinished\x1b[0m {} package(s) ready", cached.len());
    }
    Ok(())
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn split_target(target: &str) -> Result<(&str, &str), String> {
    target
        .rsplit_once('@')
        .ok_or_else(|| format!("missing version tag in `{}` (expected URI@version)", target))
}
