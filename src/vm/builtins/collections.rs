use crate::vm::{Value, VM};
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

// ── Map ───────────────────────────────────────────────────────────────────────

pub fn map_keys(_vm: &mut VM, args: Vec<Value>) -> Value {
    let map = match args.get(0) {
        Some(Value::Map(m)) => m.lock().unwrap(),
        _ => return Value::List(Arc::new(Mutex::new(Vec::new()))),
    };
    let mut keys = Vec::new();
    for k in map.keys() {
        keys.push(Value::Str(Arc::new(k.clone())));
    }
    Value::List(Arc::new(Mutex::new(keys)))
}

pub fn map_has(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Bool(false);
    }
    let map = match &args[0] {
        Value::Map(m) => m.lock().unwrap(),
        _ => return Value::Bool(false),
    };
    let key = match &args[1] {
        Value::Str(s) => s.as_str(),
        _ => return Value::Bool(false),
    };
    Value::Bool(map.contains_key(key))
}

pub fn map_values(_vm: &mut VM, args: Vec<Value>) -> Value {
    let map = match args.get(0) {
        Some(Value::Map(m)) => m.lock().unwrap(),
        _ => return Value::List(Arc::new(Mutex::new(Vec::new()))),
    };
    let mut values = Vec::new();
    for v in map.values() {
        values.push(v.clone());
    }
    Value::List(Arc::new(Mutex::new(values)))
}

pub fn map_merge(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("merge expects two maps".into()));
    }
    let base = match args.get(0) {
        Some(Value::Map(m)) => m.lock().unwrap().clone(),
        _ => return Value::Error(Arc::new("first arg must be map".into())),
    };
    let overrides = match args.get(1) {
        Some(Value::Map(m)) => m.lock().unwrap().clone(),
        _ => return Value::Error(Arc::new("second arg must be map".into())),
    };
    let mut merged = base;
    for (k, v) in overrides {
        merged.insert(k, v);
    }
    Value::Map(Arc::new(Mutex::new(merged)))
}

// ── List ──────────────────────────────────────────────────────────────────────

pub fn list_push(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("push expects list and value".into()));
    }
    let list = match args.get(0) {
        Some(Value::List(l)) => l,
        _ => return Value::Error(Arc::new("first arg must be list".into())),
    };
    let value = args.get(1).cloned().unwrap_or(Value::Null);
    list.lock().unwrap().push(value);
    Value::Null
}

pub fn list_pop(_vm: &mut VM, args: Vec<Value>) -> Value {
    let list = match args.get(0) {
        Some(Value::List(l)) => l,
        _ => return Value::Error(Arc::new("first arg must be list".into())),
    };
    let mut borrow = list.lock().unwrap();
    match borrow.pop() {
        Some(v) => v,
        None => Value::Error(Arc::new("pop from empty list".into())),
    }
}

pub fn list_first(_vm: &mut VM, args: Vec<Value>) -> Value {
    let list = match args.get(0) {
        Some(Value::List(l)) => l,
        _ => return Value::Error(Arc::new("first arg must be list".into())),
    };
    let borrow = list.lock().unwrap();
    match borrow.first() {
        Some(v) => v.clone(),
        None => Value::Error(Arc::new("empty list".into())),
    }
}

pub fn list_last(_vm: &mut VM, args: Vec<Value>) -> Value {
    let list = match args.get(0) {
        Some(Value::List(l)) => l,
        _ => return Value::Error(Arc::new("first arg must be list".into())),
    };
    let borrow = list.lock().unwrap();
    match borrow.last() {
        Some(v) => v.clone(),
        None => Value::Error(Arc::new("empty list".into())),
    }
}

pub fn list_insert(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 3 {
        return Value::Error(Arc::new("insert expects list, index, value".into()));
    }
    let list = match args.get(0) {
        Some(Value::List(l)) => l,
        _ => return Value::Error(Arc::new("first arg must be list".into())),
    };
    let idx = match args.get(1) {
        Some(Value::Int(n)) => *n,
        _ => return Value::Error(Arc::new("index must be int".into())),
    };
    let value = args.get(2).cloned().unwrap_or(Value::Null);
    let mut borrow = list.lock().unwrap();
    if idx < 0 || idx as usize > borrow.len() {
        return Value::Error(Arc::new("index out of bounds".into()));
    }
    borrow.insert(idx as usize, value);
    Value::Null
}

