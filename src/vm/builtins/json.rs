use crate::vm::{Value, VM};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub fn parse(_vm: &mut VM, args: Vec<Value>) -> Value {
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("parse expects string".into())),
    };
    match serde_json::from_str::<serde_json::Value>(s) {
        Ok(v) => json_to_value(v),
        Err(e) => Value::Error(Arc::new(e.to_string())),
    }
}

pub fn stringify(_vm: &mut VM, args: Vec<Value>) -> Value {
    let v = match args.get(0) {
        Some(v) => v,
        None => return Value::Error(Arc::new("stringify expects value".into())),
    };
    match value_to_json(v) {
        Ok(j) => Value::Str(Arc::new(j.to_string())),
        Err(e) => Value::Error(Arc::new(e)),
    }
}

pub fn pretty(_vm: &mut VM, args: Vec<Value>) -> Value {
    let v = match args.get(0) {
        Some(v) => v,
        None => return Value::Error(Arc::new("pretty expects value".into())),
    };
    match value_to_json(v) {
        Ok(j) => match serde_json::to_string_pretty(&j) {
            Ok(s) => Value::Str(Arc::new(s)),
            Err(e) => Value::Error(Arc::new(e.to_string())),
        },
        Err(e) => Value::Error(Arc::new(e)),
    }
}

fn json_to_value(v: serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Int(i)
            } else {
                Value::Float(n.as_f64().unwrap_or(0.0))
            }
        }
        serde_json::Value::String(s) => Value::Str(Arc::new(s)),
        serde_json::Value::Array(a) => {
            let mut out = Vec::new();
            for item in a {
                out.push(json_to_value(item));
            }
            Value::List(Arc::new(Mutex::new(out)))
        }
        serde_json::Value::Object(o) => {
            let mut out = HashMap::new();
            for (k, v) in o {
                out.insert(k, json_to_value(v));
            }
            Value::Map(Arc::new(Mutex::new(out)))
        }
    }
}

fn value_to_json(v: &Value) -> Result<serde_json::Value, String> {
    match v {
        Value::Null => Ok(serde_json::Value::Null),
        Value::Bool(b) => Ok(serde_json::Value::Bool(*b)),
        Value::Int(n) => Ok(serde_json::Value::Number((*n).into())),
        Value::Float(f) => Ok(serde_json::Value::Number(
            serde_json::Number::from_f64(*f).ok_or("invalid float for JSON")?,
        )),
        Value::Str(s) => Ok(serde_json::Value::String(s.to_string())),
        Value::List(l) => {
            let mut out = Vec::new();
            for item in l.lock().unwrap().iter() {
                out.push(value_to_json(item)?);
            }
            Ok(serde_json::Value::Array(out))
        }
        Value::Map(m) => {
            let mut out = serde_json::Map::new();
            for (k, v) in m.lock().unwrap().iter() {
                out.insert(k.clone(), value_to_json(v)?);
            }
            Ok(serde_json::Value::Object(out))
        }
        _ => Err(format!("cannot stringify {} to JSON", v.type_name())),
    }
}
