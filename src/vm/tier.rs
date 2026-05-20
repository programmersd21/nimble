//! Tiered execution system for Nimble runtime.
//!
//! Manages which tier a function is currently executing in (Tier 0, 1, or 2)
//! and orchestrates transitions between tiers.

use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;

// ── Tier Enumeration ──────────────────────────────────────────────────────────

/// Execution tiers in the Nimble runtime.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExecutionTier {
    /// Tier 0: Direct bytecode interpretation.
    ///
    /// Fast startup, slow execution. Used initially for all code.
    Interpreter = 0,

    /// Tier 1: Baseline JIT compilation.
    ///
    /// Quick compilation (<10ms), moderate optimization.
    /// Entered when function hotness exceeds 10,000 calls.
    BaselineJIT = 1,

    /// Tier 2: Optimizing JIT compilation.
    ///
    /// Full SSA-based optimization, high compilation cost (1–5ms).
    /// Entered when function hotness exceeds 100,000 calls or loop is hot.
    OptimizingJIT = 2,
}

impl ExecutionTier {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(ExecutionTier::Interpreter),
            1 => Some(ExecutionTier::BaselineJIT),
            2 => Some(ExecutionTier::OptimizingJIT),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ExecutionTier::Interpreter => "Interpreter (Tier 0)",
            ExecutionTier::BaselineJIT => "Baseline JIT (Tier 1)",
            ExecutionTier::OptimizingJIT => "Optimizing JIT (Tier 2)",
        }
    }
}

// ── Tier State ────────────────────────────────────────────────────────────────

/// State tracking for a function's tier progression.
pub struct TierState {
    /// Current execution tier.
    current_tier: AtomicU8,

    /// When was the last tier transition?
    last_transition_us: std::sync::Mutex<u64>,

    /// Is Tier 1 compilation pending?
    tier1_pending: std::sync::atomic::AtomicBool,

    /// Is Tier 2 compilation pending?
    tier2_pending: std::sync::atomic::AtomicBool,

    /// Should we deoptimize from Tier 2?
    deoptimize_requested: std::sync::atomic::AtomicBool,
}

impl TierState {
    pub fn new() -> Self {
        Self {
            current_tier: AtomicU8::new(ExecutionTier::Interpreter as u8),
            last_transition_us: std::sync::Mutex::new(0),
            tier1_pending: std::sync::atomic::AtomicBool::new(false),
            tier2_pending: std::sync::atomic::AtomicBool::new(false),
            deoptimize_requested: std::sync::atomic::AtomicBool::new(false),
        }
    }

    /// Get the current execution tier.
    pub fn current(&self) -> ExecutionTier {
        ExecutionTier::from_u8(self.current_tier.load(Ordering::Acquire))
            .unwrap_or(ExecutionTier::Interpreter)
    }

    /// Request transition to Tier 1 (non-blocking).
    ///
    /// If Tier 1 code is ready, `apply_pending_tier1()` will transition execution.
    pub fn request_tier1(&self) -> bool {
        !self.tier1_pending.swap(true, Ordering::Relaxed)
    }

    /// Request transition to Tier 2 (non-blocking).
    pub fn request_tier2(&self) -> bool {
        !self.tier2_pending.swap(true, Ordering::Relaxed)
    }

    /// Has Tier 1 been requested?
    pub fn is_tier1_pending(&self) -> bool {
        self.tier1_pending.load(Ordering::Acquire)
    }

    /// Has Tier 2 been requested?
    pub fn is_tier2_pending(&self) -> bool {
        self.tier2_pending.load(Ordering::Acquire)
    }

    /// Mark that Tier 1 compilation is done. Transition to Tier 1.
    fn current_time_micros() -> u64 {
        std::time::Instant::now().elapsed().as_micros() as u64
    }

    pub fn apply_pending_tier1(&self) {
        if self.tier1_pending.load(Ordering::Acquire) {
            self.current_tier.store(ExecutionTier::BaselineJIT as u8, Ordering::Release);
            if let Ok(mut last) = self.last_transition_us.lock() {
                *last = Self::current_time_micros();
            }
            self.tier1_pending.store(false, Ordering::Release);
        }
    }

