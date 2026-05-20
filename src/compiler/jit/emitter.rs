//! x86-64 machine code emitter for Nimble JIT.
//!
//! Provides a simple, robust interface for emitting x86-64 machine instructions.
//! Used by the Baseline JIT to generate native code from SSA IR.

pub struct X64Emitter {
    code: Vec<u8>,
}

impl X64Emitter {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    /// Emit a raw byte.
    pub fn emit_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }

    /// Emit a 32-bit immediate (little-endian).
    pub fn emit_u32(&mut self, val: u32) {
        self.code.extend_from_slice(&val.to_le_bytes());
    }

    /// Move immediate 64-bit value to register.
    /// mov rax, imm64 (REX.W + B8 + imm64)
    pub fn emit_mov_rax_imm64(&mut self, imm: u64) {
        self.emit_byte(0x48); // REX.W
        self.emit_byte(0xb8); // mov rax, imm64
        self.code.extend_from_slice(&imm.to_le_bytes());
    }

    /// Add register to rax.
    /// add rax, rbx (REX.W + 01 D8)
    pub fn emit_add_rax_rbx(&mut self) {
        self.emit_byte(0x48); // REX.W
        self.emit_byte(0x01);
        self.emit_byte(0xd8);
    }

    /// Return from function.
    /// ret (C3)
    pub fn emit_ret(&mut self) {
        self.emit_byte(0xc3);
    }

    /// Get the generated machine code.
    pub fn code(&self) -> &[u8] {
        &self.code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emit_basic_add() {
        let mut emitter = X64Emitter::new();
        emitter.emit_mov_rax_imm64(10);
        emitter.emit_add_rax_rbx();
        emitter.emit_ret();

        // 48 B8 0A 00 00 00 00 00 00 00 48 01 D8 C3
        let expected = [
            0x48, 0xb8, 0x0a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
            0x48, 0x01, 0xd8, 0xc3
        ];
        assert_eq!(emitter.code(), expected);
    }
}
