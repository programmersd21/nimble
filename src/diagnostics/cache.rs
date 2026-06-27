use crate::diagnostics::diagnostic::Diagnostic;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Default)]
pub struct DiagnosticCache {
    emitted_hashes: HashSet<u64>,
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticCache {
    pub fn new() -> Self {
        DiagnosticCache::default()
    }

    fn hash_diagnostic(&self, diag: &Diagnostic) -> u64 {
        let mut s = std::collections::hash_map::DefaultHasher::new();
        diag.severity.hash(&mut s);
        diag.code.hash(&mut s);
        diag.message.hash(&mut s);
        for span in &diag.primary_spans {
            span.file_id.hash(&mut s);
            span.start.hash(&mut s);
            span.end.hash(&mut s);
        }
        s.finish()
    }

    pub fn insert(&mut self, diag: Diagnostic) -> bool {
        let h = self.hash_diagnostic(&diag);
        if self.emitted_hashes.insert(h) {
            self.diagnostics.push(diag);
            true
        } else {
            false
        }
    }

    pub fn get_diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn clear(&mut self) {
        self.emitted_hashes.clear();
        self.diagnostics.clear();
    }
}
