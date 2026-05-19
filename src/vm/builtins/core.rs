use crate::vm::{Value, VM};
use std::sync::Arc;
use std::io::{self, Write};

pub fn out(_vm: &mut VM, args: Vec<Value>) -> Value {
    let mut first = true;
    for arg in args {
        if !first {
            print!(" ");
        }
        first = false;
        print!("{}", arg.stringify());
    }
    println!();
    Value::Null
}

pub fn input(_vm: &mut VM, args: Vec<Value>) -> Value {
    if let Some(Value::Str(s)) = args.get(0) {
        print!("{}", s);
        io::stdout().flush().unwrap();
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    Value::Str(Arc::new(input.trim().to_string()))
}

pub fn len(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.is_empty() {
        return Value::Int(0);
    }
    let res = match &args[0] {
        Value::Str(s) => s.chars().count() as i64,
        Value::List(l) => l.lock().unwrap().len() as i64,
        Value::Map(m) => m.lock().unwrap().len() as i64,
        Value::Range { start, end } => (end - start).max(0),
        _ => 0,
    };
    Value::Int(res)
}

pub fn to_int(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.is_empty() {
        return Value::Int(0);
    }
    match &args[0] {
        Value::Int(n) => Value::Int(*n),
        Value::Float(f) => Value::Int(*f as i64),
        Value::Bool(b) => Value::Int(if *b { 1 } else { 0 }),
        Value::Str(s) => Value::Int(s.parse().unwrap_or(0)),
        _ => Value::Int(0),
    }
}

pub fn index_of(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Int(-1);
    }
    let s = match &args[0] {
        Value::Str(s) => s.as_str(),
        _ => return Value::Int(-1),
    };
    let sub = match &args[1] {
        Value::Str(s) => s.as_str(),
        _ => return Value::Int(-1),
    };
    if let Some(idx) = s.find(sub) {
        Value::Int(idx as i64)
    } else {
        Value::Int(-1)
    }
}

pub fn error(_vm: &mut VM, args: Vec<Value>) -> Value {
    let msg = if let Some(v) = args.get(0) {
        v.stringify()
    } else {
        "error".into()
    };
    Value::Error(Arc::new(msg))
}

pub fn load_module(vm: &mut VM, args: Vec<Value>) -> Value {
    let source = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("source must be string".into())),
    };
    match vm.load_module(&source) {
        Ok(m) => m,
        Err(e) => Value::Error(Arc::new(e)),
    }
}
