//! SSA-based Intermediate Representation (IR) for optimization.
//!
//! The IR is a Static Single Assignment (SSA) form where:
//! - Each value is assigned exactly once
//! - Values are immutable
//! - Control flow is explicit via basic blocks and phi nodes
//!
//! This enables powerful compiler optimizations like:
//! - Constant folding
//! - Dead code elimination
//! - Loop invariant code motion
//! - Inlining
//! - Escape analysis
//! - Type specialization

use std::collections::{HashMap, HashSet};
use std::fmt;

// ── SSA Values ────────────────────────────────────────────────────────────────

/// A unique identifier for an SSA value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ValueId(pub u32);

impl ValueId {
    pub fn new(id: u32) -> Self {
        ValueId(id)
    }
}

// ── Types in IR ───────────────────────────────────────────────────────────────

/// Type information available during IR construction.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IRType {
    /// 64-bit signed integer.
    Int64,

    /// IEEE 754 double precision floating point.
    Float64,

    /// String (pointer to immutable string data).
    String,

    /// List/array (pointer to vector data).
    List,

    /// Map/dictionary (pointer to hash table).
    Map,

    /// Function/callable (pointer to function metadata).
    Function,

    /// Pointer to heap object (tagged).
    Ptr(Box<IRType>),

    /// Unknown type (inferred or polymorphic).
    Unknown,

    /// Never type (unreachable code).
    Never,
}

impl IRType {
    pub fn is_numeric(&self) -> bool {
        matches!(self, IRType::Int64 | IRType::Float64)
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, IRType::String | IRType::List | IRType::Map | IRType::Function | IRType::Ptr(_))
    }
}

impl fmt::Display for IRType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IRType::Int64 => write!(f, "i64"),
            IRType::Float64 => write!(f, "f64"),
            IRType::String => write!(f, "string"),
            IRType::List => write!(f, "list"),
            IRType::Map => write!(f, "map"),
            IRType::Function => write!(f, "function"),
            IRType::Ptr(t) => write!(f, "*{}", t),
            IRType::Unknown => write!(f, "?"),
            IRType::Never => write!(f, "never"),
        }
    }
}

// ── IR Instructions ──────────────────────────────────────────────────────────

/// An instruction in SSA form.
///
/// Each instruction computes exactly one SSA value (or side effects).
#[derive(Clone, Debug)]
pub enum IRInstr {
    // ── Constant folding ─────────────────────────────────────────────────────
    
    /// Integer constant.
    ConstInt { value: i64 },

    /// Float constant.
    ConstFloat { value: f64 },

    /// String constant.
    ConstString { value: String },

    /// Null constant.
    ConstNull,

    // ── Arithmetic (type-specialized) ────────────────────────────────────────

    /// Integer addition: a + b
    AddInt { a: ValueId, b: ValueId },

    /// Integer subtraction: a - b
    SubInt { a: ValueId, b: ValueId },

    /// Integer multiplication: a * b
    MulInt { a: ValueId, b: ValueId },

    /// Integer division: a / b
    DivInt { a: ValueId, b: ValueId },

    /// Integer modulo: a % b
    ModInt { a: ValueId, b: ValueId },

    /// Floating point addition: a + b
    AddFloat { a: ValueId, b: ValueId },

    /// Floating point subtraction: a - b
    SubFloat { a: ValueId, b: ValueId },

    /// Floating point multiplication: a * b
    MulFloat { a: ValueId, b: ValueId },

    /// Floating point division: a / b
    DivFloat { a: ValueId, b: ValueId },

    // ── Comparisons ──────────────────────────────────────────────────────────

    /// Integer comparison: a < b
    LessThanInt { a: ValueId, b: ValueId },

    /// Integer comparison: a <= b
    LessEqualInt { a: ValueId, b: ValueId },

    /// Integer comparison: a == b
    EqualInt { a: ValueId, b: ValueId },

    /// Integer comparison: a != b
    NotEqualInt { a: ValueId, b: ValueId },

    // ── Memory access ────────────────────────────────────────────────────────

    /// Load a value from memory: load(address)
    Load { addr: ValueId, offset: i32 },

    /// Store a value to memory: store(address, value)
    Store { addr: ValueId, offset: i32, value: ValueId },

    /// Allocate an object on the heap.
    Alloc { size: u32, ty: IRType },

    // ── Control flow ─────────────────────────────────────────────────────────

    /// Merge multiple values from different control flow paths (phi node).
    Phi { incoming: Vec<(u32, ValueId)> }, // (block_id, value) pairs

    // ── Function calls ──────────────────────────────────────────────────────

    /// Call a function: call(func, args...)
    Call {
        func: ValueId,
        args: Vec<ValueId>,
        is_tail: bool,
    },

    // ── Type casts ───────────────────────────────────────────────────────────

    /// Cast between types.
    Cast {
        value: ValueId,
        from_type: IRType,
        to_type: IRType,
    },

    // ── Misc ──────────────────────────────────────────────────────────────────

    /// Unary negation.
    Negate { value: ValueId },

    /// Logical NOT.
    Not { value: ValueId },
}

// ── Basic Blocks (Control Flow Graph) ──────────────────────────────────────────

/// A basic block in the control flow graph.
///
/// A basic block is a sequence of instructions with no branches except at the end.
#[derive(Clone, Debug)]
pub struct BasicBlock {
    /// Unique identifier for this block.
    pub id: u32,

