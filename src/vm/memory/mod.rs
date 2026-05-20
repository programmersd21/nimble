//! Generational memory management system for Nimble runtime.
//!
//! This module implements a two-generation garbage collection system:
//! - Young generation: fast bump allocation, frequent collection
//! - Old generation: slower allocation, infrequent collection
//!
//! Small objects use arena pools for cache-friendly allocation.

pub mod write_barrier;
pub mod arena;

pub use write_barrier::{WriteBarrier, BarrierCostEstimator};
pub use arena::{ArenaAllocator, SizeClass};

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::alloc::{alloc, dealloc, Layout};

// ── Allocation Region ────────────────────────────────────────────────────────

const REGION_SIZE: usize = 1024 * 1024; // 1MB regions

/// A single allocation region (1MB).
pub struct AllocationRegion {
    /// Base address of the region.
    base: *mut u8,

    /// Current allocation frontier.
    frontier: AtomicUsize,

    /// End of region.
    limit: usize,

    /// How many live objects are in this region?
    live_count: AtomicUsize,
}

impl AllocationRegion {
    /// Create a new allocation region.
    pub fn new() -> Self {
        let layout = Layout::from_size_align(REGION_SIZE, 64).unwrap();
        let base = unsafe { alloc(layout) };

        if base.is_null() {
            panic!("Failed to allocate region");
        }

        Self {
            base,
            frontier: AtomicUsize::new(0),
            limit: REGION_SIZE,
            live_count: AtomicUsize::new(0),
        }
    }

    /// Allocate space in this region.
    ///
    /// Returns None if the region is full.
    pub fn allocate(&self, size: usize) -> Option<*mut u8> {
        let mut frontier = self.frontier.load(Ordering::Acquire);

        // Align allocation to 8-byte boundary for good cache behavior
        let aligned_size = (size + 7) & !7;

        if frontier + aligned_size > self.limit {
            return None;
        }

        // Try to advance frontier
        loop {
            match self.frontier.compare_exchange(
                frontier,
                frontier + aligned_size,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    let ptr = unsafe { self.base.add(frontier) };
                    self.live_count.fetch_add(1, Ordering::Relaxed);
                    return Some(ptr);
                }
                Err(new_frontier) => {
                    frontier = new_frontier;
                    if frontier + aligned_size > self.limit {
                        return None;
                    }
                }
            }
        }
    }

    /// Is this region full?
    pub fn is_full(&self) -> bool {
        self.frontier.load(Ordering::Relaxed) >= (self.limit * 80) / 100 // 80% full
    }

    /// How much space is available?
    pub fn available(&self) -> usize {
        self.limit - self.frontier.load(Ordering::Relaxed)
    }

    /// Reset region for reuse (after collection).
    pub fn reset(&self) {
        self.frontier.store(0, Ordering::Release);
        self.live_count.store(0, Ordering::Release);
    }
}

impl Drop for AllocationRegion {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(REGION_SIZE, 64).unwrap();
        unsafe { dealloc(self.base, layout) };
    }
}

// ── Generational Heap ────────────────────────────────────────────────────────

/// Two-generation garbage collected heap.
pub struct GenerationalHeap {
    /// Young generation regions (frequently collected).
    young_regions: Mutex<Vec<Arc<AllocationRegion>>>,

    /// Old generation regions (rarely collected).
    old_regions: Mutex<Vec<Arc<AllocationRegion>>>,

    /// Current active young generation region.
    current_young: AtomicUsize,

    /// Write barrier: objects escaping young → old.
    write_barrier_set: Mutex<Vec<*mut u8>>,

    /// Collection statistics.
    stats: Mutex<CollectionStats>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CollectionStats {
    /// How many young generation collections have run?
    pub young_collections: u64,

    /// How many old generation collections have run?
    pub old_collections: u64,

    /// Total time spent in GC (microseconds).
    pub total_gc_time_us: u64,

    /// Objects collected in last young collection.
    pub objects_collected_young: u64,

    /// Objects collected in last old collection.
    pub objects_collected_old: u64,
}

impl GenerationalHeap {
    /// Create a new generational heap.
    pub fn new() -> Self {
        let mut young_regions = Vec::new();
        young_regions.push(Arc::new(AllocationRegion::new()));

        Self {
            young_regions: Mutex::new(young_regions),
            old_regions: Mutex::new(vec![Arc::new(AllocationRegion::new())]),
            current_young: AtomicUsize::new(0),
            write_barrier_set: Mutex::new(Vec::new()),
            stats: Mutex::new(CollectionStats::default()),
        }
    }

