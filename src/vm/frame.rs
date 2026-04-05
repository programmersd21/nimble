use crate::compiler::bytecode::Chunk;
use crate::vm::Value;
use std::sync::Arc;

pub struct CallFrame {
    pub chunk: Arc<Chunk>,
    pub ip: usize,
    pub registers: [Value; 256],
}

impl CallFrame {
    pub fn new(chunk: Arc<Chunk>) -> Self {
        Self {
            chunk,
            ip: 0,
            registers: std::array::from_fn(|_| Value::Null),
        }
    }

    pub fn get(&self, reg: u8) -> Value {
        self.registers[reg as usize].clone()
    }

    pub fn set(&mut self, reg: u8, value: Value) {
        self.registers[reg as usize] = value;
    }
}
