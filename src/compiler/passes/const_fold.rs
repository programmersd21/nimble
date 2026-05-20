//! Constant folding optimization pass.
//!
//! Evaluates constant expressions at compile time and replaces them
//! with their computed values. This reduces runtime computation cost.

use crate::compiler::ir::{IRFunction, IRInstr, ValueId};
use std::collections::HashMap;

/// Result of attempting to fold a constant expression.
#[derive(Clone, Debug, PartialEq)]
pub enum FoldedValue {
    /// An integer constant.
    Int(i64),

    /// A floating point constant.
    Float(f64),

    /// A string constant.
    String(String),

    /// Null value.
    Null,

    /// Value could not be folded (not a constant).
    NotConstant,
}

/// Constant folding pass.
pub struct ConstantFolder {
    /// Mapping from SSA value ID to its constant value (if known).
    constants: HashMap<ValueId, FoldedValue>,
}

impl ConstantFolder {
    pub fn new() -> Self {
        Self {
            constants: HashMap::new(),
        }
    }

    /// Run constant folding on a function.
    pub fn fold(&mut self, func: &mut IRFunction) {
        // First pass: identify all constant values
        for block_id in func.cfg.blocks.keys().copied().collect::<Vec<_>>() {
            if let Some(block) = func.cfg.get_block(block_id) {
                for (value_id, instr) in &block.instrs {
                    if let Some(folded) = self.try_fold(instr, &self.constants.clone()) {
                        self.constants.insert(*value_id, folded);
                    }
                }
            }
        }

        // Second pass: replace constant expressions with folded values
        for block_id in func.cfg.blocks.keys().copied().collect::<Vec<_>>() {
            if let Some(block) = func.cfg.get_block_mut(block_id) {
                let mut modified_instrs = Vec::new();

                for (value_id, instr) in &block.instrs {
                    let folded = self.try_fold(instr, &self.constants);
                    if let Some(folded_val) = folded {
                        // Replace with constant
                        let const_instr = match folded_val {
                            FoldedValue::Int(n) => IRInstr::ConstInt { value: n },
                            FoldedValue::Float(f) => IRInstr::ConstFloat { value: f },
                            FoldedValue::String(s) => IRInstr::ConstString { value: s },
                            FoldedValue::Null => IRInstr::ConstNull,
                            FoldedValue::NotConstant => instr.clone(),
                        };
                        modified_instrs.push((*value_id, const_instr));
                    } else {
                        modified_instrs.push((*value_id, instr.clone()));
                    }
                }

                block.instrs = modified_instrs;
            }
        }
    }

