use crate::vm::{Value, VM};
use rand::Rng;
use std::sync::Arc;

fn number_from_value(v: &Value) -> Option<(f64, bool)> {
    match v {
        Value::Int(n) => Some((*n as f64, true)),
        Value::Float(f) => Some((*f, false)),
        _ => None,
    }
}

pub fn pow(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("pow expects base and exp".into()));
    }
    let a = number_from_value(&args[0]).map(|v| v.0).unwrap_or(0.0);
    let b = number_from_value(&args[1]).map(|v| v.0).unwrap_or(0.0);
    Value::Float(a.powf(b))
}

pub fn sqrt(_vm: &mut VM, args: Vec<Value>) -> Value {
    let x = number_from_value(args.get(0).unwrap_or(&Value::Int(0)))
        .map(|v| v.0)
        .unwrap_or(0.0);
    if x < 0.0 {
        return Value::Error(Arc::new("sqrt of negative".into()));
    }
    Value::Float(x.sqrt())
}

pub fn abs(_vm: &mut VM, args: Vec<Value>) -> Value {
    match args.get(0) {
        Some(Value::Int(n)) => Value::Int(n.abs()),
        Some(Value::Float(f)) => Value::Float(f.abs()),
        _ => Value::Error(Arc::new("abs expects number".into())),
    }
}

pub fn floor(_vm: &mut VM, args: Vec<Value>) -> Value {
    let x = number_from_value(args.get(0).unwrap_or(&Value::Int(0)))
        .map(|v| v.0)
        .unwrap_or(0.0);
    Value::Float(x.floor())
}

pub fn ceil(_vm: &mut VM, args: Vec<Value>) -> Value {
    let x = number_from_value(args.get(0).unwrap_or(&Value::Int(0)))
        .map(|v| v.0)
        .unwrap_or(0.0);
    Value::Float(x.ceil())
}

pub fn round(_vm: &mut VM, args: Vec<Value>) -> Value {
    let x = number_from_value(args.get(0).unwrap_or(&Value::Int(0)))
        .map(|v| v.0)
        .unwrap_or(0.0);
    Value::Float(x.round())
}

pub fn min(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("min expects two numbers".into()));
    }
    let (a, ai) = match number_from_value(&args[0]) {
        Some(v) => v,
        None => return Value::Error(Arc::new("min expects numbers".into())),
    };
    let (b, bi) = match number_from_value(&args[1]) {
        Some(v) => v,
        None => return Value::Error(Arc::new("min expects numbers".into())),
    };
    if ai && bi {
        Value::Int(a.min(b) as i64)
    } else {
        Value::Float(a.min(b))
    }
}

pub fn max(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("max expects two numbers".into()));
    }
    let (a, ai) = match number_from_value(&args[0]) {
        Some(v) => v,
        None => return Value::Error(Arc::new("max expects numbers".into())),
    };
    let (b, bi) = match number_from_value(&args[1]) {
        Some(v) => v,
        None => return Value::Error(Arc::new("max expects numbers".into())),
    };
    if ai && bi {
        Value::Int(a.max(b) as i64)
    } else {
        Value::Float(a.max(b))
    }
}

pub fn clamp(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 3 {
        return Value::Error(Arc::new("clamp expects x, lo, hi".into()));
    }
    let (x, xi) = match number_from_value(&args[0]) {
        Some(v) => v,
        None => return Value::Error(Arc::new("clamp expects numbers".into())),
    };
    let (lo, _) = match number_from_value(&args[1]) {
        Some(v) => v,
        None => return Value::Error(Arc::new("clamp expects numbers".into())),
    };
    let (hi, _) = match number_from_value(&args[2]) {
        Some(v) => v,
        None => return Value::Error(Arc::new("clamp expects numbers".into())),
    };
    let out = x.max(lo).min(hi);
    if xi {
        Value::Int(out as i64)
    } else {
        Value::Float(out)
    }
}

pub fn log(_vm: &mut VM, args: Vec<Value>) -> Value {
    let x = number_from_value(args.get(0).unwrap_or(&Value::Int(0)))
        .map(|v| v.0)
        .unwrap_or(0.0);
    if x <= 0.0 {
        return Value::Error(Arc::new("log expects positive".into()));
    }
    Value::Float(x.ln())
}

pub fn log2(_vm: &mut VM, args: Vec<Value>) -> Value {
    let x = number_from_value(args.get(0).unwrap_or(&Value::Int(0)))
        .map(|v| v.0)
        .unwrap_or(0.0);
    if x <= 0.0 {
        return Value::Error(Arc::new("log2 expects positive".into()));
    }
    Value::Float(x.log2())
}

pub fn sin(_vm: &mut VM, args: Vec<Value>) -> Value {
    let x = number_from_value(args.get(0).unwrap_or(&Value::Int(0)))
        .map(|v| v.0)
        .unwrap_or(0.0);
    Value::Float(x.sin())
}

pub fn cos(_vm: &mut VM, args: Vec<Value>) -> Value {
    let x = number_from_value(args.get(0).unwrap_or(&Value::Int(0)))
        .map(|v| v.0)
        .unwrap_or(0.0);
    Value::Float(x.cos())
}

pub fn tan(_vm: &mut VM, args: Vec<Value>) -> Value {
    let x = number_from_value(args.get(0).unwrap_or(&Value::Int(0)))
        .map(|v| v.0)
        .unwrap_or(0.0);
    Value::Float(x.tan())
}

pub fn random(_vm: &mut VM, _args: Vec<Value>) -> Value {
    let mut rng = rand::thread_rng();
    Value::Float(rng.gen::<f64>())
}

pub fn rand_int(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("rand_int expects lo and hi".into()));
    }
    let lo = match args.get(0) {
        Some(Value::Int(n)) => *n,
        _ => return Value::Error(Arc::new("lo must be int".into())),
    };
    let hi = match args.get(1) {
        Some(Value::Int(n)) => *n,
        _ => return Value::Error(Arc::new("hi must be int".into())),
    };
    if lo > hi {
        return Value::Error(Arc::new("lo must be <= hi".into()));
    }
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(lo..=hi);
    Value::Int(n)
}

pub fn div(_vm: &mut VM, args: Vec<Value>) -> Value {
    if args.len() < 2 {
        return Value::Error(Arc::new("div expects two numbers".into()));
    }
    let a = match args.get(0) {
        Some(Value::Int(n)) => *n as f64,
        Some(Value::Float(f)) => *f,
        _ => return Value::Error(Arc::new("a must be number".into())),
    };
    let b = match args.get(1) {
        Some(Value::Int(n)) => *n as f64,
        Some(Value::Float(f)) => *f,
        _ => return Value::Error(Arc::new("b must be number".into())),
    };
    if b == 0.0 {
        return Value::Error(Arc::new("division by zero".into()));
    }
    Value::Float(a / b)
}
