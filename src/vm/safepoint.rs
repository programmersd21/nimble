//! Safepoint system for stop-the-world GC coordination.
//!
//! Safepoints are explicit checkpoints where:
//! - Object pointers can't change
//! - All threads can safely be paused for GC
//! - Code cache invalidation can be applied

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

// ── Safepoint Request ─────────────────────────────────────────────────────────

/// Global safepoint request state.
///
/// The runtime sets this flag when it needs all executing threads to reach
/// a safepoint and pause. Threads check this flag periodically and stop
/// execution when they see it set.
pub struct SafepointRequest {
    /// Has a safepoint been requested?
    pub requested: AtomicBool,

    /// What generation is this request? (for ABA detection)
    pub generation: AtomicU64,

    /// How many threads have reached the safepoint?
    pub threads_at_safepoint: AtomicU64,

    /// How many threads are expected?
    pub expected_threads: u64,
}

impl SafepointRequest {
    pub fn new(expected_threads: u64) -> Self {
        Self {
            requested: AtomicBool::new(false),
            generation: AtomicU64::new(0),
            threads_at_safepoint: AtomicU64::new(0),
            expected_threads,
        }
    }

    /// Request a safepoint.
    pub fn request(&self) {
        self.requested.store(true, Ordering::Release);
    }

    /// Have all threads reached the safepoint?
    pub fn all_threads_safe(&self) -> bool {
        self.threads_at_safepoint.load(Ordering::Acquire) >= self.expected_threads
    }

    /// Wait for all threads to reach safepoint (blocking).
    pub fn wait_for_all(&self, timeout_ms: u64) {
        let start = std::time::Instant::now();
        while !self.all_threads_safe() {
            if start.elapsed().as_millis() > timeout_ms as u128 {
                eprintln!("Safepoint timeout: not all threads reached safepoint");
                break;
            }
            std::thread::yield_now();
        }
    }

    /// Reset after safepoint is complete.
    pub fn reset(&self) {
        self.requested.store(false, Ordering::Release);
        self.threads_at_safepoint.store(0, Ordering::Release);
        self.generation.fetch_add(1, Ordering::Release);
    }
}

// ── Safepoint Coordinator ─────────────────────────────────────────────────────

/// Coordinates safepoint requests and ensures all threads stop safely.
#[derive(Clone)]
pub struct SafepointCoordinator {
    request: Arc<SafepointRequest>,
}

impl SafepointCoordinator {
    pub fn new(expected_threads: u64) -> Self {
        Self {
            request: Arc::new(SafepointRequest::new(expected_threads)),
        }
    }

    /// Request a global safepoint.
    pub fn request_safepoint(&self) {
        self.request.request();
    }

    /// Wait for all threads to reach the safepoint.
    pub fn wait_for_safepoint(&self, timeout_ms: u64) {
        self.request.wait_for_all(timeout_ms);
    }

    /// Done with safepoint, resume all threads.
    pub fn resume_from_safepoint(&self) {
        self.request.reset();
    }

    /// Get a clone of the request for use by a thread.
    pub fn request_clone(&self) -> Arc<SafepointRequest> {
        self.request.clone()
    }
}

// ── Safepoint Check ───────────────────────────────────────────────────────────

/// Per-thread safepoint check.
///
/// Called periodically (e.g., at loop backedges) to check if a safepoint
/// has been requested and pause execution if so.
pub struct SafepointCheck {
    request: Arc<SafepointRequest>,
}

impl SafepointCheck {
    pub fn new(request: Arc<SafepointRequest>) -> Self {
        Self { request }
    }

    /// Check if safepoint is requested; if so, pause until resumed.
    ///
    /// This should be called at:
    /// - Loop backedges
    /// - Function entries
    /// - Allocation points
    pub fn check(&self) {
        if self.request.requested.load(Ordering::Acquire) {
            self.wait_at_safepoint();
        }
    }

    /// Pause at safepoint until coordinator resumes execution.
    fn wait_at_safepoint(&self) {
        // Announce arrival at safepoint
        self.request.threads_at_safepoint.fetch_add(1, Ordering::Release);

        // Wait until safepoint is cleared
        while self.request.requested.load(Ordering::Acquire) {
            std::thread::yield_now();
        }

        // Decrement counter as we leave
        self.request.threads_at_safepoint.fetch_sub(1, Ordering::Release);
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_safepoint_coordination() {
        let coordinator = Arc::new(SafepointCoordinator::new(2));

        let mut handles = Vec::new();

        // Spawn two worker threads
        for _id in 0..2 {
            let coord = coordinator.clone();
            let handle = thread::spawn(move || {
                let request = coord.request_clone();
                let check = SafepointCheck::new(request);

                // Simulate some work
                for _ in 0..100 {
                    check.check();
                    thread::yield_now();
                }
            });
            handles.push(handle);
        }

        // Request safepoint
        coordinator.request_safepoint();
        coordinator.wait_for_safepoint(1000);

        // All threads should be paused
        assert_eq!(
            coordinator.request_clone().threads_at_safepoint.load(Ordering::Acquire),
            2
        );

        // Resume
        coordinator.resume_from_safepoint();

        // Wait for threads to finish
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
