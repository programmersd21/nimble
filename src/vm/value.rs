use crate::compiler::bytecode::FunctionChunk;
use crate::vm::VM;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(Arc<String>),
    Null,
    List(Arc<Mutex<Vec<Value>>>),
    Map(Arc<Mutex<HashMap<String, Value>>>),
    Struct {
        class: Arc<String>,
        fields: Arc<Mutex<HashMap<String, Value>>>,
    },
    Class {
        name: Arc<String>,
        fields: Vec<String>,
    },
    Module(Arc<HashMap<String, Value>>),
    Range {
        start: i64,
        end: i64,
    },
    Function(Arc<FunctionChunk>),
    NativeFunction(fn(&mut VM, Vec<Value>) -> Value),
    Error(Arc<String>),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            _ => true,
        }
    }

    pub fn stringify(&self) -> String {
        match self {
            Value::Int(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.to_string(),
            Value::Null => "null".into(),
            Value::List(l) => format!("{:?}", l.lock().unwrap()),
            Value::Map(m) => format!("{:?}", m.lock().unwrap()),
            Value::Struct { class, .. } => format!("<struct {}>", class),
            Value::Class { name, .. } => format!("<class {}>", name),
            Value::Module(_) => "<module>".into(),
            Value::Range { start, end } => format!("{}..{}", start, end),
            Value::Function(c) => format!("<fn {}>", c.name),
            Value::NativeFunction(_) => "<native fn>".into(),
            Value::Error(e) => format!("Error: {}", e),
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::Str(_) => "str",
            Value::Null => "null",
            Value::List(_) => "list",
            Value::Map(_) => "map",
            Value::Struct { .. } => "struct",
            Value::Class { .. } => "class",
            Value::Module(_) => "module",
            Value::Range { .. } => "range",
            Value::Function(_) => "fn",
            Value::NativeFunction(_) => "native fn",
            Value::Error(_) => "error",
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stringify())
    }
}