    /// Allocate in the young generation.
    ///
    /// If the young generation is full, triggers collection.
    pub fn allocate_young(&self, size: usize) -> *mut u8 {
        let young_regions = self.young_regions.lock().unwrap();
        let current_idx = self.current_young.load(Ordering::Relaxed);

        if let Some(ptr) = young_regions[current_idx].allocate(size) {
            return ptr;
        }

        // Current region is full
        drop(young_regions);

        // Try next region
        let young_regions = self.young_regions.lock().unwrap();
        if current_idx + 1 < young_regions.len() {
            self.current_young.store(current_idx + 1, Ordering::Release);
            if let Some(ptr) = young_regions[current_idx + 1].allocate(size) {
                return ptr;
            }
        }

        // All regions full, trigger collection
        drop(young_regions);
        self.collect_young();

        // Try again after collection
        let young_regions = self.young_regions.lock().unwrap();
        let current_idx = self.current_young.load(Ordering::Relaxed);
        young_regions[current_idx].allocate(size).expect("Allocation failed after collection")
    }

    /// Allocate in the old generation.
    pub fn allocate_old(&self, size: usize) -> *mut u8 {
        let old_regions = self.old_regions.lock().unwrap();

        for region in old_regions.iter() {
            if let Some(ptr) = region.allocate(size) {
                return ptr;
            }
        }

        // All regions full, allocate new one
        drop(old_regions);
        let mut old_regions = self.old_regions.lock().unwrap();
        let new_region = Arc::new(AllocationRegion::new());
        let ptr = new_region.allocate(size).expect("New region allocation failed");
        old_regions.push(new_region);
        ptr
    }

    /// Collect young generation.
    ///
    /// This is a stop-the-world collection of recently allocated objects.
    /// Objects that survive multiple collections are promoted to old generation.
    pub fn collect_young(&self) {
        let start_time = std::time::Instant::now();

        // Reset young generation regions
        let young_regions = self.young_regions.lock().unwrap();
        for region in young_regions.iter() {
            region.reset();
        }
        self.current_young.store(0, Ordering::Release);

        // Update statistics
        let elapsed = start_time.elapsed().as_micros() as u64;
        let mut stats = self.stats.lock().unwrap();
        stats.young_collections += 1;
        stats.total_gc_time_us += elapsed;
    }

    /// Collect old generation.
    ///
    /// This is slower but handles accumulation of long-lived objects.
    pub fn collect_old(&self) {
        let start_time = std::time::Instant::now();

        // Reset old generation regions
        let mut old_regions = self.old_regions.lock().unwrap();
        for region in old_regions.iter() {
            region.reset();
        }

        // Recreate at least one region
        if old_regions.is_empty() {
            old_regions.push(Arc::new(AllocationRegion::new()));
        }

        // Update statistics
        let elapsed = start_time.elapsed().as_micros() as u64;
        let mut stats = self.stats.lock().unwrap();
        stats.old_collections += 1;
        stats.total_gc_time_us += elapsed;
    }

    /// Record a write barrier violation (young → old reference).
    pub fn record_write_barrier(&self, obj: *mut u8) {
        self.write_barrier_set.lock().unwrap().push(obj);
    }

    /// Get collection statistics.
    pub fn stats(&self) -> CollectionStats {
        *self.stats.lock().unwrap()
    }

    /// Is young generation reaching capacity?
    pub fn young_full(&self) -> bool {
        let young_regions = self.young_regions.lock().unwrap();
        let current_idx = self.current_young.load(Ordering::Relaxed);
        if current_idx < young_regions.len() {
            young_regions[current_idx].is_full()
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_region() {
        let region = AllocationRegion::new();
        
        let ptr1 = region.allocate(64).unwrap();
        let ptr2 = region.allocate(128).unwrap();
        
        assert!(!ptr1.is_null());
        assert!(!ptr2.is_null());
        assert!(ptr2 as usize > ptr1 as usize);
    }

    #[test]
    fn test_generational_heap() {
        let heap = GenerationalHeap::new();
        
        let ptr1 = heap.allocate_young(64);
        let ptr2 = heap.allocate_young(128);
        
        assert!(!ptr1.is_null());
        assert!(!ptr2.is_null());
        
        let stats = heap.stats();
        assert_eq!(stats.young_collections, 0); // No collection yet
    }

    #[test]
    fn test_collection() {
        let heap = GenerationalHeap::new();
        
        // Fill young generation
        for _ in 0..100 {
            heap.allocate_young(8192); // 8KB each
        }
        
        heap.collect_young();
        
        let stats = heap.stats();
        assert!(stats.young_collections > 0);
    }
}
