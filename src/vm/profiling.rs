//! Profiling system for Nimble runtime.
//!
//! Tracks function hotness, loop iteration counts, branch frequencies,
//! and type feedback to drive tiered JIT compilation decisions.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;

// ── Function Profile ──────────────────────────────────────────────────────────

/// Profile data for a single function.
///
/// Collected during Tier 0 execution to inform JIT compilation decisions.
#[derive(Debug)]
pub struct FunctionProfile {
    /// How many times has this function been called?
    pub entry_count: AtomicU32,
    
    /// Has this function already been enqueued for JIT?
    pub is_jit_queued: AtomicBool,

    /// How many loop backedges have been taken?
    pub loop_backedge_count: AtomicU32,

    /// How many times have we sampled this function in the profiler?
    pub sample_count: AtomicU32,

    /// Total time spent in this function (in microseconds, if measured).
    pub total_time_us: AtomicU64,

    /// Tier 0 → Tier 1 threshold (default: 10,000 entries)
    pub tier1_threshold: u32,

    /// Tier 1 → Tier 2 threshold (default: 100,000 entries)
    pub tier2_threshold: u32,

    /// Type feedback per call site (indexed by call site offset in bytecode)
    pub type_feedback: Arc<HashMap<u32, Arc<InlineCache>>>,

    /// Branch frequency tracking
    pub branch_frequencies: Arc<HashMap<u32, BranchFrequency>>,
}

impl FunctionProfile {
    pub fn new() -> Self {
        Self {
            entry_count: AtomicU32::new(0),
            loop_backedge_count: AtomicU32::new(0),
            sample_count: AtomicU32::new(0),
            total_time_us: AtomicU64::new(0),
            tier1_threshold: 10_000,
            tier2_threshold: 100_000,
            type_feedback: Arc::new(HashMap::new()),
            branch_frequencies: Arc::new(HashMap::new()),
        }
    }

    /// Increment entry counter and check if should promote to Tier 1.
    pub fn record_entry(&self) -> bool {
        let count = self.entry_count.fetch_add(1, Ordering::Relaxed);
        if count % 100000 == 0 {
            println!("Function entry count reached: {}", count + 1);
        }
        count >= self.tier1_threshold - 1 // Returns true when we cross threshold
    }

    /// Check if should promote to Tier 2.
    pub fn should_promote_to_tier2(&self) -> bool {
        self.entry_count.load(Ordering::Relaxed) >= self.tier2_threshold
            || self.loop_backedge_count.load(Ordering::Relaxed) >= 50_000
    }

    /// Record a loop backedge.
    pub fn record_loop_backedge(&self) -> u32 {
        self.loop_backedge_count.fetch_add(1, Ordering::Relaxed)
    }

    /// Record a profiler sample hit.
    pub fn record_sample(&self) {
        self.sample_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Record time spent in this function (in microseconds).
    pub fn record_time(&self, us: u64) {
        self.total_time_us.fetch_add(us, Ordering::Relaxed);
    }

    /// Get estimated time per entry (in nanoseconds).
    pub fn time_per_entry_ns(&self) -> u64 {
        let total_us = self.total_time_us.load(Ordering::Relaxed);
        let count = self.entry_count.load(Ordering::Relaxed);
        if count == 0 {
            0
        } else {
            (total_us * 1000) / count as u64
        }
    }

    /// Is this function considered "hot"?
    pub fn is_hot(&self) -> bool {
        self.entry_count.load(Ordering::Relaxed) > self.tier1_threshold
    }

    /// Is this function considered "very hot"?
    pub fn is_very_hot(&self) -> bool {
        self.entry_count.load(Ordering::Relaxed) > self.tier2_threshold
    }
}

// ── Inline Cache (Type Feedback) ──────────────────────────────────────────────

/// State machine for inline caches at polymorphic call sites.
///
/// Tracks type distribution to enable type specialization in JIT.
#[derive(Debug)]
pub enum InlineCacheState {
    /// No calls yet.
    Uninitialized,

    /// All calls have been to a single type.
    Monomorphic {
        type_id: u32,
        hit_count: u32,
    },

    /// Calls have been to 2–3 different types.
    Polymorphic {
        types: Vec<(u32, u32)>, // (type_id, count) pairs
    },

    /// Calls have been to too many types. Abandon caching.
    Megamorphic {
        deopt_count: u32,
    },
}

/// Inline cache at a single call site.
#[derive(Debug)]
pub struct InlineCache {
    /// Current state of the cache.
    pub state: std::sync::Mutex<InlineCacheState>,

    /// How many times has this cache missed (and triggered fallback)?
    pub deopt_count: AtomicU32,

    /// Should we deoptimize and regenerate code?
    pub should_deopt: AtomicBool,
}

impl InlineCache {
    pub fn new() -> Self {
        Self {
            state: std::sync::Mutex::new(InlineCacheState::Uninitialized),
            deopt_count: AtomicU32::new(0),
            should_deopt: AtomicBool::new(false),
        }
    }