pub fn list_remove(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("remove expects list and index".into()));
    }
    let list = match args.get(0) {
        Some(Value::List(l)) => l,
        _ => return Value::Error(Arc::new("first arg must be list".into())),
    };
    let idx = match args.get(1) {
        Some(Value::Int(n)) => *n,
        _ => return Value::Error(Arc::new("index must be int".into())),
    };
    let mut borrow = list.lock().unwrap();
    if idx < 0 || idx as usize >= borrow.len() {
        return Value::Error(Arc::new("index out of bounds".into()));
    }
    borrow.remove(idx as usize)
}

pub fn list_slice(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 3 {
        return Value::Error(Arc::new("slice expects list, start, end".into()));
    }
    let list = match args.get(0) {
        Some(Value::List(l)) => l.lock().unwrap().clone(),
        _ => return Value::Error(Arc::new("first arg must be list".into())),
    };
    let start = match args.get(1) {
        Some(Value::Int(n)) => *n,
        _ => return Value::Error(Arc::new("start must be int".into())),
    };
    let end = match args.get(2) {
        Some(Value::Int(n)) => *n,
        _ => return Value::Error(Arc::new("end must be int".into())),
    };
    let len = list.len() as i64;
    let mut s = if start < 0 { len + start } else { start };
    let mut e = if end < 0 { len + end } else { end };
    s = s.clamp(0, len);
    e = e.clamp(0, len);
    if e < s {
        e = s;
    }
    let mut out = Vec::new();
    for i in s..e {
        out.push(list[i as usize].clone());
    }
    Value::List(Arc::new(Mutex::new(out)))
}

pub fn list_reverse(_vm: &mut VM, args: Vec<Value>) -> Value {
    let list = match args.get(0) {
        Some(Value::List(l)) => l.lock().unwrap().clone(),
        _ => return Value::Error(Arc::new("first arg must be list".into())),
    };
    let mut out = list;
    out.reverse();
    Value::List(Arc::new(Mutex::new(out)))
}

pub fn list_sort(_vm: &mut VM, args: Vec<Value>) -> Value {
    let list = match args.get(0) {
        Some(Value::List(l)) => l.lock().unwrap().clone(),
        _ => return Value::Error(Arc::new("first arg must be list".into())),
    };
    if list.iter().all(|v| matches!(v, Value::Int(_))) {
        let mut out = list;
        out.sort_by(|a, b| match (a, b) {
            (Value::Int(x), Value::Int(y)) => x.cmp(y),
            _ => Ordering::Equal,
        });
        return Value::List(Arc::new(Mutex::new(out)));
    }
    if list
        .iter()
        .all(|v| matches!(v, Value::Int(_) | Value::Float(_)))
    {
        let mut out = list;
        out.sort_by(|a, b| {
            let ax = match a {
                Value::Int(n) => *n as f64,
                Value::Float(f) => *f,
                _ => 0.0,
            };
            let bx = match b {
                Value::Int(n) => *n as f64,
                Value::Float(f) => *f,
                _ => 0.0,
            };
            ax.partial_cmp(&bx).unwrap_or(Ordering::Equal)
        });
        return Value::List(Arc::new(Mutex::new(out)));
    }
    if list.iter().all(|v| matches!(v, Value::Str(_))) {
        let mut out = list;
        out.sort_by(|a, b| match (a, b) {
            (Value::Str(x), Value::Str(y)) => x.cmp(y),
            _ => Ordering::Equal,
        });
        return Value::List(Arc::new(Mutex::new(out)));
    }
    Value::Error(Arc::new(
        "sort supports only int, float, or str lists".into(),
    ))
}