    /// Mark that Tier 2 compilation is done. Transition to Tier 2.
    pub fn apply_pending_tier2(&self) {
        if self.tier2_pending.load(Ordering::Acquire) {
            self.current_tier.store(ExecutionTier::OptimizingJIT as u8, Ordering::Release);
            if let Ok(mut last) = self.last_transition_us.lock() {
                *last = Self::current_time_micros();
            }
            self.tier2_pending.store(false, Ordering::Release);
        }
    }

    /// Request deoptimization from Tier 2 back to Tier 0.
    pub fn request_deoptimization(&self) {
        self.deoptimize_requested.store(true, Ordering::Release);
    }

    /// Should we deoptimize?
    pub fn should_deoptimize(&self) -> bool {
        self.deoptimize_requested.load(Ordering::Acquire)
    }

    /// Apply deoptimization: revert to Tier 0 interpreter.
    pub fn apply_deoptimization(&self) {
        self.current_tier.store(ExecutionTier::Interpreter as u8, Ordering::Release);
        if let Ok(mut last) = self.last_transition_us.lock() {
            *last = Self::current_time_micros();
        }
        self.deoptimize_requested.store(false, Ordering::Release);
    }

    /// Reset all pending compilations (for testing/debugging).
    pub fn reset(&self) {
        self.current_tier.store(ExecutionTier::Interpreter as u8, Ordering::Release);
        self.tier1_pending.store(false, Ordering::Release);
        self.tier2_pending.store(false, Ordering::Release);
        self.deoptimize_requested.store(false, Ordering::Release);
    }
}

// ── Tier Manager ──────────────────────────────────────────────────────────────

/// Coordinates JIT compilation across all functions.
#[derive(Clone)]
pub struct TierManager {
    /// Queue of functions pending Tier 1 compilation.
    tier1_queue: Arc<std::sync::Mutex<Vec<u32>>>,

    /// Queue of functions pending Tier 2 compilation.
    tier2_queue: Arc<std::sync::Mutex<Vec<u32>>>,

    /// Is background JIT compilation enabled?
    jit_enabled: Arc<std::sync::atomic::AtomicBool>,
}

impl TierManager {
    pub fn new() -> Self {
        Self {
            tier1_queue: Arc::new(std::sync::Mutex::new(Vec::new())),
            tier2_queue: Arc::new(std::sync::Mutex::new(Vec::new())),
            jit_enabled: Arc::new(std::sync::atomic::AtomicBool::new(true)),
        }
    }

    /// Enqueue a function for Tier 1 compilation.
    pub fn enqueue_tier1(&self, func_id: u32) {
        if self.jit_enabled.load(Ordering::Relaxed) {
            self.tier1_queue.lock().unwrap().push(func_id);
        }
    }

    /// Enqueue a function for Tier 2 compilation.
    pub fn enqueue_tier2(&self, func_id: u32) {
        if self.jit_enabled.load(Ordering::Relaxed) {
            self.tier2_queue.lock().unwrap().push(func_id);
        }
    }

    /// Get the next function to compile in Tier 1.
    pub fn dequeue_tier1(&self) -> Option<u32> {
        self.tier1_queue.lock().unwrap().pop()
    }

    /// Get the next function to compile in Tier 2.
    pub fn dequeue_tier2(&self) -> Option<u32> {
        self.tier2_queue.lock().unwrap().pop()
    }

    /// Disable JIT compilation (for debugging).
    pub fn disable_jit(&self) {
        self.jit_enabled.store(false, Ordering::Release);
    }

    /// Enable JIT compilation.
    pub fn enable_jit(&self) {
        self.jit_enabled.store(true, Ordering::Release);
    }

    /// Is JIT compilation enabled?
    pub fn is_jit_enabled(&self) -> bool {
        self.jit_enabled.load(Ordering::Acquire)
    }
}
