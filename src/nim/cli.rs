use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use crate::nim::commands;
use crate::nim::error::NimResult;
use crate::nim::manifest::DepSource;

fn looks_like_git_url(s: &str) -> bool {
    s.starts_with("https://") || s.starts_with("http://") || s.starts_with("git@")
    || s.starts_with("git://") || s.starts_with("ssh://") || s.ends_with(".git")
}

#[derive(Parser)]
#[command(name = "nim", version, about = "Nimble package manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a dependency to the project
    Add {
        name: String,
        #[arg(long)]
        git: Option<String>,
        #[arg(long)]
        path: Option<String>,
        #[arg(long)]
        tag: Option<String>,
        #[arg(long)]
        branch: Option<String>,
        #[arg(long)]
        rev: Option<String>,
    },

    /// Remove a dependency from the project
    Remove { name: String },

    /// Fetch and lock all dependencies
    Fetch {
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Update all dependencies per version constraints
    Update {
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Install a standalone binary globally
    Install {
        #[arg(value_name = "URL@VERSION")]
        target: String,
    },

    /// Uninstall a binary
    Uninstall {
        #[arg(value_name = "NAME")]
        name: String,
    },

    /// Upgrade an installed binary
    Upgrade {
        #[arg(value_name = "URL@VERSION")]
        target: String,
    },

    /// Library package management
    Pkg {
        #[command(subcommand)]
        action: PkgAction,
    },
}

#[derive(Subcommand)]
pub enum PkgAction {
    Install { target: String },
    Uninstall { target: String },
    Upgrade { target: String },
}

impl Cli {
    pub fn run(self) -> NimResult<()> {
        match self.command {
            Commands::Add { name, git, path, tag, branch, rev } => {
                let (dep_name, source) = if let Some(url) = git {
                    (name.clone(), DepSource::Git { url, tag, branch, rev })
                } else if let Some(p) = path {
                    let dep_name = Path::new(&p).file_stem()
                        .and_then(|s| s.to_str()).unwrap_or(&name).to_string();
                    (dep_name, DepSource::Path(PathBuf::from(p)))
                } else if looks_like_git_url(&name) {
                    let dep_name = name.rsplit('/').next()
                        .and_then(|s| s.strip_suffix(".git"))
                        .unwrap_or(&name).to_string();
                    (dep_name, DepSource::Git { url: name.clone(), tag: None, branch: None, rev: None })
                } else {
                    let p = PathBuf::from(&name);
                    if p.exists() {
                        let dep_name = p.file_stem()
                            .and_then(|s| s.to_str()).unwrap_or(&name).to_string();
                        (dep_name, DepSource::Path(p))
                    } else {
                        return Err(crate::nim::error::NimError::Other(
                            "use --git <url> or --path <path> to specify the dependency source".into()
                        ));
                    }
                };
                commands::add_dep(&std::env::current_dir().unwrap(), &dep_name, source)
            }
            Commands::Remove { name } => {
                commands::remove_dep(&std::env::current_dir().unwrap(), &name)
            }
            Commands::Fetch { path } => commands::fetch_deps(&path),
            Commands::Update { path } => commands::update_deps(&path),
            Commands::Install { target } => {
                let (url, version) = split_target(&target)?;
                commands::install_binary(url, version)
            }
            Commands::Uninstall { name } => commands::uninstall_binary(&name),
            Commands::Upgrade { target } => {
                let (url, version) = split_target(&target)?;
                let name = name_from_url(url);
                commands::uninstall_binary(name)?;
                commands::install_binary(url, version)
            }
            Commands::Pkg { action } => {
                match action {
                    PkgAction::Install { target } => {
                        let (url, version) = split_target(&target)?;
                        commands::install_pkg_library(url, version)
                    }
                    PkgAction::Uninstall { target } => {
                        let (url, version) = split_target(&target)?;
                        commands::uninstall_pkg_library(url, version)
                    }
                    PkgAction::Upgrade { target } => {
                        let (url, version) = split_target(&target)?;
                        commands::uninstall_pkg_library(url, version)?;
                        commands::install_pkg_library(url, version)
                    }
                }
            }
        }
    }
}

fn split_target(target: &str) -> NimResult<(&str, &str)> {
    target.rsplit_once('@')
        .ok_or_else(|| crate::nim::error::NimError::Other(
            format!("missing version in `{}` (expected URL@version)", target)
        ))
}

fn name_from_url(url: &str) -> &str {
    let last = url.rsplit('/').next().unwrap_or(url);
    last.strip_suffix(".git").unwrap_or(last)
}
