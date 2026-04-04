#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reg(pub u8);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstIdx(pub u16);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NameIdx(pub u16);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BuiltinIdx(pub u16);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Label(pub u32);

#[derive(Debug, Clone)]
pub enum Instr {
    // Data movement
    LoadConst {
        dst: Reg,
        idx: ConstIdx,
    },
    Move {
        dst: Reg,
        src: Reg,
    },
    LoadGlobal {
        dst: Reg,
        name: NameIdx,
    },
    StoreGlobal {
        name: NameIdx,
        src: Reg,
    },
    LoadField {
        dst: Reg,
        obj: Reg,
        field: NameIdx,
    },
    StoreField {
        obj: Reg,
        field: NameIdx,
        src: Reg,
    },
    LoadIndex {
        dst: Reg,
        obj: Reg,
        idx: Reg,
    },
    StoreIndex {
        obj: Reg,
        idx: Reg,
        src: Reg,
    },

    // Arithmetic
    AddInt {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    AddFloat {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    SubInt {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    SubFloat {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    MulInt {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    MulFloat {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    DivInt {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    DivFloat {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    Mod {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    Negate {
        dst: Reg,
        src: Reg,
    },

    // Comparison
    CmpEq {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    CmpNe {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    CmpLt {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    CmpGt {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    CmpLe {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    CmpGe {
        dst: Reg,
        a: Reg,
        b: Reg,
    },

    // Logic
    And {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    Or {
        dst: Reg,
        a: Reg,
        b: Reg,
    },
    Not {
        dst: Reg,
        src: Reg,
    },

    // Control flow
    Jump {
        target: Label,
    },
    JumpIfFalse {
        cond: Reg,
        target: Label,
    },
    JumpIfTrue {
        cond: Reg,
        target: Label,
    },
    Return {
        src: Option<Reg>,
    },

    // Calls
    Call {
        dst: Option<Reg>,
        callee: Reg,
        args: Vec<Reg>,
    },
    CallBuiltin {
        dst: Option<Reg>,
        name: BuiltinIdx,
        args: Vec<Reg>,
    },
    Spawn {
        callee: Reg,
        args: Vec<Reg>,
    },

    // Collections
    MakeList {
        dst: Reg,
        items: Vec<Reg>,
    },
    MakeMap {
        dst: Reg,
        pairs: Vec<(Reg, Reg)>,
    },
    MakeRange {
        dst: Reg,
        start: Reg,
        end: Reg,
    },
    MakeStruct {
        dst: Reg,
        class: NameIdx,
        fields: Vec<(NameIdx, Reg)>,
    },
    Len {
        dst: Reg,
        src: Reg,
    },

    // String interp
    Concat {
        dst: Reg,
        parts: Vec<Reg>,
    },
    Stringify {
        dst: Reg,
        src: Reg,
    },

    // Error handling
    MakeError {
        dst: Reg,
        msg: Reg,
    },
    Propagate {
        src: Reg,
    },
    IsError {
        dst: Reg,
        src: Reg,
    },
}

#[derive(Debug, Clone)]
pub struct FunctionChunk {
    pub name: String,
    pub instrs: Vec<Instr>,
    pub constants: Vec<crate::vm::Value>,
    pub names: Vec<String>,
    pub num_registers: u8,
    pub exports: Vec<String>,
}
