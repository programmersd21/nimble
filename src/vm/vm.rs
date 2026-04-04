use crate::compiler::bytecode::Instr;
use crate::modules::resolver::ModuleResolver;
use crate::vm::builtins;
use crate::vm::frame::CallFrame;
use crate::vm::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct VM {
    frames: Vec<CallFrame>,
    globals: HashMap<String, Value>,
    module_cache: Arc<Mutex<HashMap<String, Value>>>,
    resolver: ModuleResolver,
}

impl VM {
    pub fn new() -> Self {
        let mut vm = Self {
            frames: Vec::new(),
            globals: HashMap::new(),
            module_cache: Arc::new(Mutex::new(HashMap::new())),
            resolver: ModuleResolver::new(),
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
        };
        vm.install_builtins();
        vm
    }

    fn install_builtins(&mut self) {
        self.globals
            .insert("out".into(), Value::NativeFunction(builtins::out));
        self.globals
            .insert("in".into(), Value::NativeFunction(builtins::input));
        self.globals
            .insert("input".into(), Value::NativeFunction(builtins::input));
        self.globals
            .insert("len".into(), Value::NativeFunction(builtins::len));
        self.globals
            .insert("to_int".into(), Value::NativeFunction(builtins::to_int));
        self.globals
            .insert("index_of".into(), Value::NativeFunction(builtins::index_of));
        self.globals
            .insert("error".into(), Value::NativeFunction(builtins::error));

        self.globals.insert(
            "__builtin_file_exists".into(),
            Value::NativeFunction(builtins::builtin_file_exists),
        );
        self.globals.insert(
            "__builtin_io_read_file".into(),
            Value::NativeFunction(builtins::builtin_io_read_file),
        );
        self.globals.insert(
            "__builtin_io_write_file".into(),
            Value::NativeFunction(builtins::builtin_io_write_file),
        );
        self.globals.insert(
            "__builtin_io_append_file".into(),
            Value::NativeFunction(builtins::builtin_io_append_file),
        );
        self.globals.insert(
            "__builtin_io_delete_file".into(),
            Value::NativeFunction(builtins::builtin_io_delete_file),
        );
        self.globals.insert(
            "__builtin_io_read_lines".into(),
            Value::NativeFunction(builtins::builtin_io_read_lines),
        );
        self.globals.insert(
            "__builtin_io_write_lines".into(),
            Value::NativeFunction(builtins::builtin_io_write_lines),
        );
        self.globals.insert(
            "__builtin_io_read_bytes".into(),
            Value::NativeFunction(builtins::builtin_io_read_bytes),
        );
        self.globals.insert(
            "__builtin_io_write_bytes".into(),
            Value::NativeFunction(builtins::builtin_io_write_bytes),
        );
        self.globals.insert(
            "__builtin_io_copy_file".into(),
            Value::NativeFunction(builtins::builtin_io_copy_file),
        );
        self.globals.insert(
            "__builtin_io_stderr".into(),
            Value::NativeFunction(builtins::builtin_io_stderr),
        );
        self.globals.insert(
            "__builtin_time_now".into(),
            Value::NativeFunction(builtins::builtin_time_now),
        );
        self.globals.insert(
            "__builtin_time_sleep".into(),
            Value::NativeFunction(builtins::builtin_time_sleep),
        );
        self.globals.insert(
            "__builtin_os_args".into(),
            Value::NativeFunction(builtins::builtin_os_args),
        );
        self.globals.insert(
            "__builtin_os_exit".into(),
            Value::NativeFunction(builtins::builtin_os_exit),
        );
        self.globals.insert(
            "__builtin_path_join".into(),
            Value::NativeFunction(builtins::builtin_path_join),
        );
        self.globals.insert(
            "__builtin_process_run".into(),
            Value::NativeFunction(builtins::builtin_process_run),
        );
        self.globals.insert(
            "__builtin_regex_matches".into(),
            Value::NativeFunction(builtins::builtin_regex_matches),
        );
        self.globals.insert(
            "__builtin_regex_find".into(),
            Value::NativeFunction(builtins::builtin_regex_find),
        );
        self.globals.insert(
            "__builtin_regex_find_all".into(),
            Value::NativeFunction(builtins::builtin_regex_find_all),
        );
        self.globals.insert(
            "__builtin_regex_replace".into(),
            Value::NativeFunction(builtins::builtin_regex_replace),
        );
        self.globals.insert(
            "__builtin_regex_replace_all".into(),
            Value::NativeFunction(builtins::builtin_regex_replace_all),
        );
        self.globals.insert(
            "__builtin_regex_split".into(),
            Value::NativeFunction(builtins::builtin_regex_split),
        );
        self.globals.insert(
            "__builtin_json_parse".into(),
            Value::NativeFunction(builtins::builtin_json_parse),
        );
        self.globals.insert(
            "__builtin_json_stringify".into(),
            Value::NativeFunction(builtins::builtin_json_stringify),
        );
        self.globals.insert(
            "__builtin_json_pretty".into(),
            Value::NativeFunction(builtins::builtin_json_pretty),
        );
        self.globals.insert(
            "__builtin_net_http_get".into(),
            Value::NativeFunction(builtins::builtin_net_http_get),
        );
        self.globals.insert(
            "__builtin_map_has".into(),
            Value::NativeFunction(builtins::builtin_map_has),
        );
        self.globals.insert(
            "__builtin_map_values".into(),
            Value::NativeFunction(builtins::builtin_map_values),
        );
        self.globals.insert(
            "__map_keys".into(),
            Value::NativeFunction(builtins::map_keys),
        );
        self.globals.insert(
            "__builtin_list_push".into(),
            Value::NativeFunction(builtins::builtin_list_push),
        );
        self.globals.insert(
            "__builtin_list_pop".into(),
            Value::NativeFunction(builtins::builtin_list_pop),
        );
        self.globals.insert(
            "__builtin_list_first".into(),
            Value::NativeFunction(builtins::builtin_list_first),
        );
        self.globals.insert(
            "__builtin_list_last".into(),
            Value::NativeFunction(builtins::builtin_list_last),
        );
        self.globals.insert(
            "__builtin_list_insert".into(),
            Value::NativeFunction(builtins::builtin_list_insert),
        );
        self.globals.insert(
            "__builtin_list_remove".into(),
            Value::NativeFunction(builtins::builtin_list_remove),
        );
        self.globals.insert(
            "__builtin_list_slice".into(),
            Value::NativeFunction(builtins::builtin_list_slice),
        );
        self.globals.insert(
            "__builtin_list_reverse".into(),
            Value::NativeFunction(builtins::builtin_list_reverse),
        );
        self.globals.insert(
            "__builtin_list_sort".into(),
            Value::NativeFunction(builtins::builtin_list_sort),
        );
        self.globals.insert(
            "__builtin_str_split".into(),
            Value::NativeFunction(builtins::builtin_str_split),
        );
        self.globals.insert(
            "__builtin_str_join".into(),
            Value::NativeFunction(builtins::builtin_str_join),
        );
        self.globals.insert(
            "__builtin_str_trim".into(),
            Value::NativeFunction(builtins::builtin_str_trim),
        );
        self.globals.insert(
            "__builtin_str_trim_start".into(),
            Value::NativeFunction(builtins::builtin_str_trim_start),
        );
        self.globals.insert(
            "__builtin_str_trim_end".into(),
            Value::NativeFunction(builtins::builtin_str_trim_end),
        );
        self.globals.insert(
            "__builtin_str_upper".into(),
            Value::NativeFunction(builtins::builtin_str_upper),
        );
        self.globals.insert(
            "__builtin_str_lower".into(),
            Value::NativeFunction(builtins::builtin_str_lower),
        );
        self.globals.insert(
            "__builtin_str_starts_with".into(),
            Value::NativeFunction(builtins::builtin_str_starts_with),
        );
        self.globals.insert(
            "__builtin_str_ends_with".into(),
            Value::NativeFunction(builtins::builtin_str_ends_with),
        );
        self.globals.insert(
            "__builtin_str_replace".into(),
            Value::NativeFunction(builtins::builtin_str_replace),
        );
        self.globals.insert(
            "__builtin_str_replace_all".into(),
            Value::NativeFunction(builtins::builtin_str_replace_all),
        );
        self.globals.insert(
            "__builtin_str_count".into(),
            Value::NativeFunction(builtins::builtin_str_count),
        );
        self.globals.insert(
            "__builtin_str_index_of".into(),
            Value::NativeFunction(builtins::builtin_str_index_of),
        );
        self.globals.insert(
            "__builtin_str_slice".into(),
            Value::NativeFunction(builtins::builtin_str_slice),
        );
        self.globals.insert(
            "__builtin_str_repeat".into(),
            Value::NativeFunction(builtins::builtin_str_repeat),
        );
        self.globals.insert(
            "__builtin_str_pad_left".into(),
            Value::NativeFunction(builtins::builtin_str_pad_left),
        );
        self.globals.insert(
            "__builtin_str_pad_right".into(),
            Value::NativeFunction(builtins::builtin_str_pad_right),
        );
        self.globals.insert(
            "__builtin_str_to_int".into(),
            Value::NativeFunction(builtins::builtin_str_to_int),
        );
        self.globals.insert(
            "__builtin_str_to_float".into(),
            Value::NativeFunction(builtins::builtin_str_to_float),
        );
        self.globals.insert(
            "__builtin_str_from_int".into(),
            Value::NativeFunction(builtins::builtin_str_from_int),
        );
        self.globals.insert(
            "__builtin_str_from_float".into(),
            Value::NativeFunction(builtins::builtin_str_from_float),
        );
        self.globals.insert(
            "__builtin_str_chars".into(),
            Value::NativeFunction(builtins::builtin_str_chars),
        );
        self.globals.insert(
            "__builtin_str_len".into(),
            Value::NativeFunction(builtins::builtin_str_len),
        );
        self.globals.insert(
            "__builtin_str_is_numeric".into(),
            Value::NativeFunction(builtins::builtin_str_is_numeric),
        );
        self.globals.insert(
            "__builtin_str_is_alpha".into(),
            Value::NativeFunction(builtins::builtin_str_is_alpha),
        );
        self.globals.insert(
            "__builtin_str_format".into(),
            Value::NativeFunction(builtins::builtin_str_format),
        );
        self.globals.insert(
            "__builtin_math_pow".into(),
            Value::NativeFunction(builtins::builtin_math_pow),
        );
        self.globals.insert(
            "__builtin_math_sqrt".into(),
            Value::NativeFunction(builtins::builtin_math_sqrt),
        );
        self.globals.insert(
            "__builtin_math_abs".into(),
            Value::NativeFunction(builtins::builtin_math_abs),
        );
        self.globals.insert(
            "__builtin_math_floor".into(),
            Value::NativeFunction(builtins::builtin_math_floor),
        );
        self.globals.insert(
            "__builtin_math_ceil".into(),
            Value::NativeFunction(builtins::builtin_math_ceil),
        );
        self.globals.insert(
            "__builtin_math_round".into(),
            Value::NativeFunction(builtins::builtin_math_round),
        );
        self.globals.insert(
            "__builtin_math_min".into(),
            Value::NativeFunction(builtins::builtin_math_min),
        );
        self.globals.insert(
            "__builtin_math_max".into(),
            Value::NativeFunction(builtins::builtin_math_max),
        );
        self.globals.insert(
            "__builtin_math_clamp".into(),
            Value::NativeFunction(builtins::builtin_math_clamp),
        );
        self.globals.insert(
            "__builtin_math_log".into(),
            Value::NativeFunction(builtins::builtin_math_log),
        );
        self.globals.insert(
            "__builtin_math_log2".into(),
            Value::NativeFunction(builtins::builtin_math_log2),
        );
        self.globals.insert(
            "__builtin_math_sin".into(),
            Value::NativeFunction(builtins::builtin_math_sin),
        );
        self.globals.insert(
            "__builtin_math_cos".into(),
            Value::NativeFunction(builtins::builtin_math_cos),
        );
        self.globals.insert(
            "__builtin_math_tan".into(),
            Value::NativeFunction(builtins::builtin_math_tan),
        );
        self.globals.insert(
            "__builtin_math_random".into(),
            Value::NativeFunction(builtins::builtin_math_random),
        );
        self.globals.insert(
            "__builtin_math_rand_int".into(),
            Value::NativeFunction(builtins::builtin_math_rand_int),
        );
        self.globals.insert(
            "__load_module".into(),
            Value::NativeFunction(builtins::load_module),
        );
    }

