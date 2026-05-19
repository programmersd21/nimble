use crate::vm::{Value, VM};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};

pub fn args(vm: &mut VM, _args: Vec<Value>) -> Value {
    let mut list = Vec::new();
    for arg in vm.script_args() {
        list.push(Value::Str(Arc::new(arg.clone())));
    }
    Value::List(Arc::new(Mutex::new(list)))
}

pub fn exit(_vm: &mut VM, args: Vec<Value>) -> Value {
    let code = match args.get(0) {
        Some(Value::Int(n)) => *n as i32,
        _ => 0,
    };
    std::process::exit(code)
}

pub fn env(_vm: &mut VM, _args: Vec<Value>) -> Value {
    let mut map = std::collections::HashMap::new();
    for (k, v) in std::env::vars() {
        map.insert(k, Value::Str(Arc::new(v)));
    }
    Value::Map(Arc::new(Mutex::new(map)))
}

pub fn path_join(_vm: &mut VM, args: Vec<Value>) -> Value {
    let parts = match args.get(0) {
        Some(Value::List(l)) => l.lock().unwrap().clone(),
        _ => return Value::Str(Arc::new(String::new())),
    };
    let mut path = PathBuf::new();
    for part in parts {
        if let Value::Str(s) = part {
            path.push(s.as_str());
        }
    }
    Value::Str(Arc::new(path.to_string_lossy().to_string()))
}

pub fn process_run(_vm: &mut VM, args: Vec<Value>) -> Value {
    let cmd = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("cmd must be string".into())),
    };
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", &cmd]).output()
    } else {
        Command::new("sh").args(["-c", &cmd]).output()
    };
    match output {
        Ok(out) => {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                Value::Str(Arc::new(s))
            } else {
                let s = String::from_utf8_lossy(&out.stderr).trim().to_string();
                Value::Error(Arc::new(s))
            }
        }
        Err(e) => Value::Error(Arc::new(e.to_string())),
    }
}