    /// Instructions in this block (in order).
    pub instrs: Vec<(ValueId, IRInstr)>,

    /// Successors (block IDs).
    pub successors: Vec<u32>,

    /// Is this block reachable?
    pub reachable: bool,
}

impl BasicBlock {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            instrs: Vec::new(),
            successors: Vec::new(),
            reachable: true,
        }
    }

    /// Add an instruction to this block.
    pub fn push(&mut self, value_id: ValueId, instr: IRInstr) {
        self.instrs.push((value_id, instr));
    }

    /// Mark a successor block.
    pub fn add_successor(&mut self, block_id: u32) {
        if !self.successors.contains(&block_id) {
            self.successors.push(block_id);
        }
    }
}

// ── Control Flow Graph ────────────────────────────────────────────────────────

/// A control flow graph of basic blocks.
pub struct ControlFlowGraph {
    /// All basic blocks, indexed by ID.
    pub blocks: HashMap<u32, BasicBlock>,

    /// Entry block ID.
    pub entry_block: u32,

    /// Exit block ID (if any).
    pub exit_block: Option<u32>,

    /// Next block ID to assign.
    next_block_id: u32,
}

impl ControlFlowGraph {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            entry_block: 0,
            exit_block: None,
            next_block_id: 1,
        }
    }

    /// Create a new basic block.
    pub fn new_block(&mut self) -> u32 {
        let id = self.next_block_id;
        self.next_block_id += 1;
        self.blocks.insert(id, BasicBlock::new(id));
        id
    }

    /// Get a mutable reference to a block.
    pub fn get_block_mut(&mut self, id: u32) -> Option<&mut BasicBlock> {
        self.blocks.get_mut(&id)
    }

    /// Get an immutable reference to a block.
    pub fn get_block(&self, id: u32) -> Option<&BasicBlock> {
        self.blocks.get(&id)
    }

    /// Compute dominators for each block (for optimizations).
    pub fn compute_dominators(&self) -> HashMap<u32, HashSet<u32>> {
        // Simplified immediate dominator computation
        // (A full implementation would use Lengauer-Tarjan algorithm)
        let mut dominators: HashMap<u32, HashSet<u32>> = HashMap::new();

        for &block_id in self.blocks.keys() {
            dominators.insert(block_id, HashSet::new());
        }

        // Initialize: only entry dominates itself
        if let Some(entry) = dominators.get_mut(&self.entry_block) {
            entry.insert(self.entry_block);
        }

        // Iterate until fixed point
        let mut changed = true;
        while changed {
            changed = false;

            for (&block_id, _block) in &self.blocks {
                if block_id == self.entry_block {
                    continue;
                }

                // Dominators of block = {block} ∪ ∩(dominators of predecessors)
                let mut new_doms = HashSet::new();
                new_doms.insert(block_id);

                for (other_id, other_block) in &self.blocks {
                    if other_block.successors.contains(&block_id) {
                        if let Some(other_doms) = dominators.get(other_id) {
                            if new_doms.is_empty() {
                                new_doms = other_doms.clone();
                            } else {
                                new_doms = new_doms.intersection(other_doms).cloned().collect();
                            }
                            new_doms.insert(*other_id);
                        }
                    }
                }

                if new_doms != *dominators.get(&block_id).unwrap() {
                    changed = true;
                    dominators.insert(block_id, new_doms);
                }
            }
        }

        dominators
    }
}

// ── IR Function ───────────────────────────────────────────────────────────────

/// A complete function in SSA form.
pub struct IRFunction {
    /// Function name.
    pub name: String,

    /// Parameter names.
    pub params: Vec<(String, IRType)>,

    /// Control flow graph.
    pub cfg: ControlFlowGraph,

    /// Next SSA value ID to assign.
    pub next_value_id: u32,

    /// Return type.
    pub return_type: IRType,
}

impl IRFunction {
    pub fn new(name: String, params: Vec<(String, IRType)>, return_type: IRType) -> Self {
        Self {
            name,
            params,
            cfg: ControlFlowGraph::new(),
            next_value_id: 1, // 0 is reserved
            return_type,
        }
    }

    /// Allocate a new SSA value ID.
    pub fn alloc_value(&mut self) -> ValueId {
        let id = ValueId(self.next_value_id);
        self.next_value_id += 1;
        id
    }

    /// Create a new basic block.
    pub fn new_block(&mut self) -> u32 {
        self.cfg.new_block()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ir_function_creation() {
        let mut func = IRFunction::new(
            "add".to_string(),
            vec![
                ("a".to_string(), IRType::Int64),
                ("b".to_string(), IRType::Int64),
            ],
            IRType::Int64,
        );

        let v1 = func.alloc_value();
        let v2 = func.alloc_value();

        assert_eq!(v1.0, 1);
        assert_eq!(v2.0, 2);
    }

    #[test]
    fn test_cfg_blocks() {
        let mut cfg = ControlFlowGraph::new();

        let b1 = cfg.new_block();
        let b2 = cfg.new_block();

        if let Some(block) = cfg.get_block_mut(b1) {
            block.add_successor(b2);
        }

        assert_eq!(cfg.blocks.len(), 2);
        assert!(cfg.get_block(b1).unwrap().successors.contains(&b2));
    }
}
