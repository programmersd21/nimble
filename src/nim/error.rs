use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NimError {
    #[error("cannot read `{path}`: {detail}")]
    FileRead { path: PathBuf, detail: String },

    #[error("cannot write `{path}`: {detail}")]
    FileWrite { path: PathBuf, detail: String },

    #[error("invalid manifest at `{path}`: {detail}")]
    InvalidManifest { path: PathBuf, detail: String },

    #[error("missing required field `{field}` in manifest `{path}`")]
    MissingField { field: String, path: PathBuf },

    #[error("dependency `{name}` not found in manifest")]
    DepNotFound { name: String },

    #[error("git error: {detail}")]
    Git { detail: String },

    #[error("git clone failed for `{url}` @ `{refspec}`: {detail}")]
    GitClone {
        url: String,
        refspec: String,
        detail: String,
    },

    #[error("git fetch failed for `{url}`: {detail}")]
    GitFetch { url: String, detail: String },

    #[error("git ref `{refspec}` not found in `{url}`")]
    GitRefNotFound { url: String, refspec: String },

    #[error("cannot resolve version constraint `{constraint}` for `{name}`")]
    VersionResolve { name: String, constraint: String },

    #[error("dependency cycle detected: {cycle}")]
    CycleDetected { cycle: String },

    #[error("version conflict for `{name}`: requires {required}, but {existing} already resolved")]
    VersionConflict {
        name: String,
        required: String,
        existing: String,
    },

    #[error("cache error: {detail}")]
    Cache { detail: String },

    #[error("home directory not found")]
    NoHomeDir,

    #[error("compilation failed for `{name}`: {detail}")]
    Compile { name: String, detail: String },

    #[error("entry point `{path}` not found for package `{name}`")]
    EntryPointMissing { name: String, path: PathBuf },

    #[error("not a nimble project at `{path}`")]
    NotAProject { path: PathBuf },

    #[error("{0}")]
    Other(String),
}

impl NimError {
    pub fn file_read(path: impl Into<PathBuf>, detail: String) -> Self {
        NimError::FileRead {
            path: path.into(),
            detail,
        }
    }
    pub fn file_write(path: impl Into<PathBuf>, detail: String) -> Self {
        NimError::FileWrite {
            path: path.into(),
            detail,
        }
    }
    pub fn invalid_manifest(path: impl Into<PathBuf>, detail: String) -> Self {
        NimError::InvalidManifest {
            path: path.into(),
            detail,
        }
    }
    pub fn missing_field(field: String, path: impl Into<PathBuf>) -> Self {
        NimError::MissingField {
            field,
            path: path.into(),
        }
    }
    pub fn git(detail: String) -> Self {
        NimError::Git { detail }
    }
    pub fn git_clone(url: String, refspec: String, detail: String) -> Self {
        NimError::GitClone {
            url,
            refspec,
            detail,
        }
    }
    pub fn cache(detail: String) -> Self {
        NimError::Cache { detail }
    }
    pub fn compile(name: String, detail: String) -> Self {
        NimError::Compile { name, detail }
    }
}

pub type NimResult<T> = Result<T, NimError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let e = NimError::file_read("/tmp/foo", "permission denied".to_string());
        let msg = e.to_string();
        assert!(msg.contains("/tmp/foo"));
        assert!(msg.contains("permission denied"));
    }

    #[test]
    fn error_helpers() {
        let e = NimError::git("clone failed".to_string());
        assert!(e.to_string().contains("clone failed"));

        let e = NimError::cache("disk full".to_string());
        assert!(e.to_string().contains("disk full"));

        let e = NimError::compile("myapp".to_string(), "syntax error".to_string());
        assert!(e.to_string().contains("myapp"));

        let e = NimError::git_clone("url".to_string(), "main".to_string(), "timeout".to_string());
        assert!(e.to_string().contains("url"));
    }

    #[test]
    fn error_kind_matching() {
        let e = NimError::NoHomeDir;
        assert!(matches!(e, NimError::NoHomeDir));

        let e = NimError::Other("test".to_string());
        assert!(matches!(e, NimError::Other(_)));
    }
}
