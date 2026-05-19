use crate::compiler::bytecode::{CallArgDesc, FunctionChunk, Instr, Reg};
use crate::modules::resolver::ModuleResolver;
use crate::vm::frame::CallFrame;
use crate::vm::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

type RuntimeCallArg = (Option<String>, Value);

pub struct VM {
    frames: Vec<CallFrame>,
    pub globals: HashMap<String, Value>,
    module_cache: Arc<Mutex<HashMap<String, Value>>>,
    resolver: ModuleResolver,
    script_args: Vec<String>,
}

impl VM {
    pub fn new() -> Self {
        let mut vm = Self {
            frames: Vec::new(),
            globals: HashMap::new(),
            module_cache: Arc::new(Mutex::new(HashMap::new())),
            resolver: ModuleResolver::new(),
            script_args: Vec::new(),
        };
        vm.install_builtins();
        vm
    }

    fn with_shared_cache(cache: Arc<Mutex<HashMap<String, Value>>>) -> Self {
        let mut vm = Self {
            frames: Vec::new(),
            globals: HashMap::new(),
            module_cache: cache,
            resolver: ModuleResolver::new(),
            script_args: Vec::new(),
        };
        vm.install_builtins();
        vm
    }

    fn install_builtins(&mut self) {
        crate::vm::builtins::install(&mut self.globals);
    }

    // ── Public run API ────────────────────────────────────────────────────────

    pub fn run(&mut self, chunk: Arc<FunctionChunk>) -> Result<Value, String> {
        let dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        self.run_with_dir(chunk, dir)
    }

    pub fn run_with_dir(
        &mut self,
        chunk: Arc<FunctionChunk>,
        module_dir: PathBuf,
    ) -> Result<Value, String> {
        self.frames.push(CallFrame::new(chunk, module_dir, None));
        self.execute()
    }

    pub fn set_script_args(&mut self, args: Vec<String>) {
        self.script_args = args;
    }

    pub fn script_args(&self) -> &[String] {
        &self.script_args
    }

    pub fn current_module_dir_path(&self) -> PathBuf {
        self.current_module_dir()
    }

    pub fn load_module(&mut self, source: &str) -> Result<Value, String> {
        if let Some(val) = self.module_cache.lock().unwrap().get(source).cloned() {
            return Ok(val);
        }
        let base_dir = self.current_module_dir();
        let (chunk, path) = self.resolver.resolve(source, &base_dir)?;
        let module_dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        let exports = chunk.exports.clone();

        let mut module_vm = VM::with_shared_cache(self.module_cache.clone());
        module_vm.script_args = self.script_args.clone();
        module_vm.run_with_dir(chunk, module_dir)?;

        let mut out = HashMap::new();
        for name in exports {
            if let Some(v) = module_vm.globals.get(&name).cloned() {
                out.insert(name, v);
            }
        }
        let module_val = Value::Module(Arc::new(out));
        self.module_cache
            .lock()
            .unwrap()
            .insert(source.to_string(), module_val.clone());
        Ok(module_val)
    }

    pub fn global_entries(&self) -> Vec<(String, Value)> {
        let mut entries: Vec<(String, Value)> = self
            .globals
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        entries
    }

    fn current_module_dir(&self) -> PathBuf {
        self.frames
            .last()
            .map(|f| f.module_dir.clone())
            .unwrap_or_else(|| PathBuf::from("."))
    }

    fn collect_call_args(frame: &CallFrame, args: &[CallArgDesc]) -> Vec<RuntimeCallArg> {
        args.iter()
            .map(|arg| {
                let name = arg
                    .name
                    .map(|idx| frame.chunk.names[idx.0 as usize].clone());
                let value = frame.get_reg(arg.reg);
                (name, value)
            })
            .collect()
    }

