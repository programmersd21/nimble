use crate::compiler::bytecode::{FunctionChunk, Reg};
use crate::vm::Value;
use std::path::PathBuf;
use std::sync::Arc;

pub struct CallFrame {
    pub chunk: Arc<FunctionChunk>,
    pub ip: usize,
    pub registers: Vec<Value>,
    pub return_reg: Option<Reg>,
    pub module_dir: PathBuf,
}

impl CallFrame {
    pub fn new(chunk: Arc<FunctionChunk>, module_dir: PathBuf, return_reg: Option<Reg>) -> Self {
        let num_regs = chunk.num_registers as usize;
        Self {
            chunk,
            ip: 0,
            registers: vec![Value::Null; num_regs],
            return_reg,
            module_dir,
        }
    }

    pub fn get_reg(&self, reg: Reg) -> Value {
        self.registers[reg.0 as usize].clone()
    }

    pub fn set_reg(&mut self, reg: Reg, val: Value) {
        self.registers[reg.0 as usize] = val;
    }
}
