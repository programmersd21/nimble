//! Inline Cache (IC) system.
//!
//! Tracks call sites and polymorphic call targets to enable devirtualization
//! and bypass slow hash-table lookups for method/field access.

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum ICState {
    Uninitialized,
    Monomorphic { receiver_type: u32, target: u64 },
    Polymorphic { targets: HashMap<u32, u64> },
    Megamorphic,
}

pub struct InlineCache {
    pub state: Arc<Mutex<ICState>>,
}

impl InlineCache {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ICState::Uninitialized)),
        }
    }

    /// Record a call at this site.
    pub fn record(&self, receiver_type: u32, target: u64) {
        let mut state = self.state.lock().unwrap();
        match &mut *state {
            ICState::Uninitialized => {
                *state = ICState::Monomorphic { receiver_type, target };
            }
            ICState::Monomorphic { receiver_type: old_type, target: old_target } => {
                if *old_type == receiver_type {
                    // Same target, do nothing
                } else {
                    // Transition to polymorphic
                    let mut targets = HashMap::new();
                    targets.insert(*old_type, *old_target);
                    targets.insert(receiver_type, target);
                    *state = ICState::Polymorphic { targets };
                }
            }
            ICState::Polymorphic { targets } => {
                if targets.len() >= 4 {
                    *state = ICState::Megamorphic;
                } else {
                    targets.insert(receiver_type, target);
                }
            }
            ICState::Megamorphic => {}
        }
    }
}
