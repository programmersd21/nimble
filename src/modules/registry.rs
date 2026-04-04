use crate::compiler::bytecode::FunctionChunk;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

pub struct Module {
    pub name: String,
    pub chunk: Arc<FunctionChunk>,
}

pub struct ModuleRegistry {
    pub loaded: HashMap<String, Arc<Module>>,
    pub stdlib_path: PathBuf,
    pub cache_path: PathBuf,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            loaded: HashMap::new(),
            stdlib_path: PathBuf::from("stdlib"),
            cache_path: PathBuf::from(".nimble/cache"),
        }
    }

    pub fn register(&mut self, name: String, module: Arc<Module>) {
        self.loaded.insert(name, module);
    }
}
