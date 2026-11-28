// ==================================================================
// PRACTICE 0883: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0883_atomic_statistics.md
// Run the tests:          cargo test practice::p0883
// ==================================================================

//! # Problem 0883: Atomic Statistics
//!
//! - **Difficulty:** Easy
//! - **Topics:** `AtomicU64` with `fetch_add`, `fetch_max` and `load`, `Ordering::Relaxed`, shared
//!   state without locks through `&self`
//! - **Source:** Original · [`std::sync::atomic`](https://doc.rust-lang.org/std/sync/atomic/index.html)
//!
//! ## Problem
//!
//! For a simple counter, a `Mutex` is more machinery than needed: atomic integers can be updated by
//! many threads at once, without locking, through a shared reference.
//!
//! Implement `AtomicStats`, a statistics collector that many threads can update concurrently:
//!
//! - `new() -> AtomicStats`: nothing recorded yet;
//! - `record(&self, value: u64)`: record one value;
//! - `count(&self) -> u64`: how many values were recorded;
//! - `sum(&self) -> u64`: their sum (it is guaranteed to fit in a `u64`);
//! - `max(&self) -> Option<u64>`: the largest value, or `None` if nothing was recorded.
//!
//! Use one atomic per statistic and no locks. The getters are only required to be exact once all
//! the recording threads have finished.
//!
//! ## Examples
//!
//! ```text
//! let stats = AtomicStats::new();
//! stats.max()   -> None
//! stats.record(4); stats.record(10); stats.record(1);
//! stats.count() -> 3    stats.sum() -> 15    stats.max() -> Some(10)
//! ```
//!
//! ## Constraints
//!
//! - Up to 16 threads recording up to `10^5` values each.
//!
//! ## Hints
//!
//! 1. `AtomicU64::fetch_add(v, Ordering::Relaxed)` adds atomically and returns the old value.
//! 2. `fetch_max` (stable since Rust 1.45) stores the maximum of the old and the new value.
//! 3. Each statistic is independent and nothing else is published through it, so `Relaxed` is
//!    enough: joining the threads already makes all their writes visible.

use std::sync::atomic::{AtomicU64, Ordering};

/// Lock-free statistics over recorded `u64` values.
pub struct AtomicStats {}

impl AtomicStats {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn record(&self, value: u64) {
        todo!("Write your solution here")
    }

    pub fn count(&self) -> u64 {
        todo!("Write your solution here")
    }

    pub fn sum(&self) -> u64 {
        todo!("Write your solution here")
    }

    pub fn max(&self) -> Option<u64> {
        todo!("Write your solution here")
    }
}

impl Default for AtomicStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn example_three_values() {
        let stats = AtomicStats::new();
        assert_eq!(stats.max(), None);
        stats.record(4);
        stats.record(10);
        stats.record(1);
        assert_eq!((stats.count(), stats.sum(), stats.max()), (3, 15, Some(10)));
    }

    #[test]
    fn zero_is_a_valid_maximum() {
        let stats = AtomicStats::default();
        stats.record(0);
        assert_eq!(stats.max(), Some(0));
        assert_eq!(stats.count(), 1);
    }

    #[test]
    fn scoped_threads_share_a_reference() {
        let stats = AtomicStats::new();
        thread::scope(|s| {
            for _ in 0..8 {
                s.spawn(|| {
                    for v in 1..=1000 {
                        stats.record(v);
                    }
                });
            }
        });
        assert_eq!(stats.count(), 8000);
        assert_eq!(stats.sum(), 8 * 500_500);
        assert_eq!(stats.max(), Some(1000));
    }

    #[test]
    fn arc_shared_across_spawned_threads() {
        let stats = Arc::new(AtomicStats::new());
        let handles: Vec<_> = (0..16_u64)
            .map(|t| {
                let stats = Arc::clone(&stats);
                thread::spawn(move || {
                    for i in 0..100_000 {
                        stats.record(t * 100_000 + i);
                    }
                })
            })
            .collect();
        for h in handles {
            h.join().unwrap();
        }
        let n = 1_600_000_u64;
        assert_eq!(stats.count(), n);
        assert_eq!(stats.sum(), n * (n - 1) / 2);
        assert_eq!(stats.max(), Some(n - 1));
    }
}
