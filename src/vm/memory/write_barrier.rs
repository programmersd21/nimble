//! Write barrier system for generational garbage collection.
//!
//! Write barriers detect when young generation objects are stored in old generation
//! objects, allowing efficient young-generation-only collections.

use std::sync::Mutex;

/// A write barrier that tracks old→young references.
pub struct WriteBarrier {
    /// Set of old generation objects that contain young generation references.
    remembered_set: Mutex<Vec<*mut u8>>,
}

impl WriteBarrier {
    pub fn new() -> Self {
        Self {
            remembered_set: Mutex::new(Vec::new()),
        }
    }

    /// Record that an old object stores a reference to a young object.
    ///
    /// This is called by generated code after every store operation where:
    /// - The object being stored into is in old generation
    /// - The value being stored is in young generation
    pub fn record_barrier(&self, old_obj: *mut u8) {
        let mut set = self.remembered_set.lock().unwrap();

        // Avoid duplicates
        if !set.contains(&old_obj) {
            set.push(old_obj);
        }
    }

    /// Get all objects in the remembered set.
    pub fn get_remembered_set(&self) -> Vec<*mut u8> {
        self.remembered_set.lock().unwrap().clone()
    }

    /// Clear the remembered set after a young generation collection.
    pub fn clear(&self) {
        self.remembered_set.lock().unwrap().clear();
    }

    /// How many objects are in the remembered set?
    pub fn size(&self) -> usize {
        self.remembered_set.lock().unwrap().len()
    }
}

// ── Barrier Cost Estimation ──────────────────────────────────────────────────

/// Estimates the cost of write barriers in generated code.
pub struct BarrierCostEstimator;

impl BarrierCostEstimator {
    /// Estimated cost of write barrier in CPU cycles.
    ///
    /// A write barrier in JIT-generated code consists of:
    /// - Check if old object is in old generation (~1 cycle)
    /// - Check if value is in young generation (~1 cycle)
    /// - Conditional branch (~1 cycle)
    /// - Add to remembered set (~2 cycles, worst case)
    /// Total: ~5 cycles per barrier in hot path (3 when taken, 2 when not)
    pub const COST_CYCLES: usize = 3;

    /// Is write barrier cost justified?
    ///
    /// Barriers are profitable when:
    /// - Young generation collection is very frequent
    /// - Most stores are within generation (not across)
    pub fn is_profitable(young_collection_frequency_per_sec: f64) -> bool {
        // If we collect young generation >10 times per second, barriers pay for themselves
        young_collection_frequency_per_sec > 10.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_barrier() {
        let barrier = WriteBarrier::new();

        let obj1 = 0x1000 as *mut u8;
        let obj2 = 0x2000 as *mut u8;

        barrier.record_barrier(obj1);
        barrier.record_barrier(obj2);
        barrier.record_barrier(obj1); // Duplicate, shouldn't increase size

        assert_eq!(barrier.size(), 2);

        let set = barrier.get_remembered_set();
        assert!(set.contains(&obj1));
        assert!(set.contains(&obj2));

        barrier.clear();
        assert_eq!(barrier.size(), 0);
    }
}