    fn bind_named_args(names: &[String], args: Vec<RuntimeCallArg>) -> Result<Vec<Value>, String> {
        let mut bound = vec![Value::Null; names.len()];
        let mut assigned = vec![false; names.len()];
        let mut next_pos = 0usize;

        for (name, value) in args {
            let idx = if let Some(name) = name {
                names
                    .iter()
                    .position(|candidate| candidate == &name)
                    .ok_or_else(|| format!("Unknown named argument '{name}'"))?
            } else {
                while next_pos < assigned.len() && assigned[next_pos] {
                    next_pos += 1;
                }
                if next_pos >= names.len() {
                    return Err("Too many arguments".into());
                }
                let idx = next_pos;
                next_pos += 1;
                idx
            };

            if assigned[idx] {
                return Err(format!("Duplicate argument '{}'", names[idx]));
            }

            assigned[idx] = true;
            bound[idx] = value;
        }

        Ok(bound)
    }

    fn iterator_items(val: Value, step: Option<Value>) -> Result<Vec<Value>, String> {
        match val {
            Value::List(list) => {
                let items = list.lock().unwrap().clone();
                let step = match step {
                    None => 1usize,
                    Some(Value::Int(n)) if n > 0 => n as usize,
                    Some(Value::Int(_)) => return Err("List iteration step must be > 0".into()),
                    Some(_) => return Err("List iteration step must be an int".into()),
                };
                Ok(items.into_iter().step_by(step).collect())
            }
            Value::Range { start, end } => {
                let step = match step {
                    None => 1,
                    Some(Value::Int(0)) => return Err("Range iteration step cannot be 0".into()),
                    Some(Value::Int(n)) => n,
                    Some(_) => return Err("Range iteration step must be an int".into()),
                };

                let mut items = Vec::new();
                if step > 0 {
                    let mut current = start;
                    while current < end {
                        items.push(Value::Int(current));
                        current += step;
                    }
                } else {
                    let mut current = start;
                    while current > end {
                        items.push(Value::Int(current));
                        current += step;
                    }
                }
                Ok(items)
            }
            Value::Str(s) => {
                let chars: Vec<Value> = s
                    .chars()
                    .map(|c| Value::Str(Arc::new(c.to_string())))
                    .collect();
                let step = match step {
                    None => 1usize,
                    Some(Value::Int(n)) if n > 0 => n as usize,
                    Some(Value::Int(_)) => return Err("String iteration step must be > 0".into()),
                    Some(_) => return Err("String iteration step must be an int".into()),
                };
                Ok(chars.into_iter().step_by(step).collect())
            }
            other => Err(format!("Cannot iterate over {}", other.type_name())),
        }
    }

    // ── Execute loop ──────────────────────────────────────────────────────────

