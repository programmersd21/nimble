use std::path::Path;
use crate::nim::cache::PackageCache;
use crate::nim::error::{NimError, NimResult};
use crate::nim::git::{GitRef, GitRepo};
use crate::nim::manifest::{DepSource, Dependency, ProjectManifest};
use crate::nim::resolve::Resolver;

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
    let resolver = Resolver::new(&manifest, &repos_dir);
    let lockfile = resolver.resolve()?;
    lockfile.save(project_dir)?;
    eprintln!("  \x1b[1mFinished\x1b[0m {} packages locked", lockfile.packages.len());
    Ok(())
}

pub fn update_deps(project_dir: &Path) -> NimResult<()> {
    let manifest = ProjectManifest::load(project_dir)?;
    let cache = PackageCache::new()?;
    cache.ensure_dirs()?;
    let repos_dir = cache.repos_dir();

    for dep in &manifest.dependencies {
        if let DepSource::Git { url, tag: _, branch: _, rev: _ } = &dep.source {
            let repo = GitRepo::new(url, &repos_dir);
            if repo.source_path().join(".git").exists() {
                eprintln!("  \x1b[34mfetch\x1b[0m {}", url);
                repo.fetch()?;
            } else {
                eprintln!("  \x1b[34mclone\x1b[0m {}", url);
                repo.clone()?;
            }
        }
    }

    let resolver = Resolver::new(&manifest, &repos_dir);
    let lockfile = resolver.resolve()?;
    lockfile.save(project_dir)?;
    eprintln!("  \x1b[1mFinished\x1b[0m {} packages updated", lockfile.packages.len());
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
        Err(_) => return Err(NimError::NotAProject { path: repo.source_path().to_path_buf() }),
    };

    let entry_path = repo.source_path().join(&manifest.project.entry_point);
    if !entry_path.exists() {
        return Err(NimError::EntryPointMissing {
            name: manifest.project.name.clone(),
            path: entry_path,
        });
    }

    eprintln!("  \x1b[32mInstalling\x1b[0m {} @ {} ({})", manifest.project.name, version, &commit[..8]);
    eprintln!("   \x1b[34mCompiling\x1b[0m {}", entry_path.display());

    let bin_dest = cache.bin_path(&manifest.project.name);
    {
        let parent = bin_dest.parent().unwrap();
        std::fs::create_dir_all(parent)
            .map_err(|e| NimError::file_write(parent, e.to_string()))?;
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

    eprintln!("  \x1b[1mFinished\x1b[0m {} installed at {}", manifest.project.name, bin_dest.display());
    Ok(())
}

pub fn uninstall_binary(name: &str) -> NimResult<()> {
    let cache = PackageCache::new()?;
    let path = cache.bin_path(name);
    if !path.exists() {
        return Err(NimError::Other(format!("`{}` is not installed", name)));
    }
    std::fs::remove_file(&path)
        .map_err(|e| NimError::file_write(&path, e.to_string()))?;
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
        crate::nim::copy_dir(repo.source_path(), &pkg_dir)
            .map_err(|e| NimError::cache(e))?;
    }

    eprintln!("  \x1b[32mInstalling\x1b[0m {} @ {} ({})", manifest.project.name, version, &commit[..8]);
    Ok(())
}

pub fn uninstall_pkg_library(uri: &str, version: &str) -> NimResult<()> {
    let cache = PackageCache::new()?;
    let pkg_dir = cache.pkg_cache(uri, version);
    if !pkg_dir.exists() {
        return Err(NimError::Other(format!("`{}@{}` is not cached", uri, version)));
    }
    std::fs::remove_dir_all(&pkg_dir)
        .map_err(|e| NimError::file_write(&pkg_dir, e.to_string()))?;
    eprintln!("  \x1b[31mRemoving\x1b[0m {}@{}", uri, version);
    Ok(())
}
