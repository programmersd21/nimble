use dashmap::DashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct FileId(pub usize);

pub struct SourceFile {
    pub id: FileId,
    pub path: PathBuf,
    pub source: String,
    pub line_starts: Vec<usize>,
}

impl SourceFile {
    pub fn new(id: FileId, path: PathBuf, source: String) -> Self {
        let mut line_starts = vec![0];
        for (offset, ch) in source.char_indices() {
            if ch == '\n' {
                line_starts.push(offset + 1);
            }
        }
        SourceFile {
            id,
            path,
            source,
            line_starts,
        }
    }

    /// Converts a 0-based byte offset to 1-based line and 1-based column.
    pub fn location(&self, byte_index: usize) -> (usize, usize) {
        if byte_index >= self.source.len() {
            if self.line_starts.is_empty() {
                return (1, 1);
            }
            let line = self.line_starts.len();
            let last_start = self.line_starts[line - 1];
            let col = byte_index.saturating_sub(last_start) + 1;
            return (line, col);
        }

        match self.line_starts.binary_search(&byte_index) {
            Ok(line_idx) => (line_idx + 1, 1),
            Err(line_idx) => {
                let line = line_idx;
                let start = self.line_starts[line - 1];
                let col = byte_index - start + 1;
                (line, col)
            }
        }
    }

    /// Gets a slice of a line (0-indexed).
    pub fn get_line(&self, line_idx: usize) -> Option<&str> {
        if line_idx >= self.line_starts.len() {
            return None;
        }
        let start = self.line_starts[line_idx];
        let end = if line_idx + 1 < self.line_starts.len() {
            self.line_starts[line_idx + 1]
        } else {
            self.source.len()
        };
        let line_content = &self.source[start..end];
        Some(line_content.trim_end_matches(|c| c == '\r' || c == '\n'))
    }
}

pub struct SourceMap {
    files: DashMap<FileId, Arc<SourceFile>>,
    paths: DashMap<PathBuf, FileId>,
    next_id: std::sync::atomic::AtomicUsize,
}

impl SourceMap {
    pub fn new() -> Self {
        SourceMap {
            files: DashMap::new(),
            paths: DashMap::new(),
            next_id: std::sync::atomic::AtomicUsize::new(1),
        }
    }

    pub fn insert(&self, path: PathBuf, source: String) -> FileId {
        if let Some(id) = self.paths.get(&path) {
            let id = *id;
            let file = Arc::new(SourceFile::new(id, path.clone(), source));
            self.files.insert(id, file);
            return id;
        }
        let id = FileId(
            self.next_id
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        );
        let file = Arc::new(SourceFile::new(id, path.clone(), source));
        self.files.insert(id, file);
        self.paths.insert(path, id);
        id
    }

    pub fn get(&self, id: FileId) -> Option<Arc<SourceFile>> {
        self.files.get(&id).map(|r| r.value().clone())
    }

    pub fn get_by_path(&self, path: &Path) -> Option<Arc<SourceFile>> {
        let id = self.paths.get(path)?;
        self.get(*id)
    }
}