    /// Record a call with the given type.
    ///
    /// Returns true if this is a cache hit (same type as before).
    pub fn record_call(&self, type_id: u32) -> bool {
        let mut state = self.state.lock().unwrap();

        match &mut *state {
            InlineCacheState::Uninitialized => {
                *state = InlineCacheState::Monomorphic {
                    type_id,
                    hit_count: 1,
                };
                true
            }
            InlineCacheState::Monomorphic {
                type_id: cached_type,
                hit_count,
            } => {
                if *cached_type == type_id {
                    *hit_count += 1;
                    true
                } else {
                    // Transition to polymorphic
                    *state = InlineCacheState::Polymorphic {
                        types: vec![(*cached_type, *hit_count), (type_id, 1)],
                    };
                    false
                }
            }
            InlineCacheState::Polymorphic { types } => {
                if let Some(entry) = types.iter_mut().find(|(t, _)| *t == type_id) {
                    entry.1 += 1;
                    true
                } else if types.len() < 3 {
                    types.push((type_id, 1));
                    false
                } else {
                    // Transition to megamorphic
                    *state = InlineCacheState::Megamorphic { deopt_count: 0 };
                    false
                }
            }
            InlineCacheState::Megamorphic { deopt_count } => {
                *deopt_count += 1;
                if *deopt_count > 100 {
                    self.should_deopt.store(true, Ordering::Relaxed);
                }
                false
            }
        }
    }

    /// Get the most common type (for monomorphic optimization).
    pub fn monomorphic_type(&self) -> Option<u32> {
        let state = self.state.lock().unwrap();
        match &*state {
            InlineCacheState::Monomorphic { type_id, .. } => Some(*type_id),
            _ => None,
        }
    }

    /// Is this IC still monomorphic?
    pub fn is_monomorphic(&self) -> bool {
        matches!(
            &*self.state.lock().unwrap(),
            InlineCacheState::Monomorphic { .. }
        )
    }

    /// Is this IC in a polymorphic state?
    pub fn is_polymorphic(&self) -> bool {
        matches!(
            &*self.state.lock().unwrap(),
            InlineCacheState::Polymorphic { .. }
        )
    }

    /// Get all observed types (for polymorphic optimization).
    pub fn types(&self) -> Vec<(u32, u32)> {
        let state = self.state.lock().unwrap();
        match &*state {
            InlineCacheState::Monomorphic { type_id, hit_count } => {
                vec![(*type_id, *hit_count)]
            }
            InlineCacheState::Polymorphic { types } => types.clone(),
            _ => Vec::new(),
        }
    }
}

// ── Branch Frequency ──────────────────────────────────────────────────────────

/// Tracks branch prediction for conditional jumps.
#[derive(Debug, Clone)]
pub struct BranchFrequency {
    /// How many times was this branch taken?
    pub taken: u32,

    /// How many times was this branch not taken?
    pub not_taken: u32,
}

impl BranchFrequency {
    pub fn new() -> Self {
        Self {
            taken: 0,
            not_taken: 0,
        }
    }

    /// Record that the branch was taken.
    pub fn record_taken(&mut self) {
        self.taken += 1;
    }

    /// Record that the branch was not taken.
    pub fn record_not_taken(&mut self) {
        self.not_taken += 1;
    }

    /// What percentage of the time is this branch taken?
    pub fn taken_probability(&self) -> f64 {
        let total = (self.taken + self.not_taken) as f64;
        if total == 0.0 {
            0.5 // Unknown, assume 50/50
        } else {
            self.taken as f64 / total
        }
    }

    /// Is this branch highly biased (>95% taken)?
    pub fn is_highly_biased_taken(&self) -> bool {
        self.taken_probability() > 0.95
    }

    /// Is this branch highly biased (>95% not taken)?
    pub fn is_highly_biased_not_taken(&self) -> bool {
        self.taken_probability() < 0.05
    }
}

// ── Profile Registry ──────────────────────────────────────────────────────────

/// Registry of all function profiles, indexed by function ID.
pub struct ProfileRegistry {
    profiles: Arc<std::sync::Mutex<HashMap<u32, Arc<FunctionProfile>>>>,
}

impl ProfileRegistry {
    pub fn new() -> Self {
        Self {
            profiles: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    /// Get or create a profile for the given function.
    pub fn get_or_create(&self, func_id: u32) -> Arc<FunctionProfile> {
        let mut profiles = self.profiles.lock().unwrap();
        profiles
            .entry(func_id)
            .or_insert_with(|| Arc::new(FunctionProfile::new()))
            .clone()
    }

    /// Get a profile if it exists.
    pub fn get(&self, func_id: u32) -> Option<Arc<FunctionProfile>> {
        self.profiles.lock().unwrap().get(&func_id).cloned()
    }

    /// Get the hottest function (for sampling profiler).
    pub fn hottest(&self) -> Option<(u32, Arc<FunctionProfile>)> {
        self.profiles
            .lock()
            .unwrap()
            .iter()
            .max_by_key(|(_, profile)| {
                profile.sample_count.load(Ordering::Relaxed)
                    + profile.entry_count.load(Ordering::Relaxed) / 100
            })
            .map(|(id, profile)| (*id, profile.clone()))
    }
}

// ── Type ID System ────────────────────────────────────────────────────────────

/// Simple type ID assignment (used by inline caches).
pub struct TypeIdProvider {
    next_id: std::sync::atomic::AtomicU32,
}

impl TypeIdProvider {
    pub fn new() -> Self {
        Self {
            next_id: std::sync::atomic::AtomicU32::new(1),
        }
    }

    pub fn next_id(&self) -> u32 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }

    // Well-known type IDs
    pub fn int() -> u32 {
        0x01
    }
    pub fn float() -> u32 {
        0x02
    }
    pub fn string() -> u32 {
        0x03
    }
    pub fn list() -> u32 {
        0x04
    }
    pub fn map() -> u32 {
        0x05
    }
    pub fn function() -> u32 {
        0x06
    }
}

use std::sync::atomic::AtomicBool;
