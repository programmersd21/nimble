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
    pub func_id: u32,
}

impl CallFrame {
    pub fn new(
        chunk: Arc<FunctionChunk>,
        module_dir: PathBuf,
        return_reg: Option<Reg>,
        func_id: u32,
    ) -> Self {
        let num_regs = (chunk.num_registers as usize).max(64);
        Self {
            chunk,
            ip: 0,
            registers: vec![Value::Null; num_regs],
            return_reg,
            module_dir,
            func_id,
        }
    }

    #[inline]
    pub fn get_reg(&self, reg: Reg) -> Value {
        self.registers[reg.0 as usize].clone()
    }

    #[inline]
    pub fn set_reg(&mut self, reg: Reg, val: Value) {
        let idx = reg.0 as usize;
        if idx >= self.registers.len() {
            self.registers.resize(idx + 1, Value::Null);
        }
        self.registers[idx] = val;
    }
}
