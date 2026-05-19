use crate::vm::{Value, VM};
use std::sync::Arc;

pub fn http_get(_vm: &mut VM, args: Vec<Value>) -> Value {
    let url = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("url must be string".into())),
    };
    match reqwest::blocking::get(url) {
        Ok(resp) => match resp.text() {
            Ok(text) => Value::Str(Arc::new(text)),
            Err(e) => Value::Error(Arc::new(e.to_string())),
        },
        Err(e) => Value::Error(Arc::new(e.to_string())),
    }
}