    /// Attempt to fold a single instruction.
    pub fn try_fold(
        &self,
        instr: &IRInstr,
        constants: &HashMap<ValueId, FoldedValue>,
    ) -> Option<FoldedValue> {
        match instr {
            IRInstr::ConstInt { value } => Some(FoldedValue::Int(*value)),
            IRInstr::ConstFloat { value } => Some(FoldedValue::Float(*value)),
            IRInstr::ConstString { value } => Some(FoldedValue::String(value.clone())),
            IRInstr::ConstNull => Some(FoldedValue::Null),

            // Fold integer arithmetic
            IRInstr::AddInt { a, b } => {
                let a_val = constants.get(a)?;
                let b_val = constants.get(b)?;
                if let (FoldedValue::Int(a_n), FoldedValue::Int(b_n)) = (a_val, b_val) {
                    Some(FoldedValue::Int(a_n.wrapping_add(*b_n)))
                } else {
                    None
                }
            }

            IRInstr::SubInt { a, b } => {
                let a_val = constants.get(a)?;
                let b_val = constants.get(b)?;
                if let (FoldedValue::Int(a_n), FoldedValue::Int(b_n)) = (a_val, b_val) {
                    Some(FoldedValue::Int(a_n.wrapping_sub(*b_n)))
                } else {
                    None
                }
            }

            IRInstr::MulInt { a, b } => {
                let a_val = constants.get(a)?;
                let b_val = constants.get(b)?;
                if let (FoldedValue::Int(a_n), FoldedValue::Int(b_n)) = (a_val, b_val) {
                    Some(FoldedValue::Int(a_n.wrapping_mul(*b_n)))
                } else {
                    None
                }
            }

            IRInstr::DivInt { a, b } => {
                let a_val = constants.get(a)?;
                let b_val = constants.get(b)?;
                if let (FoldedValue::Int(a_n), FoldedValue::Int(b_n)) = (a_val, b_val) {
                    if *b_n != 0 {
                        Some(FoldedValue::Int(a_n / b_n))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            IRInstr::ModInt { a, b } => {
                let a_val = constants.get(a)?;
                let b_val = constants.get(b)?;
                if let (FoldedValue::Int(a_n), FoldedValue::Int(b_n)) = (a_val, b_val) {
                    if *b_n != 0 {
                        Some(FoldedValue::Int(a_n % b_n))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            // Fold floating point arithmetic
            IRInstr::AddFloat { a, b } => {
                let a_val = constants.get(a)?;
                let b_val = constants.get(b)?;
                if let (FoldedValue::Float(a_f), FoldedValue::Float(b_f)) = (a_val, b_val) {
                    Some(FoldedValue::Float(a_f + b_f))
                } else {
                    None
                }
            }

            IRInstr::SubFloat { a, b } => {
                let a_val = constants.get(a)?;
                let b_val = constants.get(b)?;
                if let (FoldedValue::Float(a_f), FoldedValue::Float(b_f)) = (a_val, b_val) {
                    Some(FoldedValue::Float(a_f - b_f))
                } else {
                    None
                }
            }

            IRInstr::MulFloat { a, b } => {
                let a_val = constants.get(a)?;
                let b_val = constants.get(b)?;
                if let (FoldedValue::Float(a_f), FoldedValue::Float(b_f)) = (a_val, b_val) {
                    Some(FoldedValue::Float(a_f * b_f))
                } else {
                    None
                }
            }

            IRInstr::DivFloat { a, b } => {
                let a_val = constants.get(a)?;
                let b_val = constants.get(b)?;
                if let (FoldedValue::Float(a_f), FoldedValue::Float(b_f)) = (a_val, b_val) {
                    if *b_f != 0.0 {
                        Some(FoldedValue::Float(a_f / b_f))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            // Algebraic simplifications
            IRInstr::Negate { value } => {
                if let Some(FoldedValue::Int(n)) = constants.get(value) {
                    Some(FoldedValue::Int(-n))
                } else if let Some(FoldedValue::Float(f)) = constants.get(value) {
                    Some(FoldedValue::Float(-f))
                } else {
                    None
                }
            }

            // Can't fold other instructions
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_folding() {
        let mut folder = ConstantFolder::new();

        // Fold 5 + 10 = 15
        let add_instr = IRInstr::AddInt {
            a: ValueId(1),
            b: ValueId(2),
        };

        folder.constants.insert(ValueId(1), FoldedValue::Int(5));
        folder.constants.insert(ValueId(2), FoldedValue::Int(10));

        let result = folder.try_fold(&add_instr, &folder.constants.clone());
        assert_eq!(result, Some(FoldedValue::Int(15)));
    }

    #[test]
    fn test_float_folding() {
        let mut folder = ConstantFolder::new();

        let add_instr = IRInstr::AddFloat {
            a: ValueId(1),
            b: ValueId(2),
        };

        folder.constants.insert(ValueId(1), FoldedValue::Float(3.5));
        folder.constants.insert(ValueId(2), FoldedValue::Float(2.5));

        let result = folder.try_fold(&add_instr, &folder.constants.clone());
        assert_eq!(result, Some(FoldedValue::Float(6.0)));
    }
}
