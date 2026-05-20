//! A lightweight object pool to manage heap-allocated memory.
//!
//! This pool reduces allocation overhead for frequent heap objects (Lists/Maps)
//! and provides a path toward garbage collection without full type refactoring.

use crate::vm::Value;
use std::collections::HashMap;

pub enum HeapObj {
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

pub struct ObjectPool {
    pub objects: Vec<HeapObj>,
}

impl ObjectPool {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn alloc(&mut self, obj: HeapObj) -> usize {
        self.objects.push(obj);
        self.objects.len() - 1
    }
}
