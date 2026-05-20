use nimble::compiler::bytecode::{FunctionChunk, Instr, Reg};
use nimble::vm::VM;
use std::sync::Arc;
use std::time::Instant;

fn main() {
    // 1. Create a simple loop:
    // r0 = 0
    // r1 = 0 (counter)
    // loop:
    // r0 = r0 + 1
    // r1 = r1 + 1
    // if r1 < 1000 jump loop
    
    let mut chunk = FunctionChunk::new("bench".into());
    chunk.constants.push(nimble::vm::Value::Int(1));
    chunk.constants.push(nimble::vm::Value::Int(1000));
    
    chunk.instrs.push(Instr::LoadConst { dst: Reg(0), idx: nimble::compiler::bytecode::ConstIdx(0) }); // r0 = 0
    // Simplified loop setup
    
    let chunk = Arc::new(chunk);
    let mut vm = VM::new();
    
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = vm.run(Arc::clone(&chunk));
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
