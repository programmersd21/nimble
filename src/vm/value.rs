//! Runtime value types for the Nimble VM.

use crate::compiler::bytecode::FunctionChunk;
use crate::vm::VM;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// ── Value ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(Arc<String>),
    List(Arc<Mutex<Vec<Value>>>),
    Map(Arc<Mutex<HashMap<String, Value>>>),
    Range {
        start: i64,
        end: i64,
    },
    Error(Arc<String>),
    Function(Arc<FunctionChunk>),
    NativeFunction(NativeFunction),
    Module(Arc<HashMap<String, Value>>),
    FfiLibrary(Arc<String>),
    Class {
        name: Arc<String>,
        fields: Vec<String>,
    },
    Struct(Box<StructData>),
    Iterator {
        items: Arc<Mutex<Vec<Value>>>,
        pos: Arc<Mutex<usize>>,
    },
}

#[derive(Clone, Debug)]
pub struct StructData {
    pub class: Arc<String>,
    pub fields: Arc<Mutex<HashMap<String, Value>>>,
}

pub type NativeFunction = fn(&mut VM, Vec<Value>) -> Value;

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::Error(_) => false,
            _ => true,
        }
    }

    pub fn stringify(&self) -> String {
        match self {
            Value::Null => "null".into(),
            Value::Int(n) => n.to_string(),
            Value::Float(f) => {
                if f.fract() == 0.0 && f.abs() < 1e15 {
                    format!("{:.1}", f)
                } else {
                    f.to_string()
                }
            }
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.to_string(),
            Value::List(l) => {
                let parts: Vec<String> = l.lock().unwrap().iter().map(|v| v.stringify()).collect();
                format!("[{}]", parts.join(", "))
            }
            Value::Map(m) => {
                let parts: Vec<String> = m
                    .lock()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| format!("{k}: {}", v.stringify()))
                    .collect();
                format!("{{{}}}", parts.join(", "))
            }
            Value::Range { start, end } => format!("range({start}, {end})"),
            Value::Error(e) => format!("error({e})"),
            Value::Function(f) => format!("<fn {}>", f.name),
            Value::NativeFunction(_) => "<native fn>".into(),
            Value::Module(_) => "<module>".into(),
            Value::FfiLibrary(path) => format!("<ffi {}>", path),
            Value::Class { name, .. } => format!("<class {name}>"),
            Value::Struct(data) => {
                let parts: Vec<String> = data.fields
                    .lock()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| format!("{k}: {}", v.stringify()))
                    .collect();
                format!("{class}{{{fields}}}", class=data.class, fields=parts.join(", "))
            }
            Value::Iterator { .. } => "<iterator>".into(),
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Null => "null",
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::Str(_) => "str",
            Value::List(_) => "list",
            Value::Map(_) => "map",
            Value::Range { .. } => "range",
            Value::Error(_) => "error",
            Value::Function(_) => "function",
            Value::NativeFunction(_) => "function",
            Value::Module(_) => "module",
            Value::FfiLibrary(_) => "ffi_library",
            Value::Class { .. } => "class",
            Value::Struct(_) => "struct",
            Value::Iterator { .. } => "iterator",
        }
    }

    pub fn downgrade(&self) -> Option<WeakValue> {
        match self {
            Value::Str(s) => Some(WeakValue::Str(Arc::downgrade(s))),
            Value::List(l) => Some(WeakValue::List(Arc::downgrade(l))),
            Value::Map(m) => Some(WeakValue::Map(Arc::downgrade(m))),
            Value::Struct(data) => Some(WeakValue::Struct(Arc::downgrade(&data.fields))),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum WeakValue {
    Str(std::sync::Weak<String>),
    List(std::sync::Weak<Mutex<Vec<Value>>>),
    Map(std::sync::Weak<Mutex<HashMap<String, Value>>>),
    Struct(std::sync::Weak<Mutex<HashMap<String, Value>>>),
}

impl WeakValue {
    pub fn is_alive(&self) -> bool {
        match self {
            WeakValue::Str(w) => w.strong_count() > 0,
            WeakValue::List(w) => w.strong_count() > 0,
            WeakValue::Map(w) => w.strong_count() > 0,
            WeakValue::Struct(w) => w.strong_count() > 0,
        }
    }
}