    pub fn run(
        &mut self,
        chunk: Arc<crate::compiler::bytecode::FunctionChunk>,
    ) -> Result<Value, String> {
        let dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        self.run_with_dir(chunk, dir)
    }

    pub fn run_with_dir(
        &mut self,
        chunk: Arc<crate::compiler::bytecode::FunctionChunk>,
        module_dir: PathBuf,
    ) -> Result<Value, String> {
        self.frames.push(CallFrame::new(chunk, module_dir, None));
        self.execute()
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

    fn current_module_dir(&self) -> PathBuf {
        self.frames
            .last()
            .map(|f| f.module_dir.clone())
            .unwrap_or_else(|| PathBuf::from("."))
    }

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
                    let va = frame.get_reg(a);
                    let vb = frame.get_reg(b);
                    let res = match (va, vb) {
                        (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
                        (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
                        (Value::Int(x), Value::Float(y)) => Value::Float(x as f64 + y),
                        (Value::Float(x), Value::Int(y)) => Value::Float(x + y as f64),
                        (Value::Str(sa), Value::Str(sb)) => {
                            let mut s = sa.to_string();
                            s.push_str(&sb);
                            Value::Str(Arc::new(s))
                        }
                        (Value::Str(sa), other) => {
                            let mut s = sa.to_string();
                            s.push_str(&other.stringify());
                            Value::Str(Arc::new(s))
                        }
                        (other, Value::Str(sb)) => {
                            let mut s = other.stringify();
                            s.push_str(&sb);
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
                        _ => return Err("Expected numbers for -".into()),
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
                        _ => return Err("Expected numbers for *".into()),
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
                        _ => return Err("Expected numbers for /".into()),
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
                        _ => return Err("Expected ints for %".into()),
                    };
                    frame.set_reg(dst, res);
                }
                Instr::Negate { dst, src } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = match frame.get_reg(src) {
                        Value::Int(x) => Value::Int(-x),
                        Value::Float(x) => Value::Float(-x),
                        _ => return Err("Expected number for unary -".into()),
                    };
                    frame.set_reg(dst, res);
                }
                Instr::CmpEq { dst, a, b } => {
                    let frame = self.frames.last_mut().unwrap();
                    let res = match (frame.get_reg(a), frame.get_reg(b)) {
                        (Value::Int(x), Value::Int(y)) => x == y,
                        (Value::Float(x), Value::Float(y)) => x == y,
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
                    let frame = self.frames.last_mut().unwrap();
                    frame.ip = target.0 as usize;
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
                        let arg_vals: Vec<Value> =
                            args.into_iter().map(|a| frame.get_reg(a)).collect();
                        (func, arg_vals, frame.module_dir.clone())
                    };
                    match func {
                        Value::NativeFunction(f) => {
                            let res = f(self, arg_vals);
                            if let Some(d) = dst {
                                let frame = self.frames.last_mut().unwrap();
                                frame.set_reg(d, res);
                            }
                        }
                        Value::Function(c) => {
                            let mut new_frame = CallFrame::new(c, module_dir, dst);
                            for (i, v) in arg_vals.into_iter().enumerate() {
                                if i < new_frame.registers.len() {
                                    new_frame.registers[i] = v;
                                }
                            }
                            self.frames.push(new_frame);
                        }
                        Value::Class { name, fields } => {
                            let mut map = HashMap::new();
                            for (i, field) in fields.iter().enumerate() {
                                if i < arg_vals.len() {
                                    map.insert(field.clone(), arg_vals[i].clone());
                                } else {
                                    map.insert(field.clone(), Value::Null);
                                }
                            }
                            let val = Value::Struct {
                                class: name,
                                fields: Arc::new(Mutex::new(map)),
                            };
                            if let Some(d) = dst {
                                let frame = self.frames.last_mut().unwrap();
                                frame.set_reg(d, val);
                            }
                        }
                        _ => return Err("Not a function".into()),
                    }
                }
                Instr::Spawn { callee, args } => {
                    let frame = self.frames.last_mut().unwrap();
                    let func = frame.get_reg(callee);
                    let arg_vals: Vec<Value> = args.into_iter().map(|a| frame.get_reg(a)).collect();
                    let globals = self.globals.clone();
                    let cache = self.module_cache.clone();
                    let module_dir = frame.module_dir.clone();

                    thread::spawn(move || {
                        let mut vm = VM::with_shared_cache(cache);
                        vm.globals = globals;
                        let _ = vm.invoke(func, arg_vals, module_dir);
                    });
                }
                Instr::LoadGlobal { dst, name } => {
                    let frame = self.frames.last_mut().unwrap();
                    let n = &frame.chunk.names[name.0 as usize];
                    let v = self.globals.get(n).cloned().unwrap_or(Value::Null);
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
                    let mut list = Vec::new();
                    for i in items {
                        list.push(frame.get_reg(i));
                    }
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
                    let start_v = frame.get_reg(start);
                    let end_v = frame.get_reg(end);
                    let (s, e) = match (start_v, end_v) {
                        (Value::Int(s), Value::Int(e)) => (s, e),
                        _ => return Err("Range bounds must be int".into()),
                    };
                    frame.set_reg(dst, Value::Range { start: s, end: e });
                }
                Instr::MakeStruct { dst, class, fields } => {
                    let frame = self.frames.last_mut().unwrap();
                    let class_name = frame.chunk.names[class.0 as usize].clone();
                    let mut map = HashMap::new();
                    for (name_idx, reg) in fields {
                        let field_name = frame.chunk.names[name_idx.0 as usize].clone();
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
                    let val = frame.get_reg(src);
                    let res = match val {
                        Value::Str(s) => s.chars().count() as i64,
                        Value::List(l) => l.lock().unwrap().len() as i64,
                        Value::Map(m) => m.lock().unwrap().len() as i64,
                        Value::Range { start, end } => (end - start).max(0),
                        _ => 0,
                    };
                    frame.set_reg(dst, Value::Int(res));
                }
                Instr::Concat { dst, parts } => {
                    let frame = self.frames.last_mut().unwrap();
                    let mut out = String::new();
                    for p in parts {
                        out.push_str(&frame.get_reg(p).stringify());
                    }
                    frame.set_reg(dst, Value::Str(Arc::new(out)));
                }
                Instr::Stringify { dst, src } => {
                    let frame = self.frames.last_mut().unwrap();
                    let s = frame.get_reg(src).stringify();
                    frame.set_reg(dst, Value::Str(Arc::new(s)));
                }
                Instr::MakeError { dst, msg } => {
                    let frame = self.frames.last_mut().unwrap();
                    let msg = frame.get_reg(msg).stringify();
                    frame.set_reg(dst, Value::Error(Arc::new(msg)));
                }
                Instr::Propagate { src } => {
                    let frame = self.frames.pop().unwrap();
                    let val = frame.get_reg(src);
                    if matches!(val, Value::Error(_)) {
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
                    let obj_val = frame.get_reg(obj);
                    let field_name = frame.chunk.names[field.0 as usize].clone();
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
                    let obj_val = frame.get_reg(obj);
                    let field_name = frame.chunk.names[field.0 as usize].clone();
                    let val = frame.get_reg(src);
                    match obj_val {
                        Value::Struct { fields, .. } => {
                            fields.lock().unwrap().insert(field_name, val);
                        }
                        _ => return Err("Invalid field assignment".into()),
                    }
                }
                Instr::LoadIndex { dst, obj, idx } => {
                    let frame = self.frames.last_mut().unwrap();
                    let o = frame.get_reg(obj);
                    let i = frame.get_reg(idx);
                    let val = match (o, i) {
                        (Value::List(l), Value::Int(idx)) => {
                            let borrow = l.lock().unwrap();
                            if idx >= 0 && (idx as usize) < borrow.len() {
                                borrow[idx as usize].clone()
                            } else {
                                return Err("Index out of bounds".into());
                            }
                        }
                        (Value::Map(m), Value::Str(s)) => {
                            let borrow = m.lock().unwrap();
                            borrow.get(&*s).cloned().unwrap_or(Value::Null)
                        }
                        (Value::Range { start, end }, Value::Int(idx)) => {
                            let len = (end - start).max(0);
                            if idx >= 0 && idx < len {
                                Value::Int(start + idx)
                            } else {
                                return Err("Index out of bounds".into());
                            }
                        }
                        _ => return Err("Invalid index access".into()),
                    };
                    frame.set_reg(dst, val);
                }
                Instr::StoreIndex { obj, idx, src } => {
                    let frame = self.frames.last_mut().unwrap();
                    let o = frame.get_reg(obj);
                    let i = frame.get_reg(idx);
                    let v = frame.get_reg(src);
                    match (o, i) {
                        (Value::List(l), Value::Int(idx)) => {
                            let mut borrow = l.lock().unwrap();
                            if idx >= 0 && (idx as usize) < borrow.len() {
                                borrow[idx as usize] = v;
                            } else {
                                return Err("Index out of bounds".into());
                            }
                        }
                        (Value::Map(m), Value::Str(s)) => {
                            m.lock().unwrap().insert(s.to_string(), v);
                        }
                        _ => return Err("Invalid index assignment".into()),
                    }
                }
                Instr::CallBuiltin { .. } => {
                    return Err("CallBuiltin not supported in this VM".into());
                }
                Instr::AddFloat { .. }
                | Instr::SubFloat { .. }
                | Instr::MulFloat { .. }
                | Instr::DivFloat { .. } => {
                    return Err("Float-specific ops not used".into());
                }
            }
        }
    }

    fn return_from_frame(
        &mut self,
        val: Value,
        return_reg: Option<crate::compiler::bytecode::Reg>,
    ) -> Option<Value> {
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
        args: Vec<Value>,
        module_dir: PathBuf,
    ) -> Result<Value, String> {
        match func {
            Value::NativeFunction(f) => Ok(f(self, args)),
            Value::Function(c) => {
                let mut frame = CallFrame::new(c, module_dir, None);
                for (i, v) in args.into_iter().enumerate() {
                    if i < frame.registers.len() {
                        frame.registers[i] = v;
                    }
                }
                self.frames.push(frame);
                self.execute()
            }
            _ => Err("Not a function".into()),
        }
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
}
