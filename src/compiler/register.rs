use std::collections::{HashMap, HashSet};
use crate::compiler::ir::ValueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalRegister(pub u8);

impl PhysicalRegister {
    // x86-64 general purpose registers
    pub const RAX: PhysicalRegister = PhysicalRegister(0);
    pub const RBX: PhysicalRegister = PhysicalRegister(1);
    pub const RCX: PhysicalRegister = PhysicalRegister(2);
    pub const RDX: PhysicalRegister = PhysicalRegister(3);
    pub const RSI: PhysicalRegister = PhysicalRegister(4);
    pub const RDI: PhysicalRegister = PhysicalRegister(5);
    pub const RBP: PhysicalRegister = PhysicalRegister(6);
    pub const RSP: PhysicalRegister = PhysicalRegister(7);
    pub const R8: PhysicalRegister = PhysicalRegister(8);
    pub const R9: PhysicalRegister = PhysicalRegister(9);
    pub const R10: PhysicalRegister = PhysicalRegister(10);
    pub const R11: PhysicalRegister = PhysicalRegister(11);
    pub const R12: PhysicalRegister = PhysicalRegister(12);
    pub const R13: PhysicalRegister = PhysicalRegister(13);
    pub const R14: PhysicalRegister = PhysicalRegister(14);
    pub const R15: PhysicalRegister = PhysicalRegister(15);
}

pub struct InterferenceGraph {
    /// Adjacency list: virtual register -> set of interfering virtual registers.
    pub adj: HashMap<ValueId, HashSet<ValueId>>,
    /// Degrees of nodes for coloring.
    pub degrees: HashMap<ValueId, usize>,
}

impl InterferenceGraph {
    pub fn new() -> Self {
        Self {
            adj: HashMap::new(),
            degrees: HashMap::new(),
        }
    }

    pub fn add_interference(&mut self, u: ValueId, v: ValueId) {
        if u == v { return; }
        
        if self.adj.entry(u).or_default().insert(v) {
            *self.degrees.entry(u).or_default() += 1;
        }
        if self.adj.entry(v).or_default().insert(u) {
            *self.degrees.entry(v).or_default() += 1;
        }
    }
}

pub struct RegisterAllocator;

impl RegisterAllocator {
    pub fn new() -> Self {
        Self
    }

    /// Colors the interference graph and identifies registers to spill.
    pub fn allocate(
        &mut self,
        graph: &InterferenceGraph,
        num_physical: usize,
    ) -> (HashMap<ValueId, PhysicalRegister>, Vec<ValueId>) {
        let mut allocation = HashMap::new();
        let mut spills = Vec::new();
        let mut stack = Vec::new();
        let mut temp_degrees = graph.degrees.clone();
        let mut nodes: Vec<ValueId> = graph.degrees.keys().cloned().collect();

        // 1. Simplify: Push nodes with degree < num_physical onto stack
        while !nodes.is_empty() {
            if let Some(pos) = nodes.iter().position(|n| *temp_degrees.get(n).unwrap_or(&0) < num_physical) {
                let node = nodes.remove(pos);
                stack.push(node);
                for neighbor in graph.adj.get(&node).unwrap_or(&HashSet::new()) {
                    if let Some(d) = temp_degrees.get_mut(neighbor) {
                        *d = d.saturating_sub(1);
                    }
                }
            } else {
                // Potential spill: pick node with highest degree
                let node = *nodes.iter().max_by_key(|n| temp_degrees.get(n).unwrap_or(&0)).unwrap();
                nodes.retain(|&n| n != node);
                stack.push(node);
            }
        }

        // 2. Select: Pop and assign colors
        let physical_regs = [
            PhysicalRegister::RAX, PhysicalRegister::RBX, PhysicalRegister::RCX,
            PhysicalRegister::RDX, PhysicalRegister::RSI, PhysicalRegister::RDI,
            PhysicalRegister::R8, PhysicalRegister::R9, PhysicalRegister::R10,
            PhysicalRegister::R11, PhysicalRegister::R12, PhysicalRegister::R13,
            PhysicalRegister::R14, PhysicalRegister::R15,
        ];

        while let Some(node) = stack.pop() {
            let mut used_colors = HashSet::new();
            for neighbor in graph.adj.get(&node).unwrap_or(&HashSet::new()) {
                if let Some(reg) = allocation.get(neighbor) {
                    used_colors.insert(*reg);
                }
            }

            let mut assigned = false;
            for &reg in physical_regs.iter().take(num_physical) {
                if !used_colors.contains(&reg) {
                    allocation.insert(node, reg);
                    assigned = true;
                    break;
                }
            }

            if !assigned {
                spills.push(node);
            }
        }

        (allocation, spills)
    }
}
