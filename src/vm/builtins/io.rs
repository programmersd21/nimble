use crate::vm::{Value, VM};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::sync::{Arc, Mutex};

pub fn file_exists(_vm: &mut VM, args: Vec<Value>) -> Value {
    let path = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Bool(false),
    };
    Value::Bool(fs::metadata(path).is_ok())
}

pub fn read_file(_vm: &mut VM, args: Vec<Value>) -> Value {
    let path = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("path must be string".into())),
    };
    match fs::read_to_string(path) {
        Ok(s) => Value::Str(Arc::new(s)),
        Err(e) => Value::Error(Arc::new(e.to_string())),
    }
}

pub fn write_file(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("write_file expects path and content".into()));
    }
    let path = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("path must be string".into())),
    };
    let content = match args.get(1) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("content must be string".into())),
    };
    match fs::write(path, content) {
        Ok(_) => Value::Null,
        Err(e) => Value::Error(Arc::new(e.to_string())),
    }
}

pub fn append_file(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("append_file expects path and content".into()));
    }
    let path = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("path must be string".into())),
    };
    let content = match args.get(1) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("content must be string".into())),
    };
    match OpenOptions::new().create(true).append(true).open(path) {
        Ok(mut f) => match f.write_all(content.as_bytes()) {
            Ok(_) => Value::Null,
            Err(e) => Value::Error(Arc::new(e.to_string())),
        },
        Err(e) => Value::Error(Arc::new(e.to_string())),
    }
}

pub fn delete_file(_vm: &mut VM, args: Vec<Value>) -> Value {
    let path = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("path must be string".into())),
    };
    match fs::remove_file(path) {
        Ok(_) => Value::Null,
        Err(e) => Value::Error(Arc::new(e.to_string())),
    }
}

pub fn read_lines(_vm: &mut VM, args: Vec<Value>) -> Value {
    let path = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("path must be string".into())),
    };
    match fs::read_to_string(path) {
        Ok(s) => {
            let mut out = Vec::new();
            for line in s.lines() {
                out.push(Value::Str(Arc::new(line.to_string())));
            }
            Value::List(Arc::new(Mutex::new(out)))
        }
        Err(e) => Value::Error(Arc::new(e.to_string())),
    }
}

pub fn write_lines(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("write_lines expects path and lines".into()));
    }
    let path = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("path must be string".into())),
    };
    let list = match args.get(1) {
        Some(Value::List(l)) => l.lock().unwrap().clone(),
        _ => return Value::Error(Arc::new("lines must be list".into())),
    };
    let mut out = String::new();
    for (i, v) in list.into_iter().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        out.push_str(&v.stringify());
    }
    match fs::write(path, out) {
        Ok(_) => Value::Null,
        Err(e) => Value::Error(Arc::new(e.to_string())),
    }
}

pub fn read_bytes(_vm: &mut VM, args: Vec<Value>) -> Value {
    let path = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("path must be string".into())),
    };
    match fs::read(path) {
        Ok(bytes) => {
            let mut out = Vec::with_capacity(bytes.len());
            for b in bytes {
                out.push(Value::Int(b as i64));
            }
            Value::List(Arc::new(Mutex::new(out)))
        }
        Err(e) => Value::Error(Arc::new(e.to_string())),
    }
}

pub fn write_bytes(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("write_bytes expects path and data".into()));
    }
    let path = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("path must be string".into())),
    };
    let list = match args.get(1) {
        Some(Value::List(l)) => l.lock().unwrap().clone(),
        _ => return Value::Error(Arc::new("data must be list".into())),
    };
    let mut bytes = Vec::with_capacity(list.len());
    for v in list {
        match v {
            Value::Int(n) if (0..=255).contains(&n) => bytes.push(n as u8),
            _ => return Value::Error(Arc::new("data must be list of ints 0..255".into())),
        }
    }
    match fs::write(path, bytes) {
        Ok(_) => Value::Null,
        Err(e) => Value::Error(Arc::new(e.to_string())),
    }
}

pub fn copy_file(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("copy_file expects src and dst".into()));
    }
    let src = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("src must be string".into())),
    };
    let dst = match args.get(1) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("dst must be string".into())),
    };
    match fs::copy(src, dst) {
        Ok(_) => Value::Null,
        Err(e) => Value::Error(Arc::new(e.to_string())),
    }
}

pub fn stderr(_vm: &mut VM, args: Vec<Value>) -> Value {
    let mut first = true;
    for arg in args {
        if !first {
            eprint!(" ");
        }
        first = false;
        eprint!("{}", arg.stringify());
    }
    eprintln!();
    Value::Null
}