pub fn list_sort_inplace(_vm: &mut VM, args: Vec<Value>) -> Value {
    let list_arc = match args.get(0) {
        Some(Value::List(l)) => l.clone(),
        _ => return Value::Error(Arc::new("sort expects a list".into())),
    };
    let mut list = list_arc.lock().unwrap();
    if list
        .iter()
        .all(|v| matches!(v, Value::Int(_) | Value::Float(_)))
    {
        list.sort_by(|a, b| {
            let ax = match a {
                Value::Int(n) => *n as f64,
                Value::Float(f) => *f,
                _ => 0.0,
            };
            let bx = match b {
                Value::Int(n) => *n as f64,
                Value::Float(f) => *f,
                _ => 0.0,
            };
            ax.partial_cmp(&bx).unwrap_or(Ordering::Equal)
        });
        Value::Null
    } else if list.iter().all(|v| matches!(v, Value::Str(_))) {
        list.sort_by(|a, b| match (a, b) {
            (Value::Str(x), Value::Str(y)) => x.cmp(y),
            _ => Ordering::Equal,
        });
        Value::Null
    } else {
        Value::Error(Arc::new("sort supports int, float, or str lists".into()))
    }
}

pub fn list_reverse_inplace(_vm: &mut VM, args: Vec<Value>) -> Value {
    let list_arc = match args.get(0) {
        Some(Value::List(l)) => l.clone(),
        _ => return Value::Error(Arc::new("reverse expects a list".into())),
    };
    list_arc.lock().unwrap().reverse();
    Value::Null
}

pub fn list_contains(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Bool(false);
    }
    let list = match args.get(0) {
        Some(Value::List(l)) => l.lock().unwrap().clone(),
        _ => return Value::Bool(false),
    };
    let target = match args.get(1) {
        Some(v) => v.stringify(),
        None => return Value::Bool(false),
    };
    for item in &list {
        if item.stringify() == target {
            return Value::Bool(true);
        }
    }
    Value::Bool(false)
}

// ── String ────────────────────────────────────────────────────────────────────

pub fn str_split(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("split expects string and delimiter".into()));
    }
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("first arg must be string".into())),
    };
    let delim = match args.get(1) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("delimiter must be string".into())),
    };
    let mut out = Vec::new();
    if delim.is_empty() {
        for ch in s.chars() {
            out.push(Value::Str(Arc::new(ch.to_string())));
        }
    } else {
        for part in s.split(&delim) {
            out.push(Value::Str(Arc::new(part.to_string())));
        }
    }
    Value::List(Arc::new(Mutex::new(out)))
}

pub fn str_join(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("join expects parts and separator".into()));
    }
    let parts = match args.get(0) {
        Some(Value::List(l)) => l.lock().unwrap().clone(),
        _ => return Value::Error(Arc::new("parts must be list".into())),
    };
    let sep = match args.get(1) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("separator must be string".into())),
    };
    let mut out = String::new();
    for (i, v) in parts.into_iter().enumerate() {
        if i > 0 {
            out.push_str(&sep);
        }
        out.push_str(&v.stringify());
    }
    Value::Str(Arc::new(out))
}

pub fn str_trim(_vm: &mut VM, args: Vec<Value>) -> Value {
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("arg must be string".into())),
    };
    Value::Str(Arc::new(s.trim().to_string()))
}

pub fn str_trim_start(_vm: &mut VM, args: Vec<Value>) -> Value {
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("arg must be string".into())),
    };
    Value::Str(Arc::new(s.trim_start().to_string()))
}

pub fn str_trim_end(_vm: &mut VM, args: Vec<Value>) -> Value {
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("arg must be string".into())),
    };
    Value::Str(Arc::new(s.trim_end().to_string()))
}

pub fn str_upper(_vm: &mut VM, args: Vec<Value>) -> Value {
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("arg must be string".into())),
    };
    Value::Str(Arc::new(s.to_uppercase()))
}

pub fn str_lower(_vm: &mut VM, args: Vec<Value>) -> Value {
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("arg must be string".into())),
    };
    Value::Str(Arc::new(s.to_lowercase()))
}

