//! Optimization passes for the Nimble IR.
//!
//! These passes transform and optimize the SSA-based intermediate representation
//! to enable high-performance execution.

pub mod const_fold;
pub mod dce;

pub use const_fold::{ConstantFolder, FoldedValue};
pub use dce::DeadCodeEliminator;
