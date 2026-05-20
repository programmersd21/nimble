//! Arena-based allocation for small objects.
//!
//! Small objects (8–256 bytes) are allocated from segregated size class pools,
//! reducing fragmentation and improving cache locality.

use std::sync::Mutex;

/// Size class for arena allocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SizeClass {
    Bytes8,   // 8 bytes
    Bytes16,  // 16 bytes
    Bytes32,  // 32 bytes
    Bytes64,  // 64 bytes
    Bytes128, // 128 bytes
    Bytes256, // 256 bytes
}

impl SizeClass {
    pub fn size(&self) -> usize {
        match self {
            SizeClass::Bytes8 => 8,
            SizeClass::Bytes16 => 16,
            SizeClass::Bytes32 => 32,
            SizeClass::Bytes64 => 64,
            SizeClass::Bytes128 => 128,
            SizeClass::Bytes256 => 256,
        }
    }

    pub fn from_size(size: usize) -> Option<Self> {
        match size {
            1..=8 => Some(SizeClass::Bytes8),
            9..=16 => Some(SizeClass::Bytes16),
            17..=32 => Some(SizeClass::Bytes32),
            33..=64 => Some(SizeClass::Bytes64),
            65..=128 => Some(SizeClass::Bytes128),
            129..=256 => Some(SizeClass::Bytes256),
            _ => None,
        }
    }
}

/// An arena pool for a single size class.
struct ArenaBin {
    /// Free list of available objects.
    freelist: Vec<*mut u8>,

    /// Total allocations from this bin.
    allocations: usize,

    /// Total deallocations to this bin.
    deallocations: usize,
}

impl ArenaBin {
    fn new() -> Self {
        Self {
            freelist: Vec::with_capacity(1024),
            allocations: 0,
            deallocations: 0,
        }
    }

    fn allocate(&mut self) -> Option<*mut u8> {
        self.freelist.pop()
    }

    fn deallocate(&mut self, ptr: *mut u8) {
        if self.freelist.len() < 10000 { // Limit freelist size
            self.freelist.push(ptr);
            self.deallocations += 1;
        }
    }

    fn live_objects(&self) -> usize {
        self.allocations - self.deallocations
    }
}

/// Arena allocator for small objects.
pub struct ArenaAllocator {
    bins: Mutex<[ArenaBin; 6]>,
}

impl ArenaAllocator {
    pub fn new() -> Self {
        Self {
            bins: Mutex::new([
                ArenaBin::new(),
                ArenaBin::new(),
                ArenaBin::new(),
                ArenaBin::new(),
                ArenaBin::new(),
                ArenaBin::new(),
            ]),
        }
    }

    /// Allocate from the appropriate size class.
    pub fn allocate(&self, size: usize) -> Option<*mut u8> {
        let size_class = SizeClass::from_size(size)?;
        let mut bins = self.bins.lock().unwrap();

        let bin_idx = match size_class {
            SizeClass::Bytes8 => 0,
            SizeClass::Bytes16 => 1,
            SizeClass::Bytes32 => 2,
            SizeClass::Bytes64 => 3,
            SizeClass::Bytes128 => 4,
            SizeClass::Bytes256 => 5,
        };

        if let Some(ptr) = bins[bin_idx].allocate() {
            return Some(ptr);
        }

        // Allocate new object
        let layout = std::alloc::Layout::from_size_align(
            size_class.size(),
            std::mem::align_of::<u64>(),
        ).unwrap();

        let ptr = unsafe { std::alloc::alloc(layout) };
        if !ptr.is_null() {
            bins[bin_idx].allocations += 1;
        }

        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    }

    /// Deallocate back to the appropriate size class.
    pub fn deallocate(&self, ptr: *mut u8, size: usize) {
        if let Some(size_class) = SizeClass::from_size(size) {
            let mut bins = self.bins.lock().unwrap();

            let bin_idx = match size_class {
                SizeClass::Bytes8 => 0,
                SizeClass::Bytes16 => 1,
                SizeClass::Bytes32 => 2,
                SizeClass::Bytes64 => 3,
                SizeClass::Bytes128 => 4,
                SizeClass::Bytes256 => 5,
            };

            bins[bin_idx].deallocate(ptr);
        }
    }

    /// Get statistics for a size class.
    pub fn stats(&self, size_class: SizeClass) -> (usize, usize, usize) {
        let bins = self.bins.lock().unwrap();
        let bin_idx = match size_class {
            SizeClass::Bytes8 => 0,
            SizeClass::Bytes16 => 1,
            SizeClass::Bytes32 => 2,
            SizeClass::Bytes64 => 3,
            SizeClass::Bytes128 => 4,
            SizeClass::Bytes256 => 5,
        };

        let bin = &bins[bin_idx];
        (bin.allocations, bin.deallocations, bin.live_objects())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_class_selection() {
        assert_eq!(SizeClass::from_size(4), Some(SizeClass::Bytes8));
        assert_eq!(SizeClass::from_size(16), Some(SizeClass::Bytes16));
        assert_eq!(SizeClass::from_size(100), Some(SizeClass::Bytes128));
        assert_eq!(SizeClass::from_size(256), Some(SizeClass::Bytes256));
        assert_eq!(SizeClass::from_size(512), None); // Too large for arena
    }

    #[test]
    fn test_arena_allocation() {
        let arena = ArenaAllocator::new();

        let ptr1 = arena.allocate(8).unwrap();
        let ptr2 = arena.allocate(32).unwrap();

        assert!(!ptr1.is_null());
        assert!(!ptr2.is_null());
    }

    #[test]
    fn test_arena_freelist_reuse() {
        let arena = ArenaAllocator::new();

        let ptr1 = arena.allocate(16).unwrap();
        arena.deallocate(ptr1, 16);

        // Next allocation should reuse freed object
        let ptr2 = arena.allocate(16);
        assert_eq!(ptr2, Some(ptr1));
    }
}