pub fn str_starts_with(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Bool(false);
    }
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Bool(false),
    };
    let prefix = match args.get(1) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Bool(false),
    };
    Value::Bool(s.starts_with(prefix))
}

pub fn str_ends_with(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Bool(false);
    }
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Bool(false),
    };
    let suffix = match args.get(1) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Bool(false),
    };
    Value::Bool(s.ends_with(suffix))
}

pub fn str_replace(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 3 {
        return Value::Error(Arc::new("replace expects s, old, new".into()));
    }
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("s must be string".into())),
    };
    let old = match args.get(1) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("old must be string".into())),
    };
    let new = match args.get(2) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("new must be string".into())),
    };
    Value::Str(Arc::new(s.replacen(&old, &new, 1)))
}

pub fn str_replace_all(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 3 {
        return Value::Error(Arc::new("replace_all expects s, old, new".into()));
    }
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("s must be string".into())),
    };
    let old = match args.get(1) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("old must be string".into())),
    };
    let new = match args.get(2) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("new must be string".into())),
    };
    Value::Str(Arc::new(s.replace(&old, &new)))
}

pub fn str_count(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Int(0);
    }
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Int(0),
    };
    let sub = match args.get(1) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Int(0),
    };
    if sub.is_empty() {
        return Value::Int(0);
    }
    Value::Int(s.matches(sub).count() as i64)
}

pub fn str_index_of(_vm: &mut VM, args: Vec<Value>) -> Value {
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

pub fn str_slice(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 3 {
        return Value::Error(Arc::new("slice expects s, start, end".into()));
    }
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("s must be string".into())),
    };
    let start = match args.get(1) {
        Some(Value::Int(n)) => *n,
        _ => return Value::Error(Arc::new("start must be int".into())),
    };
    let end = match args.get(2) {
        Some(Value::Int(n)) => *n,
        _ => return Value::Error(Arc::new("end must be int".into())),
    };
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len() as i64;
    let mut s_idx = if start < 0 { len + start } else { start };
    let mut e_idx = if end < 0 { len + end } else { end };
    s_idx = s_idx.clamp(0, len);
    e_idx = e_idx.clamp(0, len);
    if e_idx < s_idx {
        e_idx = s_idx;
    }
    let out: String = chars[s_idx as usize..e_idx as usize].iter().collect();
    Value::Str(Arc::new(out))
}

pub fn str_repeat(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("repeat expects s, n".into()));
    }
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("s must be string".into())),
    };
    let n = match args.get(1) {
        Some(Value::Int(n)) => *n,
        _ => return Value::Error(Arc::new("n must be int".into())),
    };
    if n < 0 {
        return Value::Error(Arc::new("n must be >= 0".into()));
    }
    Value::Str(Arc::new(s.repeat(n as usize)))
}

pub fn str_pad_left(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 3 {
        return Value::Error(Arc::new("pad_left expects s, width, char".into()));
    }
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("s must be string".into())),
    };
    let width = match args.get(1) {
        Some(Value::Int(n)) => *n,
        _ => return Value::Error(Arc::new("width must be int".into())),
    };
    let ch = match args.get(2) {
        Some(Value::Str(s)) => s.chars().next(),
        _ => None,
    };
    let pad = match ch {
        Some(c) => c,
        None => return Value::Error(Arc::new("char must be non-empty string".into())),
    };
    let len = s.chars().count() as i64;
    if width <= len {
        return Value::Str(Arc::new(s));
    }
    let mut out = String::new();
    for _ in 0..(width - len) {
        out.push(pad);
    }
    out.push_str(&s);
    Value::Str(Arc::new(out))
}

pub fn str_pad_right(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 3 {
        return Value::Error(Arc::new("pad_right expects s, width, char".into()));
    }
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Error(Arc::new("s must be string".into())),
    };
    let width = match args.get(1) {
        Some(Value::Int(n)) => *n,
        _ => return Value::Error(Arc::new("width must be int".into())),
    };
    let ch = match args.get(2) {
        Some(Value::Str(s)) => s.chars().next(),
        _ => None,
    };
    let pad = match ch {
        Some(c) => c,
        None => return Value::Error(Arc::new("char must be non-empty string".into())),
    };
    let len = s.chars().count() as i64;
    if width <= len {
        return Value::Str(Arc::new(s));
    }
    let mut out = s.clone();
    for _ in 0..(width - len) {
        out.push(pad);
    }
    Value::Str(Arc::new(out))
}

