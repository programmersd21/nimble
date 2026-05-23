// anvil - init / build / run command implementations

use std::io::Write;
use std::path::Path;
use std::process::Command;

use crate::config::ProjectManifest;


/// Create a new Nimble project in `dir` with a default layout.
///
/// Creates:
///   nimble.toml
///   src/main.nbl
pub fn init_project(dir: &Path, name: &str) -> Result<(), String> {
    if dir.exists() {
        if dir.join("nimble.toml").exists() {
            return Err(format!(
                "a nimble project already exists at {}",
                dir.display()
            ));
        }
    } else {
        fs_create_dir_all(dir)?;
    }

    // Source directory
    let src_dir = dir.join("src");
    fs_create_dir_all(&src_dir)?;

    // nimble.toml
    let manifest = ProjectManifest::default_for(name);
    let toml_str = format!(
        r#"[project]
name = "{}"
version = "{}"
entry_point = "{}"
"#,
        manifest.project.name, manifest.project.version, manifest.project.entry_point,
    );
    let manifest_path = dir.join("nimble.toml");
    fs_write(&manifest_path, toml_str.as_bytes())?;

    // src/main.nbl
    let main_content = "fn main() -> Int:\n    print(\"hello, world\")\n    return 0\n";
    let main_path = src_dir.join("main.nbl");
    fs_write(&main_path, main_content.as_bytes())?;

    eprintln!(
        "anvil: initialised project `{}` at {}",
        name,
        dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf()).display()
    );
    Ok(())
}


/// Build a Nimble project from its root directory.
///
/// Pipeline:
///   1. Load `nimble.toml`
///   2. Read entry source file
///   3. Invoke `smelt` to compile and link
pub fn build_project(project_dir: &Path, run_after: bool, clean_after: bool) -> Result<(), String> {
    let manifest = ProjectManifest::load(project_dir)?;
    let entry_path = project_dir.join(&manifest.project.entry_point);

    let source = fs_read_to_string(&entry_path)?;

    let output_name = format!("{}.exe", manifest.project.name);
    let output_path = project_dir.join("target").join(&output_name);

    // Ensure target directory exists
    let target_dir = project_dir.join("target");
    fs_create_dir_all(&target_dir)?;

    eprintln!(
        "anvil: building `{}` v{}",
        manifest.project.name, manifest.project.version
    );

    // Write source to a temp file and compile via smelt
    let tmp_dir = std::env::temp_dir().join(format!("anvil_build_{}", std::process::id()));
    fs_create_dir_all(&tmp_dir)?;

    let tmp_src = tmp_dir.join("main.nbl");
    fs_write(&tmp_src, source.as_bytes())?;

    let status = Command::new("smelt")
        .arg(tmp_src.to_str().unwrap())
        .arg("-o")
        .arg(output_path.to_str().unwrap())
        .status()
        .map_err(|e| format!("failed to invoke smelt: {} (is it on PATH?)", e))?;

    if !status.success() {
        return Err("build failed".to_string());
    }

    // Cleanup temp
    let _ = std::fs::remove_dir_all(&tmp_dir);

    eprintln!(
        "anvil: built `{}` → {}",
        manifest.project.name,
        output_path.display()
    );

    if run_after {
        eprintln!("anvil: running `{}`", manifest.project.name);
        let status = Command::new(&output_path)
            .status()
            .map_err(|e| format!("failed to run executable: {}", e))?;
        
        if clean_after {
            let _ = std::fs::remove_file(&output_path);
        }

        if !status.success() {
            if let Some(code) = status.code() {
                std::process::exit(code);
            }
        }
    }

    Ok(())
}


/// Run a previously built project executable.
pub fn run_project(project_dir: &Path) -> Result<(), String> {
    let manifest = ProjectManifest::load(project_dir)?;
    let exe_name = format!("{}.exe", manifest.project.name);
    let exe_path = project_dir.join("target").join(&exe_name);

    if !exe_path.exists() {
        return Err(format!(
            "executable not found at {}. Run `anvil build` first.",
            exe_path.display()
        ));
    }

    eprintln!("anvil: running `{}`", manifest.project.name);

    let status = Command::new(&exe_path)
        .status()
        .map_err(|e| format!("failed to run executable: {}", e))?;

    if !status.success() {
        return Err(format!("executable exited with: {:?}", status.code()));
    }

    Ok(())
}


fn fs_create_dir_all(dir: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dir)
        .map_err(|e| format!("failed to create directory {}: {}", dir.display(), e))
}

fn fs_write(path: &Path, data: &[u8]) -> Result<(), String> {
    let mut f =
        std::fs::File::create(path).map_err(|e| format!("failed to create {}: {}", path.display(), e))?;
    f.write_all(data)
        .map_err(|e| format!("failed to write {}: {}", path.display(), e))
}

fn fs_read_to_string(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| format!("cannot read {}: {}", path.display(), e))
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_project_creates_files() {
        let dir = std::env::temp_dir().join(format!("anvil_test_init_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);

        init_project(&dir, "testapp").unwrap();

        assert!(dir.join("nimble.toml").exists());
        assert!(dir.join("src/main.nbl").exists());

        // Verify manifest is valid
        let manifest = ProjectManifest::load(&dir).unwrap();
        assert_eq!(manifest.project.name, "testapp");
        assert_eq!(manifest.project.version, "0.1.0");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn init_project_twice_fails() {
        let dir = std::env::temp_dir().join(format!("anvil_test_twice_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);

        init_project(&dir, "test").unwrap();
        let err = init_project(&dir, "test2").unwrap_err();
        assert!(err.contains("already exists"));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn build_project_missing_manifest() {
        let dir = std::env::temp_dir().join(format!("anvil_test_build_missing_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let err = build_project(&dir, false, false).unwrap_err();
        assert!(err.contains("cannot read"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
