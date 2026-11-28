//! # Problem 0886: Lock-Free Maximum
//!
//! - **Difficulty:** Medium
//! - **Topics:** compare-and-swap loops with `compare_exchange_weak`, retrying with the value
//!   returned on failure, `swap`
//! - **Source:** Original · [`std::sync::atomic`](https://doc.rust-lang.org/std/sync/atomic/index.html)
//!
//! ## Problem
//!
//! `fetch_add` and `fetch_max` are ready-made atomic read-modify-write operations. Any other update
//! can be built from **compare-and-swap** (CAS): read the current value, compute the new one, and
//! store it only if nobody changed the value in the meantime; otherwise retry with the fresh value.
//!
//! Implement `AtomicMax`, a shared maximum, **without** `fetch_max` (use a CAS loop):
//!
//! - `new(initial: u64) -> AtomicMax`;
//! - `update(&self, candidate: u64) -> bool`: if `candidate` is **greater** than the current value,
//!   store it and return `true`; otherwise leave the value unchanged and return `false`. Even when
//!   many threads call `update` at once, the stored value never decreases and every candidate is
//!   taken into account;
//! - `get(&self) -> u64`: the current value;
//! - `reset(&self, value: u64) -> u64`: unconditionally replace the value, returning the old one.
//!
//! ## Examples
//!
//! ```text
//! let m = AtomicMax::new(5);
//! m.update(3) -> false     m.update(7) -> true     m.update(7) -> false
//! m.get()     -> 7
//! m.reset(1)  -> 7         m.get()     -> 1
//! ```
//!
//! ## Constraints
//!
//! - Up to 16 threads calling `update` concurrently.
//!
//! ## Hints
//!
//! 1. `compare_exchange_weak(current, new, success_ordering, failure_ordering)` returns `Ok(old)` if
//!    the value was `current` and is now `new`, or `Err(actual)` with the value it found instead.
//! 2. On `Err(actual)`, do not reload: `actual` *is* the fresh value. Loop while `candidate > current`.
//! 3. The `_weak` version may fail spuriously (even if the value matched); that is harmless inside a
//!    loop and can be faster on some processors.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** load the current value; while `candidate > current`, try
//! `compare_exchange_weak(current, candidate)`. Success means this call raised the maximum. Failure
//! hands back the value another thread stored; if it is already `>= candidate`, give up with
//! `false`, otherwise retry. `reset` is a plain atomic `swap`.
//!
//! **Rust concepts:**
//! - CAS loops are the basic building block of lock-free code; `AtomicU64::fetch_update` wraps the
//!   same loop around a closure.
//! - `Ordering::Relaxed` suffices because the maximum does not publish other data; code that does
//!   publish data through an atomic uses `Release` on the store and `Acquire` on the load.
//! - The loop always terminates: each failure means the value grew, and it can only grow up to
//!   `candidate`.
// @solution-end

use std::sync::atomic::{AtomicU64, Ordering};

/// A maximum that many threads can update without locks.
pub struct AtomicMax {
    // @solution-start remove
    value: AtomicU64,
    // @solution-end
}

impl AtomicMax {
    pub fn new(initial: u64) -> Self {
        // @solution-start
        Self {
            value: AtomicU64::new(initial),
        }
        // @solution-end
    }

    pub fn update(&self, candidate: u64) -> bool {
        // @solution-start
        let mut current = self.value.load(Ordering::Relaxed);
        while candidate > current {
            match self.value.compare_exchange_weak(
                current,
                candidate,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return true,
                Err(actual) => current = actual,
            }
        }
        false
        // @solution-end
    }

    pub fn get(&self) -> u64 {
        // @solution-start
        self.value.load(Ordering::Relaxed)
        // @solution-end
    }

    pub fn reset(&self, value: u64) -> u64 {
        // @solution-start
        self.value.swap(value, Ordering::Relaxed)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn example_sequence() {
        let m = AtomicMax::new(5);
        assert!(!m.update(3));
        assert!(m.update(7));
        assert!(!m.update(7));
        assert_eq!(m.get(), 7);
        assert_eq!(m.reset(1), 7);
        assert_eq!(m.get(), 1);
    }

    #[test]
    fn update_results_in_order() {
        let m = AtomicMax::new(0);
        let results: Vec<bool> = [5, 3, 7, 7, 2, 8].iter().map(|&v| m.update(v)).collect();
        assert_eq!(results, vec![true, false, true, false, false, true]);
    }

    #[test]
    fn extreme_values() {
        let m = AtomicMax::new(u64::MAX);
        assert!(!m.update(u64::MAX));
        assert_eq!(m.reset(0), u64::MAX);
        assert!(!m.update(0));
        assert!(m.update(u64::MAX));
    }

    #[test]
    fn concurrent_updates_never_lose_the_maximum() {
        let m = AtomicMax::new(0);
        let raised: usize = thread::scope(|s| {
            let handles: Vec<_> = (0..16_u64)
                .map(|t| {
                    let m = &m;
                    s.spawn(move || (0..50_000).filter(|i| m.update(i * 16 + t)).count())
                })
                .collect();
            handles.into_iter().map(|h| h.join().unwrap()).sum()
        });
        assert_eq!(m.get(), 49_999 * 16 + 15);
        // At least one update raised the value, and never more than the number of calls.
        assert!((1..=800_000).contains(&raised));
    }
}
