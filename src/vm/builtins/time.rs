use crate::vm::{Value, VM};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn now(_vm: &mut VM, _args: Vec<Value>) -> Value {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    Value::Int(since_the_epoch.as_millis() as i64)
}

pub fn sleep(_vm: &mut VM, args: Vec<Value>) -> Value {
    let ms = match args.get(0) {
        Some(Value::Int(n)) => *n,
        _ => 0,
    };
    if ms > 0 {
        thread::sleep(Duration::from_millis(ms as u64));
    }
    Value::Null
}
