use crate::compiler::bytecode::{FunctionChunk, Instr};
use cranelift::prelude::*;

pub struct CodeGenerator<'a> {
    builder: &'a mut FunctionBuilder<'a>,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(builder: &'a mut FunctionBuilder<'a>) -> Self {
        Self { builder }
    }

    pub fn translate(&mut self, chunk: &FunctionChunk) {
        let entry_block = self.builder.create_block();
        self.builder
            .append_block_params_for_function_params(entry_block);
        self.builder.switch_to_block(entry_block);
        self.builder.seal_block(entry_block);

        // Map VM registers to Cranelift variables
        let mut regs = Vec::new();
        for i in 0..chunk.num_registers {
            let var = Variable::new(i as usize);
            self.builder.declare_var(var, types::I64);
            regs.push(var);
        }

        for instr in &chunk.instrs {
            match instr {
                Instr::AddInt { dst, a, b } => {
                    let va = self.builder.use_var(regs[a.0 as usize]);
                    let vb = self.builder.use_var(regs[b.0 as usize]);
                    let res = self.builder.ins().iadd(va, vb);
                    self.builder.def_var(regs[dst.0 as usize], res);
                }
                Instr::SubInt { dst, a, b } => {
                    let va = self.builder.use_var(regs[a.0 as usize]);
                    let vb = self.builder.use_var(regs[b.0 as usize]);
                    let res = self.builder.ins().isub(va, vb);
                    self.builder.def_var(regs[dst.0 as usize], res);
                }
                Instr::Return { src } => {
                    if let Some(s) = src {
                        let val = self.builder.use_var(regs[s.0 as usize]);
                        self.builder.ins().return_(&[val]);
                    } else {
                        self.builder.ins().return_(&[]);
                    }
                }
                _ => {} // Fallback to VM for complex instructions
            }
        }
    }
}
