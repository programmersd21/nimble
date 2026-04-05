/// Compact, cache-friendly bytecode definitions for the future VM.

/// A fixed source location for debugging and profiling.
#[derive(Clone, Copy, Debug)]
pub struct SourceLocation {
    pub line: u32,
    pub column: u32,
}

/// Opcode table for the register-based VM.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
    LoadConst,
    LoadLocal,
    StoreLocal,
    Move,
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Lt,
    Jump,
    JumpIfFalse,
    Call,
    Return,
}

/// A single 32-bit instruction with four 8-bit fields.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub dst: u8,
    pub src1: u8,
    pub src2: u8,
}

impl Instruction {
    /// Pack an immediate in the src1/src2 slots.
    #[inline(always)]
    pub const fn with_imm(opcode: Opcode, dst: u8, imm: u16) -> Self {
        Self {
            opcode,
            dst,
            src1: (imm & 0xFF) as u8,
            src2: (imm >> 8) as u8,
        }
    }

    /// Rebuild the immediate stored in src1/src2.
    #[inline(always)]
    pub const fn imm_u16(&self) -> u16 {
        (self.src1 as u16) | ((self.src2 as u16) << 8)
    }
}

/// Lightweight constant pool used by the compiler.
#[derive(Clone, Debug)]
pub enum Constant {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Null,
}

/// A chunk of bytecode with constants and debug metadata.
pub struct Chunk {
    pub code: Vec<Instruction>,
    pub constants: Vec<Constant>,
    pub debug_info: Vec<SourceLocation>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            debug_info: Vec::new(),
        }
    }
}
