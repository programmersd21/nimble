use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::ast::Program;
use crate::env::Environment;
use crate::lexer::Token;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub key: String,
    pub fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryNode {
    pub key: String,
    pub fingerprint: String,
    pub dependencies: Vec<DependencyInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheManifest {
    pub nodes: HashMap<String, QueryNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypecheckResult {
    pub env: Environment,
    pub externs: Vec<crate::ast::Stmt>,
    pub module_stmts: Vec<(crate::ast::Stmt, Environment)>,
}

pub struct Database {
    pub cache_dir: PathBuf,
    pub manifest: CacheManifest,

    pub memo_read_file: HashMap<PathBuf, String>,
    pub memo_lex: HashMap<PathBuf, Vec<Token>>,
    pub memo_parse: HashMap<PathBuf, Program>,
    pub memo_typecheck: HashMap<PathBuf, TypecheckResult>,
    pub memo_codegen: HashMap<PathBuf, String>,

    pub query_stack: Vec<String>,
    pub recorded_deps: HashMap<String, Vec<DependencyInfo>>,
}

fn hash_string(s: &str) -> String {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn compute_hash<T: serde::Serialize>(val: &T) -> String {
    let json = serde_json::to_string(val).unwrap_or_default();
    hash_string(&json)
}

impl Database {
    pub fn new(workspace_root: &Path) -> Self {
        let cache_dir = workspace_root.join("target").join(".nimble_cache");
        let manifest_path = cache_dir.join("manifest.json");
        let manifest = if manifest_path.exists() {
            let data = std::fs::read_to_string(&manifest_path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            CacheManifest::default()
        };

        Database {
            cache_dir,
            manifest,
            memo_read_file: HashMap::new(),
            memo_lex: HashMap::new(),
            memo_parse: HashMap::new(),
            memo_typecheck: HashMap::new(),
            memo_codegen: HashMap::new(),
            query_stack: Vec::new(),
            recorded_deps: HashMap::new(),
        }
    }

    pub fn save_manifest(&self) -> Result<(), String> {
        std::fs::create_dir_all(&self.cache_dir)
            .map_err(|e| format!("failed to create cache dir: {}", e))?;
        let manifest_path = self.cache_dir.join("manifest.json");
        let data = serde_json::to_string_pretty(&self.manifest)
            .map_err(|e| format!("failed to serialize manifest: {}", e))?;
        std::fs::write(&manifest_path, data)
            .map_err(|e| format!("failed to write manifest: {}", e))?;
        Ok(())
    }

    fn cache_file_path(&self, key: &str) -> PathBuf {
        let hash_str = hash_string(key);
        self.cache_dir.join(format!("{}.json", hash_str))
    }

    fn load_cached_value<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        let path = self.cache_file_path(key);
        if !path.exists() {
            return None;
        }
        let data = std::fs::read_to_string(&path).ok()?;
        serde_json::from_str(&data).ok()
    }

    fn save_cached_value<T: serde::Serialize>(&self, key: &str, val: &T) -> Result<(), String> {
        std::fs::create_dir_all(&self.cache_dir)
            .map_err(|e| format!("failed to create cache dir: {}", e))?;
        let path = self.cache_file_path(key);
        let data = serde_json::to_string(val)
            .map_err(|e| format!("failed to serialize cache value: {}", e))?;
        std::fs::write(&path, data).map_err(|e| format!("failed to write cache file: {}", e))?;
        Ok(())
    }

    fn record_dependency(&mut self, child_key: &str, child_fingerprint: &str) {
        if let Some(parent) = self.query_stack.last() {
            let deps = self.recorded_deps.entry(parent.clone()).or_default();
            if !deps.iter().any(|d| d.key == child_key) {
                deps.push(DependencyInfo {
                    key: child_key.to_string(),
                    fingerprint: child_fingerprint.to_string(),
                });
            }
        }
    }

    fn start_query(&mut self, key: &str) {
        self.query_stack.push(key.to_string());
    }

    fn end_query(&mut self, key: &str, fingerprint: &str) {
        let popped = self.query_stack.pop();
        assert_eq!(popped.as_deref(), Some(key));

        let deps = self.recorded_deps.remove(key).unwrap_or_default();
        self.manifest.nodes.insert(
            key.to_string(),
            QueryNode {
                key: key.to_string(),
                fingerprint: fingerprint.to_string(),
                dependencies: deps,
            },
        );
    }

    pub fn is_query_valid(&self, key: &str) -> bool {
        let node = match self.manifest.nodes.get(key) {
            Some(n) => n,
            None => return false,
        };

        if key.starts_with("read_file:") {
            let path_str = key.strip_prefix("read_file:").unwrap();
            let path = PathBuf::from(path_str);
            if !path.exists() {
                return false;
            }
            let content = match std::fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => return false,
            };
            let current_fp = hash_string(&content);
            return current_fp == node.fingerprint;
        }

        for dep in &node.dependencies {
            if !self.is_query_valid(&dep.key) {
                return false;
            }
            let dep_node = match self.manifest.nodes.get(&dep.key) {
                Some(dn) => dn,
                None => return false,
            };
            if dep_node.fingerprint != dep.fingerprint {
                return false;
            }
        }

        true
    }

    // --- Static Query System API ---

    pub fn query_read_file(db: Rc<RefCell<Self>>, path: &Path) -> Result<String, String> {
        let key = format!("read_file:{}", path.to_string_lossy());

        let cached = {
            let db_ref = db.borrow();
            db_ref.memo_read_file.get(path).cloned()
        };

        if let Some(content) = cached {
            let fp = hash_string(&content);
            db.borrow_mut().record_dependency(&key, &fp);
            return Ok(content);
        }

        db.borrow_mut().start_query(&key);

        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read file {}: {}", path.display(), e))?;

        let fp = hash_string(&content);

        {
            let mut db_mut = db.borrow_mut();
            db_mut
                .memo_read_file
                .insert(path.to_path_buf(), content.clone());
            let _ = db_mut.save_cached_value(&key, &content);
            db_mut.end_query(&key, &fp);
            db_mut.record_dependency(&key, &fp);
        }

        Ok(content)
    }

    pub fn query_lex(db: Rc<RefCell<Self>>, path: &Path) -> Result<Vec<Token>, String> {
        let key = format!("lex:{}", path.to_string_lossy());

        let cached = {
            let db_ref = db.borrow();
            if db_ref.is_query_valid(&key) {
                if let Some(tokens) = db_ref.memo_lex.get(path) {
                    Some(tokens.clone())
                } else {
                    db_ref.load_cached_value::<Vec<Token>>(&key)
                }
            } else {
                None
            }
        };

        if let Some(tokens) = cached {
            let fp = compute_hash(&tokens);
            {
                let mut db_mut = db.borrow_mut();
                db_mut.memo_lex.insert(path.to_path_buf(), tokens.clone());
                db_mut.record_dependency(&key, &fp);
            }
            return Ok(tokens);
        }

        db.borrow_mut().start_query(&key);

        let content = Self::query_read_file(db.clone(), path)?;

        let mut lexer = crate::lexer::Lexer::new(&content);
        let mut tokens = Vec::new();
        loop {
            let tok = lexer
                .next_token()
                .map_err(|e| format!("{:?}", miette::Report::from(e)))?;
            let is_eof = tok.kind == crate::lexer::TokenKind::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }

        let fp = compute_hash(&tokens);

        {
            let mut db_mut = db.borrow_mut();
            db_mut.memo_lex.insert(path.to_path_buf(), tokens.clone());
            let _ = db_mut.save_cached_value(&key, &tokens);
            db_mut.end_query(&key, &fp);
            db_mut.record_dependency(&key, &fp);
        }

        Ok(tokens)
    }

    pub fn query_parse(db: Rc<RefCell<Self>>, path: &Path) -> Result<Program, String> {
        let key = format!("parse:{}", path.to_string_lossy());

        let cached = {
            let db_ref = db.borrow();
            if db_ref.is_query_valid(&key) {
                if let Some(prog) = db_ref.memo_parse.get(path) {
                    Some(prog.clone())
                } else {
                    db_ref.load_cached_value::<Program>(&key)
                }
            } else {
                None
            }
        };

        if let Some(prog) = cached {
            let fp = compute_hash(&prog);
            {
                let mut db_mut = db.borrow_mut();
                db_mut.memo_parse.insert(path.to_path_buf(), prog.clone());
                db_mut.record_dependency(&key, &fp);
            }
            return Ok(prog);
        }

        db.borrow_mut().start_query(&key);

        let source = Self::query_read_file(db.clone(), path)?;

        let mut parser = crate::parser::Parser::new(&source)
            .map_err(|e| format!("{:?}", miette::Report::from(e)))?;
        let prog = parser
            .parse()
            .map_err(|e| format!("{:?}", miette::Report::from(e)))?;

        let fp = compute_hash(&prog);

        {
            let mut db_mut = db.borrow_mut();
            db_mut.memo_parse.insert(path.to_path_buf(), prog.clone());
            let _ = db_mut.save_cached_value(&key, &prog);
            db_mut.end_query(&key, &fp);
            db_mut.record_dependency(&key, &fp);
        }

        Ok(prog)
    }

    pub fn query_typecheck(db: Rc<RefCell<Self>>, path: &Path) -> Result<TypecheckResult, String> {
        let key = format!("typecheck:{}", path.to_string_lossy());

        let cached = {
            let db_ref = db.borrow();
            if db_ref.is_query_valid(&key) {
                if let Some(res) = db_ref.memo_typecheck.get(path) {
                    Some(res.clone())
                } else {
                    db_ref.load_cached_value::<TypecheckResult>(&key)
                }
            } else {
                None
            }
        };

        if let Some(res) = cached {
            let fp = compute_hash(&res);
            {
                let mut db_mut = db.borrow_mut();
                db_mut
                    .memo_typecheck
                    .insert(path.to_path_buf(), res.clone());
                db_mut.record_dependency(&key, &fp);
            }
            return Ok(res);
        }

        db.borrow_mut().start_query(&key);

        let source = Self::query_read_file(db.clone(), path)?;
        let prog = Self::query_parse(db.clone(), path)?;

        let stdlib_dirs = crate::driver::find_stdlib_dirs();
        let source_dir = path.parent().map(|p| p.to_path_buf());

        let loader =
            crate::module_loader::ModuleLoader::new(stdlib_dirs, source_dir).with_db(db.clone());

        let externs_rc = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let module_fns_rc = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));

        let mut checker = crate::typechecker::TypeChecker::with_externs_and_module_stmts(
            &source,
            externs_rc.clone(),
            module_fns_rc.clone(),
        )
        .with_loader(loader);

        let env = checker
            .check_program(&prog)
            .map_err(|e| format!("{:?}", miette::Report::from(e)))?;

        let res = TypecheckResult {
            env,
            externs: externs_rc.borrow().clone(),
            module_stmts: module_fns_rc.borrow().clone(),
        };

        let fp = compute_hash(&res);

        {
            let mut db_mut = db.borrow_mut();
            db_mut
                .memo_typecheck
                .insert(path.to_path_buf(), res.clone());
            let _ = db_mut.save_cached_value(&key, &res);
            db_mut.end_query(&key, &fp);
            db_mut.record_dependency(&key, &fp);
        }

        Ok(res)
    }

    pub fn query_codegen(db: Rc<RefCell<Self>>, path: &Path) -> Result<String, String> {
        let key = format!("codegen:{}", path.to_string_lossy());

        let cached = {
            let db_ref = db.borrow();
            if db_ref.is_query_valid(&key) {
                if let Some(ir) = db_ref.memo_codegen.get(path) {
                    Some(ir.clone())
                } else {
                    db_ref.load_cached_value::<String>(&key)
                }
            } else {
                None
            }
        };

        if let Some(ir) = cached {
            let fp = hash_string(&ir);
            {
                let mut db_mut = db.borrow_mut();
                db_mut.memo_codegen.insert(path.to_path_buf(), ir.clone());
                db_mut.record_dependency(&key, &fp);
            }
            return Ok(ir);
        }

        db.borrow_mut().start_query(&key);

        let prog = Self::query_parse(db.clone(), path)?;
        let tc_res = Self::query_typecheck(db.clone(), path)?;

        let mut cg = crate::codegen::Codegen::new();
        let externs_rc = std::rc::Rc::new(std::cell::RefCell::new(tc_res.externs));
        let module_fns_rc = std::rc::Rc::new(std::cell::RefCell::new(tc_res.module_stmts));
        cg.generate_with_externs_and_module_fns(&prog, &tc_res.env, &externs_rc, &module_fns_rc)
            .map_err(|e| format!("codegen error: {}", e))?;
        let ir = cg.into_ir();

        let fp = hash_string(&ir);

        {
            let mut db_mut = db.borrow_mut();
            db_mut.memo_codegen.insert(path.to_path_buf(), ir.clone());
            let _ = db_mut.save_cached_value(&key, &ir);
            db_mut.end_query(&key, &fp);
            db_mut.record_dependency(&key, &fp);
        }

        Ok(ir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_query_system_basic() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("main.nbl");

        std::fs::write(&file_path, "fn main() -> Int:\n    return 42\n").unwrap();

        let db = Rc::new(RefCell::new(Database::new(dir.path())));

        // 1. Initial compile
        let ir1 = Database::query_codegen(db.clone(), &file_path).unwrap();
        assert!(ir1.contains("define i64 @main") || ir1.contains("define") || ir1.contains("ret"));

        // Check manifest has the codegen query
        let key = format!("codegen:{}", file_path.to_string_lossy());
        {
            let db_borrow = db.borrow();
            assert!(db_borrow.manifest.nodes.contains_key(&key));
            assert!(db_borrow.is_query_valid(&key));
        }

        // 2. Compile again (should be cached, not re-executing query stack)
        // Clear in-memory caches to force loading from disk cache
        {
            let mut db_borrow = db.borrow_mut();
            db_borrow.memo_codegen.clear();
            db_borrow.memo_typecheck.clear();
            db_borrow.memo_parse.clear();
            db_borrow.memo_lex.clear();
            db_borrow.memo_read_file.clear();
        }

        let ir2 = Database::query_codegen(db.clone(), &file_path).unwrap();
        assert_eq!(ir1, ir2);

        // 3. Modify the source file (should invalidate and rebuild)
        std::fs::write(&file_path, "fn main() -> Int:\n    return 100\n").unwrap();

        // Now codegen is not valid
        {
            let db_borrow = db.borrow();
            assert!(!db_borrow.is_query_valid(&key));
        }

        let ir3 = Database::query_codegen(db.clone(), &file_path).unwrap();
        assert!(ir3.contains("define i64 @main") || ir3.contains("define") || ir3.contains("ret"));
        assert_ne!(ir1, ir3);
    }
}
