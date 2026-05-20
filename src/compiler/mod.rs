pub mod bytecode;
pub mod compiler;
pub mod register;
pub mod jit;
pub mod ir;
pub mod passes;

pub use bytecode::{Addr, ConstIdx, FunctionChunk, Instr, NameIdx, Reg};
pub use compiler::Compiler;
pub use register::RegisterAllocator;
pub use jit::{JitCoordinator, CompiledCode};
pub use ir::{IRFunction, IRInstr, IRType, ValueId, ControlFlowGraph, BasicBlock};
pub use passes::{ConstantFolder, DeadCodeEliminator};
