//! Dead Code Elimination (DCE) optimization pass.
//!
//! Removes instructions whose results are never used, and removes
//! unreachable blocks from the control flow graph.

use crate::compiler::ir::{IRFunction, IRInstr, ValueId};
use std::collections::{HashSet};

/// Dead code elimination pass.
pub struct DeadCodeEliminator;

impl DeadCodeEliminator {
    pub fn new() -> Self {
        Self
    }

    /// Run DCE on a function.
    pub fn eliminate(&self, func: &mut IRFunction) {
        // First pass: identify live values
        let live_values = self.compute_live_values(func);

        // Second pass: remove dead instructions
        for block_id in func.cfg.blocks.keys().copied().collect::<Vec<_>>() {
            if let Some(block) = func.cfg.get_block_mut(block_id) {
                block.instrs.retain(|(value_id, _)| live_values.contains(value_id));
            }
        }

        // Third pass: remove unreachable blocks
        self.remove_unreachable_blocks(func);
    }

    /// Compute which values are live (used downstream).
    fn compute_live_values(&self, func: &IRFunction) -> HashSet<ValueId> {
        let mut live = HashSet::new();

        // Work backwards from exit block to mark all used values
        let mut worklist = vec![func.cfg.exit_block.unwrap_or(func.cfg.entry_block)];
        let mut visited = HashSet::new();

        while let Some(block_id) = worklist.pop() {
            if visited.contains(&block_id) {
                continue;
            }
            visited.insert(block_id);

            if let Some(block) = func.cfg.get_block(block_id) {
                // All instructions in this block that contribute to successors are live
                for (value_id, instr) in &block.instrs {
                    // Mark the value as live if it's used by a successor
                    // or if it has side effects
                    if self.has_side_effects(instr) || self.is_used_by_successors(func, block_id, *value_id) {
                        live.insert(*value_id);

                        // Mark operands as live
                        self.mark_operands_live(instr, &mut live);
                    }
                }

                // Add predecessors to worklist
                for &pred_id in &block.successors {
                    if !visited.contains(&pred_id) {
                        worklist.push(pred_id);
                    }
                }
            }
        }

        live
    }

    /// Does this instruction have side effects (can't be eliminated)?
    fn has_side_effects(&self, instr: &IRInstr) -> bool {
        matches!(
            instr,
            IRInstr::Store { .. }
                | IRInstr::Call { .. }
                | IRInstr::Alloc { .. }
        )
    }

    /// Is this value used by any successor block?
    fn is_used_by_successors(
        &self,
        func: &IRFunction,
        block_id: u32,
        value_id: ValueId,
    ) -> bool {
        if let Some(block) = func.cfg.get_block(block_id) {
            for &succ_id in &block.successors {
                if let Some(succ_block) = func.cfg.get_block(succ_id) {
                    for (_, instr) in &succ_block.instrs {
                        if self.uses_value(instr, value_id) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Mark all operands of an instruction as live.
    fn mark_operands_live(&self, instr: &IRInstr, live: &mut HashSet<ValueId>) {
        match instr {
            IRInstr::AddInt { a, b } | IRInstr::SubInt { a, b } | IRInstr::MulInt { a, b }
            | IRInstr::DivInt { a, b } | IRInstr::ModInt { a, b }
            | IRInstr::AddFloat { a, b } | IRInstr::SubFloat { a, b }
            | IRInstr::MulFloat { a, b } | IRInstr::DivFloat { a, b }
            | IRInstr::LessThanInt { a, b } | IRInstr::LessEqualInt { a, b }
            | IRInstr::EqualInt { a, b } | IRInstr::NotEqualInt { a, b } => {
                live.insert(*a);
                live.insert(*b);
            }

            IRInstr::Load { addr, .. } => {
                live.insert(*addr);
            }

            IRInstr::Store { addr, value, .. } => {
                live.insert(*addr);
                live.insert(*value);
            }

            IRInstr::Negate { value } | IRInstr::Not { value } => {
                live.insert(*value);
            }

            IRInstr::Cast { value, .. } => {
                live.insert(*value);
            }

            IRInstr::Call { func, args, .. } => {
                live.insert(*func);
                for arg in args {
                    live.insert(*arg);
                }
            }

            IRInstr::Phi { incoming } => {
                for (_, value_id) in incoming {
                    live.insert(*value_id);
                }
            }

            _ => {}
        }
    }

    /// Does an instruction use this value?
    fn uses_value(&self, instr: &IRInstr, value_id: ValueId) -> bool {
        match instr {
            IRInstr::AddInt { a, b } | IRInstr::SubInt { a, b } | IRInstr::MulInt { a, b }
            | IRInstr::DivInt { a, b } | IRInstr::ModInt { a, b }
            | IRInstr::AddFloat { a, b } | IRInstr::SubFloat { a, b }
            | IRInstr::MulFloat { a, b } | IRInstr::DivFloat { a, b }
            | IRInstr::LessThanInt { a, b } | IRInstr::LessEqualInt { a, b }
            | IRInstr::EqualInt { a, b } | IRInstr::NotEqualInt { a, b } => {
                *a == value_id || *b == value_id
            }

            IRInstr::Load { addr, .. } => *addr == value_id,

            IRInstr::Store { addr, value, .. } => *addr == value_id || *value == value_id,

            IRInstr::Negate { value } | IRInstr::Not { value } => *value == value_id,

            IRInstr::Cast { value, .. } => *value == value_id,

            IRInstr::Call { func, args, .. } => {
                *func == value_id || args.iter().any(|arg| *arg == value_id)
            }

            IRInstr::Phi { incoming } => {
                incoming.iter().any(|(_, vid)| *vid == value_id)
            }

            _ => false,
        }
    }

    /// Remove unreachable blocks from the CFG.
    fn remove_unreachable_blocks(&self, func: &mut IRFunction) {
        let mut reachable = HashSet::new();
        let mut worklist = vec![func.cfg.entry_block];

        while let Some(block_id) = worklist.pop() {
            if reachable.contains(&block_id) {
                continue;
            }
            reachable.insert(block_id);

            if let Some(block) = func.cfg.get_block(block_id) {
                for &succ_id in &block.successors {
                    if !reachable.contains(&succ_id) {
                        worklist.push(succ_id);
                    }
                }
            }
        }

        // Remove unreachable blocks
        func.cfg.blocks.retain(|id, _| reachable.contains(id));

        // Mark blocks as unreachable
        for block in func.cfg.blocks.values_mut() {
            block.reachable = reachable.contains(&block.id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::ir::IRType;

    #[test]
    fn test_dead_code_elimination() {
        let mut func = IRFunction::new(
            "test".to_string(),
            vec![],
            IRType::Int64,
        );

        let v1 = func.alloc_value();
        let block = func.new_block();
        func.cfg.entry_block = block;
        func.cfg.exit_block = Some(block);

        if let Some(b) = func.cfg.get_block_mut(block) {
            // Dead: result is never used
            b.push(v1, IRInstr::ConstInt { value: 42 });
        }

        let eliminator = DeadCodeEliminator::new();
        eliminator.eliminate(&mut func);

        // Dead instruction should be removed
        assert_eq!(func.cfg.get_block(block).unwrap().instrs.len(), 0);
    }
}
