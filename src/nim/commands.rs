use crate::nim::cache::PackageCache;
use crate::nim::error::{NimError, NimResult};
use crate::nim::git::{GitRef, GitRepo};
use crate::nim::manifest::{DepSource, Dependency, ProjectManifest};
use crate::nim::resolve::Resolver;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub fn add_dep(project_dir: &Path, name: &str, source: DepSource) -> NimResult<()> {
    let mut manifest = ProjectManifest::load(project_dir)?;
    let features = vec![];
    manifest.add_dependency(Dependency {
        name: name.to_string(),
        source,
        features,
    });
    manifest.save()?;
    eprintln!("  \x1b[32madd\x1b[0m {} added to dependencies", name);
    Ok(())
}

pub fn remove_dep(project_dir: &Path, name: &str) -> NimResult<()> {
    let mut manifest = ProjectManifest::load(project_dir)?;
    manifest.remove_dependency(name)?;
    manifest.save()?;
    eprintln!("  \x1b[31mremove\x1b[0m {} removed from dependencies", name);
    Ok(())
}

pub fn fetch_deps(project_dir: &Path) -> NimResult<()> {
    let manifest = ProjectManifest::load(project_dir)?;
    let cache = PackageCache::new()?;
    cache.ensure_dirs()?;
    let repos_dir = cache.repos_dir();

    let all_deps: Vec<&Dependency> = manifest
        .dependencies
        .iter()
        .chain(manifest.dev_dependencies.iter())
        .chain(manifest.build_dependencies.iter())
        .collect();
    parallel_fetch_git_deps(&all_deps, &repos_dir)?;

    let resolver = Resolver::new(&manifest, &repos_dir);
    let lockfile = resolver.resolve_all()?;
    lockfile.save(project_dir)?;
    eprintln!(
        "  \x1b[1mFinished\x1b[0m {} packages locked",
        lockfile.packages.len()
    );
    Ok(())
}

fn parallel_fetch_git_deps(deps: &[&Dependency], repos_dir: &Path) -> NimResult<()> {
    let errors = Arc::new(Mutex::new(Vec::new()));
    let handles: Vec<_> = deps
        .iter()
        .filter_map(|dep| {
            let url = match &dep.source {
                DepSource::Git { url, .. } => url.clone(),
                _ => return None,
            };
            let repos_dir = repos_dir.to_path_buf();
            let errors = Arc::clone(&errors);
            Some(std::thread::spawn(move || {
                let repo = GitRepo::new(&url, &repos_dir);
                if repo.source_path().join(".git").exists() {
                    if let Err(e) = repo.fetch() {
                        errors.lock().unwrap().push((url, e));
                    }
                } else if let Err(e) = repo.clone_repo() {
                    errors.lock().unwrap().push((url, e));
                }
            }))
        })
        .collect();

    for h in handles {
        let _ = h.join();
    }

    let errs = Arc::into_inner(errors).unwrap().into_inner().unwrap();
    if !errs.is_empty() {
        for (url, e) in &errs {
            eprintln!("  \x1b[31merror\x1b[0m {}: {}", url, e);
        }
        return Err(NimError::Other(format!("{} fetch errors", errs.len())));
    }
    Ok(())
}

pub fn update_deps(project_dir: &Path) -> NimResult<()> {
    let manifest = ProjectManifest::load(project_dir)?;
    let cache = PackageCache::new()?;
    cache.ensure_dirs()?;
    let repos_dir = cache.repos_dir();

    let all_deps: Vec<&Dependency> = manifest
        .dependencies
        .iter()
        .chain(manifest.dev_dependencies.iter())
        .chain(manifest.build_dependencies.iter())
        .collect();
    parallel_fetch_git_deps(&all_deps, &repos_dir)?;

    // Re-resolve semver constraints against latest remote tags
    let resolver = Resolver::new(&manifest, &repos_dir);
    let lockfile = resolver.resolve_all()?;
    lockfile.save(project_dir)?;
    eprintln!(
        "  \x1b[1mFinished\x1b[0m {} packages updated",
        lockfile.packages.len()
    );
    Ok(())
}