    fn execute(&mut self) -> Result<Value, String> {
        loop {
            let mut frame = match self.frames.pop() {
                Some(f) => f,
                None => return Ok(Value::Null),
            };

            if frame.ip >= frame.chunk.instrs.len() {
                if let Some(ret) = self.return_from_frame(Value::Null, frame.return_reg) {
                    return Ok(ret);
                }
                continue;
            }

            let instr = frame.chunk.instrs[frame.ip].clone();
            frame.ip += 1;
            self.frames.push(frame);

            match instr {
                Instr::LoadConst { dst, idx } => {
                    let frame = self.frames.last_mut().unwrap();
                    let val = frame.chunk.constants[idx.0 as usize].clone();
                    frame.set_reg(dst, val);
                }
                Instr::Move { dst, src } => {
                    let frame = self.frames.last_mut().unwrap();
                    let val = frame.get_reg(src);
                    frame.set_reg(dst, val);
                }
                Instr::AddInt { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
                        (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
                        (Value::Int(x), Value::Float(y)) => Value::Float(x as f64 + y),
                        (Value::Float(x), Value::Int(y)) => Value::Float(x + y as f64),
                        (Value::Str(sa), Value::Str(sb)) => {
                            let s = sa.to_string() + &sb;
                            Value::Str(Arc::new(s))
                        }
                        (Value::Str(sa), other) => {
                            let s = sa.to_string() + &other.stringify();
                            Value::Str(Arc::new(s))
                        }
                        (other, Value::Str(sb)) => {
                            let s = other.stringify() + &sb;
                            Value::Str(Arc::new(s))
                        }
                        _ => return Err("Invalid operands for +".into()),
                    };
                    frame.set_reg(dst, res);
                }
                Instr::SubInt { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
                        (Value::Float(x), Value::Float(y)) => Value::Float(x - y),
                        (Value::Int(x), Value::Float(y)) => Value::Float(x as f64 - y),
                        (Value::Float(x), Value::Int(y)) => Value::Float(x - y as f64),
                        _ => return Err("Invalid operands for -".into()),
                    };
                    frame.set_reg(dst, res);
                }
                Instr::MulInt { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
                        (Value::Float(x), Value::Float(y)) => Value::Float(x * y),
                        (Value::Int(x), Value::Float(y)) => Value::Float(x as f64 * y),
                        (Value::Float(x), Value::Int(y)) => Value::Float(x * y as f64),
                        _ => return Err("Invalid operands for *".into()),
                    };
                    frame.set_reg(dst, res);
                }
                Instr::DivInt { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Int(x), Value::Int(y)) => {
                            if y == 0 {
                                return Err("Division by zero".into());
                            }
                            Value::Int(x / y)
                        }
                        (Value::Float(x), Value::Float(y)) => {
                            if y == 0.0 {
                                return Err("Division by zero".into());
                            }
                            Value::Float(x / y)
                        }
                        (Value::Int(x), Value::Float(y)) => {
                            if y == 0.0 {
                                return Err("Division by zero".into());
                            }
                            Value::Float(x as f64 / y)
                        }
                        (Value::Float(x), Value::Int(y)) => {
                            if y == 0 {
                                return Err("Division by zero".into());
                            }
                            Value::Float(x / y as f64)
                        }
                        _ => return Err("Invalid operands for /".into()),
                    };
                    frame.set_reg(dst, res);
                }
                Instr::Mod { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Int(x), Value::Int(y)) => {
                            if y == 0 {
                                return Err("Modulo by zero".into());
                            }
                            Value::Int(x % y)
                        }
                        _ => return Err("Modulo expects integers".into()),
                    };
                    frame.set_reg(dst, res);
                }
                Instr::Negate { dst, src } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = match frame.get_reg(src) {
                        Value::Int(x) => Value::Int(-x),
                        Value::Float(x) => Value::Float(-x),
                        _ => return Err("Negation expects number".into()),
                    };
                    frame.set_reg(dst, res);
                }
                Instr::CmpEq { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Int(x), Value::Int(y)) => x == y,
                        (Value::Float(x), Value::Float(y)) => x == y,
                        (Value::Int(x), Value::Float(y)) => (x as f64) == y,
                        (Value::Float(x), Value::Int(y)) => x == (y as f64),
                        (Value::Str(x), Value::Str(y)) => x == y,
                        (Value::Bool(x), Value::Bool(y)) => x == y,
                        (Value::Null, Value::Null) => true,
                        _ => false,
                    };
                    frame.set_reg(dst, Value::Bool(res));
                }
                Instr::CmpNe { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Int(x), Value::Int(y)) => x != y,
                        (Value::Float(x), Value::Float(y)) => x != y,
                        (Value::Int(x), Value::Float(y)) => (x as f64) != y,
                        (Value::Float(x), Value::Int(y)) => x != (y as f64),
                        (Value::Str(x), Value::Str(y)) => x != y,
                        (Value::Bool(x), Value::Bool(y)) => x != y,
                        (Value::Null, Value::Null) => false,
                        _ => true,
                    };
                    frame.set_reg(dst, Value::Bool(res));
                }
                Instr::CmpLt { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Int(x), Value::Int(y)) => x < y,
                        (Value::Float(x), Value::Float(y)) => x < y,
                        (Value::Int(x), Value::Float(y)) => (x as f64) < y,
                        (Value::Float(x), Value::Int(y)) => x < (y as f64),
                        (Value::Str(x), Value::Str(y)) => x < y,
                        _ => return Err("Invalid operands for <".into()),
                    };
                    frame.set_reg(dst, Value::Bool(res));
                }
                Instr::CmpGt { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Int(x), Value::Int(y)) => x > y,
                        (Value::Float(x), Value::Float(y)) => x > y,
                        (Value::Int(x), Value::Float(y)) => (x as f64) > y,
                        (Value::Float(x), Value::Int(y)) => x > (y as f64),
                        (Value::Str(x), Value::Str(y)) => x > y,
                        _ => return Err("Invalid operands for >".into()),
                    };
                    frame.set_reg(dst, Value::Bool(res));
                }
                Instr::CmpLe { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Int(x), Value::Int(y)) => x <= y,
                        (Value::Float(x), Value::Float(y)) => x <= y,
                        (Value::Int(x), Value::Float(y)) => (x as f64) <= y,
                        (Value::Float(x), Value::Int(y)) => x <= (y as f64),
                        (Value::Str(x), Value::Str(y)) => x <= y,
                        _ => return Err("Invalid operands for <=".into()),
                    };
                    frame.set_reg(dst, Value::Bool(res));
                }
                Instr::CmpGe { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Int(x), Value::Int(y)) => x >= y,
                        (Value::Float(x), Value::Float(y)) => x >= y,
                        (Value::Int(x), Value::Float(y)) => (x as f64) >= y,
                        (Value::Float(x), Value::Int(y)) => x >= (y as f64),
                        (Value::Str(x), Value::Str(y)) => x >= y,
                        _ => return Err("Invalid operands for >=".into()),
                    };
                    frame.set_reg(dst, Value::Bool(res));
                }
                Instr::And { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = frame.get_reg(a).is_truthy() && frame.get_reg(b).is_truthy();
                    frame.set_reg(dst, Value::Bool(res));
                }
                Instr::Or { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = frame.get_reg(a).is_truthy() || frame.get_reg(b).is_truthy();
                    frame.set_reg(dst, Value::Bool(res));
                }
                Instr::Not { dst, src } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = !frame.get_reg(src).is_truthy();
                    frame.set_reg(dst, Value::Bool(res));
                }
                Instr::Jump { target } => {
                    self.frames.last_mut().unwrap().ip = target.0 as usize;
                }
                Instr::JumpIfFalse { cond, target } => {
                    let frame = self.frames.last_mut().unwrap();
                    if !frame.get_reg(cond).is_truthy() {
                        frame.ip = target.0 as usize;
                    }
                }
                Instr::JumpIfTrue { cond, target } => {
                    let frame = self.frames.last_mut().unwrap();
                    if frame.get_reg(cond).is_truthy() {
                        frame.ip = target.0 as usize;
                    }
                }
                Instr::Return { src } => {
                    let frame = self.frames.pop().unwrap();
                    let val = src.map(|s| frame.get_reg(s)).unwrap_or(Value::Null);
                    if let Some(ret) = self.return_from_frame(val, frame.return_reg) {
                        return Ok(ret);
                    }
                }
                Instr::Call { dst, callee, args } => {
                    let (func, arg_vals, module_dir) = {
                        let frame = self.frames.last().unwrap();
                        let func = frame.get_reg(callee);
                        let arg_vals = Self::collect_call_args(frame, &args);
                        (func, arg_vals, frame.module_dir.clone())
                    };
                    match func {
                        Value::NativeFunction(f) => {
                            if arg_vals.iter().any(|(name, _)| name.is_some()) {
                                return Err(
                                    "Named arguments are not supported for native functions".into(),
                                );
                            }
                            let res =
                                f(self, arg_vals.into_iter().map(|(_, value)| value).collect());
                            if let Some(d) = dst {
                                self.frames.last_mut().unwrap().set_reg(d, res);
                            }
                        }
                        Value::Function(c) => {
                            let mut new_frame = CallFrame::new(c, module_dir, dst);
                            let bound =
                                Self::bind_named_args(&new_frame.chunk.param_names, arg_vals)?;
                            for (i, v) in bound.into_iter().enumerate() {
                                new_frame.set_reg(Reg(i as u8), v);
                            }
                            self.frames.push(new_frame);
                        }
                        Value::Class { name, fields } => {
                            let bound = Self::bind_named_args(&fields, arg_vals)?;
                            let mut map = HashMap::new();
                            for (i, field) in fields.iter().enumerate() {
                                map.insert(
                                    field.clone(),
                                    bound.get(i).cloned().unwrap_or(Value::Null),
                                );
                            }
                            let val = Value::Struct {
                                class: name,
                                fields: Arc::new(Mutex::new(map)),
                            };
                            if let Some(d) = dst {
                                self.frames.last_mut().unwrap().set_reg(d, val);
                            }
                        }
                        _ => return Err(format!("Not callable: {}", func.type_name())),
                    }
                }
                Instr::Spawn { callee, args } => {
                    let (func, arg_vals, globals, cache, script_args, mdir) = {
                        let frame = self.frames.last().unwrap();
                        (
                            frame.get_reg(callee),
                            Self::collect_call_args(frame, &args),
                            self.globals.clone(),
                            self.module_cache.clone(),
                            self.script_args.clone(),
                            frame.module_dir.clone(),
                        )
                    };
                    thread::spawn(move || {
                        let mut vm = VM::with_shared_cache(cache);
                        vm.globals = globals;
                        vm.script_args = script_args;
                        let _ = vm.invoke(func, arg_vals, mdir);
                    });
                }
                Instr::LoadGlobal { dst, name } => {
                    let frame = self.frames.last_mut().unwrap();
                    let n = frame.chunk.names[name.0 as usize].clone();
                    let v = self.globals.get(&n).cloned().unwrap_or(Value::Null);
                    frame.set_reg(dst, v);
                }
                Instr::StoreGlobal { name, src } => {
                    let frame = self.frames.last_mut().unwrap();
                    let n = frame.chunk.names[name.0 as usize].clone();
                    let v = frame.get_reg(src);
                    self.globals.insert(n, v);
                }
                Instr::MakeList { dst, items } => {
                    let frame = self.frames.last_mut().unwrap();
                    let list: Vec<Value> = items.iter().map(|&i| frame.get_reg(i)).collect();
                    frame.set_reg(dst, Value::List(Arc::new(Mutex::new(list))));
                }
                Instr::MakeMap { dst, pairs } => {
                    let frame = self.frames.last_mut().unwrap();
                    let mut map = HashMap::new();
                    for (k, v) in pairs {
                        if let Value::Str(s) = frame.get_reg(k) {
                            map.insert(s.to_string(), frame.get_reg(v));
                        }
                    }
                    frame.set_reg(dst, Value::Map(Arc::new(Mutex::new(map))));
                }
                Instr::MakeRange { dst, start, end } => {
                    let frame = self.frames.last_mut().unwrap();
                    let (s, e) = match (frame.get_reg(start), frame.get_reg(end)) {
                        (Value::Int(s), Value::Int(e)) => (s, e),
                        _ => return Err("Range bounds must be integers".into()),
                    };
                    frame.set_reg(dst, Value::Range { start: s, end: e });
                }
                Instr::MakeStruct { dst, class, fields } => {
                    let frame = self.frames.last_mut().unwrap();
                    let class_name = frame.chunk.names[class.0 as usize].clone();
                    let mut map = HashMap::new();
                    for (ni, reg) in fields {
                        let field_name = frame.chunk.names[ni.0 as usize].clone();
                        map.insert(field_name, frame.get_reg(reg));
                    }
                    frame.set_reg(
                        dst,
                        Value::Struct {
                            class: Arc::new(class_name),
                            fields: Arc::new(Mutex::new(map)),
                        },
                    );
                }
                Instr::Len { dst, src } => {
                    let frame = self.frames.last_mut().unwrap();
                    let n = match frame.get_reg(src) {
                        Value::Str(s) => s.chars().count() as i64,
                        Value::List(l) => l.lock().unwrap().len() as i64,
                        Value::Map(m) => m.lock().unwrap().len() as i64,
                        Value::Range { start, end } => (end - start).max(0),
                        _ => 0,
                    };
                    frame.set_reg(dst, Value::Int(n));
                }
                Instr::Concat { dst, parts } => {
                    let frame = self.frames.last_mut().unwrap();
                    let mut s = String::new();
                    for p in parts {
                        s.push_str(&frame.get_reg(p).stringify());
                    }
                    frame.set_reg(dst, Value::Str(Arc::new(s)));
                }
                Instr::Stringify { dst, src } => {
                    let frame = self.frames.last_mut().unwrap();
                    let s = frame.get_reg(src).stringify();
                    frame.set_reg(dst, Value::Str(Arc::new(s)));
                }
                Instr::MakeError { dst, msg } => {
                    let frame = self.frames.last_mut().unwrap();
                    let m = frame.get_reg(msg).stringify();
                    frame.set_reg(dst, Value::Error(Arc::new(m)));
                }
                Instr::Propagate { src } => {
                    let frame = self.frames.pop().unwrap();
                    let val = frame.get_reg(src);
                    if let Value::Error(ref e) = val {
                        if self.frames.is_empty() {
                            return Err(e.to_string());
                        }
                        if let Some(ret) = self.return_from_frame(val, frame.return_reg) {
                            return Ok(ret);
                        }
                    } else {
                        self.frames.push(frame);
                    }
                }
                Instr::IsError { dst, src } => {
                    let frame = self.frames.last_mut().unwrap();
                    let is_err = matches!(frame.get_reg(src), Value::Error(_));
                    frame.set_reg(dst, Value::Bool(is_err));
                }
                Instr::LoadField { dst, obj, field } => {
                    let frame = self.frames.last_mut().unwrap();
                    let field_name = frame.chunk.names[field.0 as usize].clone();
                    let obj_val = frame.get_reg(obj);
                    let val = match obj_val {
                        Value::Struct { fields, .. } => fields
                            .lock()
                            .unwrap()
                            .get(&field_name)
                            .cloned()
                            .unwrap_or(Value::Null),
                        Value::Module(m) => m.get(&field_name).cloned().unwrap_or(Value::Null),
                        Value::Map(m) => m
                            .lock()
                            .unwrap()
                            .get(&field_name)
                            .cloned()
                            .unwrap_or(Value::Null),
                        _ => Value::Null,
                    };
                    frame.set_reg(dst, val);
                }
                Instr::StoreField { obj, field, src } => {
                    let frame = self.frames.last_mut().unwrap();
                    let field_name = frame.chunk.names[field.0 as usize].clone();
                    let obj_val = frame.get_reg(obj);
                    let val = frame.get_reg(src);
                    match obj_val {
                        Value::Struct { fields, .. } => {
                            fields.lock().unwrap().insert(field_name, val);
                        }
                        Value::Map(m) => {
                            m.lock().unwrap().insert(field_name, val);
                        }
                        _ => return Err("Cannot set field on non-struct/map".into()),
                    }
                }
                Instr::LoadIndex { dst, obj, idx } => {
                    let frame = self.frames.last_mut().unwrap();
                    let o = frame.get_reg(obj);
                    let i = frame.get_reg(idx);
                    let val = match (o, i) {
                        (Value::List(l), Value::Int(n)) => {
                            let b = l.lock().unwrap();
                            let idx = if n < 0 { b.len() as i64 + n } else { n };
                            if idx >= 0 && (idx as usize) < b.len() {
                                b[idx as usize].clone()
                            } else {
                                return Err(format!("Index {n} out of bounds"));
                            }
                        }
                        (Value::Map(m), Value::Str(s)) => {
                            m.lock().unwrap().get(&*s).cloned().unwrap_or(Value::Null)
                        }
                        (Value::Range { start, end }, Value::Int(n)) => {
                            let len = (end - start).max(0);
                            if n >= 0 && n < len {
                                Value::Int(start + n)
                            } else {
                                return Err(format!("Index {n} out of range"));
                            }
                        }
                        (Value::Str(s), Value::Int(n)) => {
                            let chars: Vec<char> = s.chars().collect();
                            let idx = if n < 0 { chars.len() as i64 + n } else { n };
                            if idx >= 0 && (idx as usize) < chars.len() {
                                Value::Str(Arc::new(chars[idx as usize].to_string()))
                            } else {
                                return Err(format!("String index {n} out of bounds"));
                            }
                        }
                        _ => return Err("Invalid index operation".into()),
                    };
                    frame.set_reg(dst, val);
                }
                Instr::StoreIndex { obj, idx, src } => {
                    let frame = self.frames.last_mut().unwrap();
                    let o = frame.get_reg(obj);
                    let i = frame.get_reg(idx);
                    let v = frame.get_reg(src);
                    match (o, i) {
                        (Value::List(l), Value::Int(n)) => {
                            let mut b = l.lock().unwrap();
                            let idx = if n < 0 { b.len() as i64 + n } else { n };
                            if idx >= 0 && (idx as usize) < b.len() {
                                b[idx as usize] = v;
                            } else {
                                return Err(format!("Index {n} out of bounds"));
                            }
                        }
                        (Value::Map(m), Value::Str(s)) => {
                            m.lock().unwrap().insert(s.to_string(), v);
                        }
                        _ => return Err("Invalid index assignment".into()),
                    }
                }
                Instr::ForIter { dst, src, step } => {
                    let frame = self.frames.last_mut().unwrap();
                    let val = frame.get_reg(src);
                    let step_val = step.map(|reg| frame.get_reg(reg));
                    let items = Self::iterator_items(val, step_val)?;
                    let iter = Value::Iterator {
                        items: Arc::new(Mutex::new(items)),
                        pos: Arc::new(Mutex::new(0)),
                    };
                    frame.set_reg(dst, iter);
                }
                Instr::IterNext { var, iter, done } => {
                    let frame = self.frames.last_mut().unwrap();
                    let iter_val = frame.get_reg(iter);
                    match iter_val {
                        Value::Iterator { items, pos } => {
                            let mut p = pos.lock().unwrap();
                            let items_lock = items.lock().unwrap();
                            if *p >= items_lock.len() {
                                frame.ip = done.0 as usize;
                            } else {
                                let item = items_lock[*p].clone();
                                *p += 1;
                                drop(p);
                                drop(items_lock);
                                frame.set_reg(var, item);
                            }
                        }
                        _ => return Err("IterNext on non-iterator".into()),
                    }
                }
                Instr::AddFloat { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let (x, y) = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Float(x), Value::Float(y)) => (x, y),
                        _ => return Err("AddFloat expects floats".into()),
                    };
                    frame.set_reg(dst, Value::Float(x + y));
                }
                Instr::SubFloat { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let (x, y) = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Float(x), Value::Float(y)) => (x, y),
                        _ => return Err("SubFloat expects floats".into()),
                    };
                    frame.set_reg(dst, Value::Float(x - y));
                }
                Instr::MulFloat { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let (x, y) = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Float(x), Value::Float(y)) => (x, y),
                        _ => return Err("MulFloat expects floats".into()),
                    };
                    frame.set_reg(dst, Value::Float(x * y));
                }
                Instr::DivFloat { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let (x, y) = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Float(x), Value::Float(y)) => (x, y),
                        _ => return Err("DivFloat expects floats".into()),
                    };
                    if y == 0.0 {
                        return Err("Float division by zero".into());
                    }
                    frame.set_reg(dst, Value::Float(x / y));
                }
            }
        }
    }

    fn return_from_frame(&mut self, val: Value, return_reg: Option<Reg>) -> Option<Value> {
        if let Some(frame) = self.frames.last_mut() {
            if let Some(dst) = return_reg {
                frame.set_reg(dst, val);
            }
            None
        } else {
            Some(val)
        }
    }

    fn invoke(
        &mut self,
        func: Value,
        args: Vec<RuntimeCallArg>,
        module_dir: PathBuf,
    ) -> Result<Value, String> {
        match func {
            Value::NativeFunction(f) => {
                if args.iter().any(|(name, _)| name.is_some()) {
                    return Err("Named arguments are not supported for native functions".into());
                }
                Ok(f(self, args.into_iter().map(|(_, value)| value).collect()))
            }
            Value::Function(c) => {
                let mut frame = CallFrame::new(c, module_dir, None);
                let bound = Self::bind_named_args(&frame.chunk.param_names, args)?;
                for (i, v) in bound.into_iter().enumerate() {
                    frame.set_reg(Reg(i as u8), v);
                }
                self.frames.push(frame);
                self.execute()
            }
            Value::Class { name, fields } => {
                let bound = Self::bind_named_args(&fields, args)?;
                let mut map = HashMap::new();
                for (i, field) in fields.iter().enumerate() {
                    map.insert(field.clone(), bound.get(i).cloned().unwrap_or(Value::Null));
                }
                Ok(Value::Struct {
                    class: name,
                    fields: Arc::new(Mutex::new(map)),
                })
            }
            _ => Err("Not a function".into()),
        }
    }
}
