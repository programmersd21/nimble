pub mod builtins;
pub mod frame;
pub mod gc;
pub mod value;
pub mod nan_value;
pub mod vm;

// ── Tiered execution infrastructure ───────────────────────────────────────────
pub mod profiling;
pub mod tier;
pub mod safepoint;
pub mod pool;

// ── Memory management (Phase 2) ──────────────────────────────────────────────
pub mod memory;

pub use gc::{BumpAllocator, Heap};
pub use value::{Value, WeakValue};
pub use vm::VM;
pub use profiling::{FunctionProfile, InlineCache, ProfileRegistry};
pub use tier::{ExecutionTier, TierState, TierManager};
pub use safepoint::{SafepointCoordinator, SafepointCheck};
pub use memory::{GenerationalHeap, WriteBarrier, ArenaAllocator, SizeClass};
