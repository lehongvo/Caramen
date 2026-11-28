//! # Problem 0887: Compute Once with OnceLock
//!
//! - **Difficulty:** Medium
//! - **Topics:** lazy one-time initialization with `std::sync::OnceLock`, `get_or_init`, a lazily
//!   built `static`, proving "exactly once" with an atomic counter
//! - **Source:** Original · [`OnceLock`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html)
//!
//! ## Problem
//!
//! Some values are expensive to build and should be built only when first needed, **once**, even
//! if many threads ask for them at the same time. `OnceLock<T>` (stable since Rust 1.70) is a cell
//! that can be written exactly once; `get_or_init(f)` runs `f` only for the first caller, while
//! concurrent callers wait for it and then all receive a reference to the same value.
//!
//! Implement `PrimeTable`, a lazily computed list of the primes below `limit`:
//!
//! - `new(limit: u32) -> PrimeTable`: does **not** compute anything yet;
//! - `is_ready(&self) -> bool`: whether the primes have been computed;
//! - `primes(&self) -> &[u32]`: the primes below `limit` in increasing order (sieve of
//!   Eratosthenes), computed on the first call only;
//! - `builds(&self) -> usize`: how many times the sieve actually ran (must end up `1` after any
//!   number of concurrent `primes` calls, `0` before the first one).
//!
//! Also implement `shared_table() -> &'static PrimeTable`, a single process-wide table with
//! `limit = 1000`, created on first use and stored in a `static`.
//!
//! ## Examples
//!
//! ```text
//! let t = PrimeTable::new(30);
//! t.is_ready() -> false     t.builds() -> 0
//! t.primes()   -> [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
//! t.is_ready() -> true      t.builds() -> 1
//! shared_table().primes().len() -> 168
//! ```
//!
//! ## Constraints
//!
//! - `0 <= limit <= 10^6`; up to 16 threads call `primes` at the same time.
//!
//! ## Hints
//!
//! 1. Fields: `limit: u32`, `primes: OnceLock<Vec<u32>>`, `builds: AtomicUsize`.
//! 2. Increment `builds` *inside* the closure passed to `get_or_init`; it then counts real runs.
//! 3. A `static TABLE: OnceLock<PrimeTable> = OnceLock::new();` inside `shared_table` works because
//!    `OnceLock::new` is a `const fn`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `primes` calls `self.primes.get_or_init(|| { self.builds.fetch_add(1); sieve(limit) })`
//! and returns the vector as a slice. `is_ready` is `self.primes.get().is_some()`. `shared_table`
//! keeps its table in a function-local `static OnceLock` and initializes it with
//! `get_or_init(|| PrimeTable::new(1000))`.
//!
//! **Rust concepts:**
//! - `OnceLock<T>` is `Sync` when `T: Send + Sync`, so a shared `&PrimeTable` can be used from many
//!   threads; the returned `&[u32]` borrows from `self` and needs no lock afterwards.
//! - Unlike `Mutex`, reading an initialized `OnceLock` costs just an atomic load.
//! - `std::sync::LazyLock` (Rust 1.80) packages the same idea with the initializer stored inside.
// @solution-end

use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};

/// The primes below a limit, computed on first use.
pub struct PrimeTable {
    // @solution-start remove
    limit: u32,
    primes: OnceLock<Vec<u32>>,
    builds: AtomicUsize,
    // @solution-end
}

// @solution-start remove
fn sieve(limit: u32) -> Vec<u32> {
    let n = limit as usize;
    let mut composite = vec![false; n];
    let mut primes = Vec::new();
    for i in 2..n {
        if composite[i] {
            continue;
        }
        primes.push(i as u32);
        for multiple in (i * i..n).step_by(i) {
            composite[multiple] = true;
        }
    }
    primes
}
// @solution-end

impl PrimeTable {
    pub fn new(limit: u32) -> Self {
        // @solution-start
        Self {
            limit,
            primes: OnceLock::new(),
            builds: AtomicUsize::new(0),
        }
        // @solution-end
    }

    pub fn is_ready(&self) -> bool {
        // @solution-start
        self.primes.get().is_some()
        // @solution-end
    }

    pub fn primes(&self) -> &[u32] {
        // @solution-start
        self.primes.get_or_init(|| {
            self.builds.fetch_add(1, Ordering::Relaxed);
            sieve(self.limit)
        })
        // @solution-end
    }

    pub fn builds(&self) -> usize {
        // @solution-start
        self.builds.load(Ordering::Relaxed)
        // @solution-end
    }
}

pub fn shared_table() -> &'static PrimeTable {
    // @solution-start
    static TABLE: OnceLock<PrimeTable> = OnceLock::new();
    TABLE.get_or_init(|| PrimeTable::new(1000))
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn example_lazy_table() {
        let t = PrimeTable::new(30);
        assert!(!t.is_ready());
        assert_eq!(t.builds(), 0);
        assert_eq!(t.primes(), &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
        assert!(t.is_ready());
        assert_eq!(t.builds(), 1);
    }

    #[test]
    fn example_shared_table() {
        assert_eq!(shared_table().primes().len(), 168);
        assert!(std::ptr::eq(shared_table(), shared_table()));
    }

    #[test]
    fn repeated_calls_reuse_the_same_vector() {
        let t = PrimeTable::new(100);
        let first = t.primes().as_ptr();
        assert_eq!(t.primes().as_ptr(), first);
        assert_eq!(t.primes().len(), 25);
        assert_eq!(t.builds(), 1);
    }

    #[test]
    fn tiny_limits() {
        assert!(PrimeTable::new(0).primes().is_empty());
        assert!(PrimeTable::new(2).primes().is_empty());
        assert_eq!(PrimeTable::new(3).primes(), &[2]);
    }

    #[test]
    fn concurrent_first_use_builds_once() {
        let t = PrimeTable::new(1_000_000);
        let lens: Vec<usize> = thread::scope(|s| {
            let handles: Vec<_> = (0..16).map(|_| s.spawn(|| t.primes().len())).collect();
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });
        assert!(lens.iter().all(|&len| len == 78_498));
        assert_eq!(t.builds(), 1);
    }
}