pub fn str_to_int(_vm: &mut VM, args: Vec<Value>) -> Value {
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("arg must be string".into())),
    };
    match s.parse::<i64>() {
        Ok(n) => Value::Int(n),
        Err(_) => Value::Error(Arc::new("invalid int".into())),
    }
}

pub fn str_to_float(_vm: &mut VM, args: Vec<Value>) -> Value {
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("arg must be string".into())),
    };
    match s.parse::<f64>() {
        Ok(n) => Value::Float(n),
        Err(_) => Value::Error(Arc::new("invalid float".into())),
    }
}

pub fn str_from_int(_vm: &mut VM, args: Vec<Value>) -> Value {
    let n = match args.get(0) {
        Some(Value::Int(n)) => *n,
        _ => return Value::Error(Arc::new("arg must be int".into())),
    };
    Value::Str(Arc::new(n.to_string()))
}

pub fn str_from_float(_vm: &mut VM, args: Vec<Value>) -> Value {
    let n = match args.get(0) {
        Some(Value::Float(n)) => *n,
        Some(Value::Int(n)) => *n as f64,
        _ => return Value::Error(Arc::new("arg must be number".into())),
    };
    Value::Str(Arc::new(n.to_string()))
}

pub fn str_chars(_vm: &mut VM, args: Vec<Value>) -> Value {
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("arg must be string".into())),
    };
    let mut out = Vec::new();
    for ch in s.chars() {
        out.push(Value::Str(Arc::new(ch.to_string())));
    }
    Value::List(Arc::new(Mutex::new(out)))
}

pub fn str_len(_vm: &mut VM, args: Vec<Value>) -> Value {
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Int(0),
    };
    Value::Int(s.chars().count() as i64)
}

pub fn str_is_numeric(_vm: &mut VM, args: Vec<Value>) -> Value {
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Bool(false),
    };
    Value::Bool(!s.is_empty() && s.chars().all(|c| c.is_ascii_digit()))
}

pub fn str_is_alpha(_vm: &mut VM, args: Vec<Value>) -> Value {
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Bool(false),
    };
    Value::Bool(!s.is_empty() && s.chars().all(|c| c.is_ascii_alphabetic()))
}

pub fn str_format(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("format expects template and args".into()));
    }
    let template = match args.get(0) {
        Some(Value::Str(s)) => s.as_str(),
        _ => return Value::Error(Arc::new("template must be string".into())),
    };
    let list = match args.get(1) {
        Some(Value::List(l)) => l.lock().unwrap().clone(),
        _ => return Value::Error(Arc::new("args must be list".into())),
    };
    let mut out = String::new();
    let mut iter = template.chars().peekable();
    let mut arg_index = 0usize;
    while let Some(ch) = iter.next() {
        if ch == '{' {
            if iter.peek() == Some(&'{') {
                iter.next();
                out.push('{');
                continue;
            }
            if iter.peek() == Some(&'}') {
                iter.next();
                if arg_index >= list.len() {
                    return Value::Error(Arc::new("not enough args for format".into()));
                }
                out.push_str(&list[arg_index].stringify());
                arg_index += 1;
                continue;
            }
        }
        if ch == '}' && iter.peek() == Some(&'}') {
            iter.next();
            out.push('}');
            continue;
        }
        out.push(ch);
    }
    Value::Str(Arc::new(out))
}

pub fn str_contains(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Bool(false);
    }
    let s = match args.get(0) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Bool(false),
    };
    let sub = match args.get(1) {
        Some(Value::Str(s)) => s.to_string(),
        _ => return Value::Bool(false),
    };
    Value::Bool(s.contains(&*sub))
}