pub fn install_binary(url: &str, version: &str) -> NimResult<()> {
    let cache = PackageCache::new()?;
    cache.ensure_dirs()?;

    let repos_dir = cache.repos_dir();
    let repo = GitRepo::new(url, &repos_dir);
    let git_ref = GitRef::Tag(version.to_string());
    repo.ensure(&git_ref)?;
    let commit = repo.current_commit()?;

    let manifest = match ProjectManifest::load(repo.source_path()) {
        Ok(m) => m,
        Err(_) => {
            return Err(NimError::NotAProject {
                path: repo.source_path().to_path_buf(),
            });
        }
    };

    let entry_path = repo.source_path().join(&manifest.project.entry_point);
    if !entry_path.exists() {
        return Err(NimError::EntryPointMissing {
            name: manifest.project.name.clone(),
            path: entry_path,
        });
    }

    eprintln!(
        "  \x1b[32mInstalling\x1b[0m {} @ {} ({})",
        manifest.project.name,
        version,
        &commit[..8]
    );
    eprintln!("   \x1b[34mCompiling\x1b[0m {}", entry_path.display());

    let bin_dest = cache.bin_path(&manifest.project.name);
    {
        let parent = bin_dest.parent().unwrap();
        std::fs::create_dir_all(parent).map_err(|e| NimError::file_write(parent, e.to_string()))?;
    }

    let opts = crate::smelt::driver::CompileOptions {
        output_path: bin_dest.to_string_lossy().to_string(),
        source_path: Some(entry_path.to_string_lossy().to_string()),
        ..Default::default()
    };

    let source = std::fs::read_to_string(&entry_path)
        .map_err(|e| NimError::file_read(&entry_path, e.to_string()))?;
    crate::smelt::driver::compile(&source, &opts)
        .map_err(|e| NimError::compile(manifest.project.name.clone(), e))?;

    eprintln!(
        "  \x1b[1mFinished\x1b[0m {} installed at {}",
        manifest.project.name,
        bin_dest.display()
    );
    Ok(())
}

pub fn uninstall_binary(name: &str) -> NimResult<()> {
    let cache = PackageCache::new()?;
    let path = cache.bin_path(name);
    if !path.exists() {
        return Err(NimError::Other(format!("`{}` is not installed", name)));
    }
    std::fs::remove_file(&path).map_err(|e| NimError::file_write(&path, e.to_string()))?;
    eprintln!("  \x1b[31mRemoving\x1b[0m {}", path.display());
    Ok(())
}

pub fn install_pkg_library(url: &str, version: &str) -> NimResult<()> {
    let cache = PackageCache::new()?;
    cache.ensure_dirs()?;
    let repos_dir = cache.repos_dir();
    let repo = GitRepo::new(url, &repos_dir);
    let git_ref = GitRef::Tag(version.to_string());
    repo.ensure(&git_ref)?;
    let commit = repo.current_commit()?;

    let manifest = ProjectManifest::load(repo.source_path())?;
    let pkg_dir = cache.pkg_cache(&manifest.project.name, version);
    if !pkg_dir.exists() {
        std::fs::create_dir_all(pkg_dir.parent().unwrap())
            .map_err(|e| NimError::cache(format!("cannot create pkg dir: {}", e)))?;
        crate::nim::copy_dir(repo.source_path(), &pkg_dir).map_err(NimError::cache)?;
    }

    eprintln!(
        "  \x1b[32mInstalling\x1b[0m {} @ {} ({})",
        manifest.project.name,
        version,
        &commit[..8]
    );
    Ok(())
}

pub fn uninstall_pkg_library(uri: &str, version: &str) -> NimResult<()> {
    let cache = PackageCache::new()?;
    let pkg_dir = cache.pkg_cache(uri, version);
    if !pkg_dir.exists() {
        return Err(NimError::Other(format!(
            "`{}@{}` is not cached",
            uri, version
        )));
    }
    std::fs::remove_dir_all(&pkg_dir).map_err(|e| NimError::file_write(&pkg_dir, e.to_string()))?;
    eprintln!("  \x1b[31mRemoving\x1b[0m {}@{}", uri, version);
    Ok(())
}
