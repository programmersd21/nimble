use crate::vm::{Value, VM};
use std::sync::{Arc, Mutex};

pub fn matches(_vm: &mut VM, args: Vec<Value>) -> Value {
    let pattern = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Bool(false),
    };
    let s = match args.get(1) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Bool(false),
    };
    match regex::Regex::new(pattern) {
        Ok(re) => Value::Bool(re.is_match(s)),
        Err(_) => Value::Bool(false),
    }
}

pub fn find(_vm: &mut VM, args: Vec<Value>) -> Value {
    let pattern = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("pattern must be string".into())),
    };
    let s = match args.get(1) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("input must be string".into())),
    };
    let re = match regex::Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => return Value::Error(Arc::new(e.to_string())),
    };
    match re.find(s) {
        Some(m) => Value::Str(Arc::new(m.as_str().to_string())),
        None => Value::Error(Arc::new("no match".into())),
    }
}

pub fn find_all(_vm: &mut VM, args: Vec<Value>) -> Value {
    let pattern = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("pattern must be string".into())),
    };
    let s = match args.get(1) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("input must be string".into())),
    };
    let re = match regex::Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => return Value::Error(Arc::new(e.to_string())),
    };
    let mut out = Vec::new();
    for m in re.find_iter(s) {
        out.push(Value::Str(Arc::new(m.as_str().to_string())));
    }
    Value::List(Arc::new(Mutex::new(out)))
}

pub fn replace(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 3 {
        return Value::Error(Arc::new(
            "replace expects pattern, input, replacement".into(),
        ));
    }
    let pattern = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("pattern must be string".into())),
    };
    let s = match args.get(1) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("input must be string".into())),
    };
    let replacement = match args.get(2) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("replacement must be string".into())),
    };
    let re = match regex::Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => return Value::Error(Arc::new(e.to_string())),
    };
    let out = re.replacen(s, 1, replacement).to_string();
    Value::Str(Arc::new(out))
}

pub fn replace_all(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 3 {
        return Value::Error(Arc::new(
            "replace_all expects pattern, input, replacement".into(),
        ));
    }
    let pattern = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("pattern must be string".into())),
    };
    let s = match args.get(1) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("input must be string".into())),
    };
    let replacement = match args.get(2) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("replacement must be string".into())),
    };
    let re = match regex::Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => return Value::Error(Arc::new(e.to_string())),
    };
    let out = re.replace_all(s, replacement).to_string();
    Value::Str(Arc::new(out))
}

pub fn split(_vm: &mut VM, args: Vec<Value>) -> Value {
    let pattern = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("pattern must be string".into())),
    };
    let s = match args.get(1) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("input must be string".into())),
    };
    let re = match regex::Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => return Value::Error(Arc::new(e.to_string())),
    };
    let mut out = Vec::new();
    for part in re.split(s) {
        out.push(Value::Str(Arc::new(part.to_string())));
    }
    Value::List(Arc::new(Mutex::new(out)))
}
