//! # Problem 0876: Make It Send and Sync
//!
//! - **Difficulty:** Medium
//! - **Topics:** the `Send` and `Sync` marker traits, why `Rc<RefCell<T>>` cannot cross threads,
//!   `Arc<Mutex<T>>` as its thread-safe counterpart, cheap `Clone` handles
//! - **Source:** Original · [The Rust Book 16.4](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html) ·
//!   [`Send`](https://doc.rust-lang.org/std/marker/trait.Send.html) ·
//!   [`Sync`](https://doc.rust-lang.org/std/marker/trait.Sync.html)
//!
//! ## Problem
//!
//! A single-threaded word counter was written like this:
//!
//! ```text
//! #[derive(Clone, Default)]
//! pub struct SharedTally {
//!     counts: Rc<RefCell<HashMap<String, u32>>>,   // clones share the same map
//! }
//! ```
//!
//! Moving a clone into `thread::spawn` fails to compile with error `E0277`:
//! "`Rc<RefCell<HashMap<String, u32>>>` cannot be sent between threads safely". `Rc` updates its
//! reference count without atomic instructions, and `RefCell` checks borrows without locking, so
//! neither is safe to share between threads.
//!
//! Rewrite `SharedTally` so that it is `Clone + Send + Sync + 'static`, clones still share the same
//! counts, and it works from many threads at once:
//!
//! - `new() -> SharedTally`: an empty tally;
//! - `add(&self, word: &str)`: count one more occurrence of `word`;
//! - `count(&self, word: &str) -> u32`: occurrences of `word` (`0` if never added);
//! - `total(&self) -> u32`: occurrences of all words together;
//! - `handles(&self) -> usize`: how many `SharedTally` values (this one and its clones) currently
//!   share the same counts.
//!
//! ## Examples
//!
//! ```text
//! let a = SharedTally::new();
//! let b = a.clone();
//! a.add("x"); b.add("x"); b.add("y");
//! a.count("x") -> 2    a.count("z") -> 0    a.total() -> 3    a.handles() -> 2
//! ```
//!
//! ## Constraints
//!
//! - Up to 16 threads; counts fit in a `u32`.
//!
//! ## Hints
//!
//! 1. `Arc` is the thread-safe `Rc`; `Mutex` is the thread-safe `RefCell` (locking instead of
//!    borrow flags).
//! 2. `Send` and `Sync` are implemented automatically: a struct is `Send`/`Sync` when all its fields
//!    are. You never write `unsafe impl Send` for this.
//! 3. `Arc::strong_count(&self.counts)` tells how many handles exist.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** replace `Rc<RefCell<HashMap<..>>>` with `Arc<Mutex<HashMap<..>>>`. Every method takes
//! `&self`, locks the mutex for the shortest possible time and works on the map. Deriving `Clone`
//! clones the `Arc`, so all clones share one map.
//!
//! **Rust concepts:**
//! - `T: Send` means a `T` can be moved to another thread; `T: Sync` means `&T` can be shared
//!   between threads (`T: Sync` exactly when `&T: Send`).
//! - `Arc<T>` is `Send + Sync` when `T: Send + Sync`; `Mutex<T>` is `Sync` when `T: Send`. So
//!   `Arc<Mutex<HashMap<String, u32>>>` is both, and the compiler derives it for `SharedTally`.
//! - Interior mutability through `&self` (`lock()` instead of `borrow_mut()`) is what lets many
//!   threads hold shared references and still update the data.
// @solution-end

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A word counter whose clones share the same counts, usable from many threads.
#[derive(Clone, Default)]
pub struct SharedTally {
    // @solution-start remove
    counts: Arc<Mutex<HashMap<String, u32>>>,
    // @solution-end
}

impl SharedTally {
    pub fn new() -> Self {
        // @solution-start
        Self::default()
        // @solution-end
    }

    pub fn add(&self, word: &str) {
        // @solution-start
        let mut counts = self
            .counts
            .lock()
            .expect("no thread panics while holding the lock");
        *counts.entry(word.to_string()).or_insert(0) += 1;
        // @solution-end
    }

    pub fn count(&self, word: &str) -> u32 {
        // @solution-start
        let counts = self
            .counts
            .lock()
            .expect("no thread panics while holding the lock");
        counts.get(word).copied().unwrap_or(0)
        // @solution-end
    }

    pub fn total(&self) -> u32 {
        // @solution-start
        let counts = self
            .counts
            .lock()
            .expect("no thread panics while holding the lock");
        counts.values().sum()
        // @solution-end
    }

    pub fn handles(&self) -> usize {
        // @solution-start
        Arc::strong_count(&self.counts)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    fn assert_thread_safe<T: Clone + Send + Sync + 'static>(_: &T) {}

    #[test]
    fn example_two_handles() {
        let a = SharedTally::new();
        let b = a.clone();
        a.add("x");
        b.add("x");
        b.add("y");
        assert_eq!(a.count("x"), 2);
        assert_eq!(a.count("z"), 0);
        assert_eq!(a.total(), 3);
        assert_eq!(a.handles(), 2);
    }

    #[test]
    fn empty_tally() {
        let t = SharedTally::new();
        assert_thread_safe(&t);
        assert_eq!(t.total(), 0);
        assert_eq!(t.handles(), 1);
    }

    #[test]
    fn clones_dropped_by_finished_threads() {
        let t = SharedTally::new();
        let worker = {
            let t = t.clone();
            thread::spawn(move || t.add("hello"))
        };
        worker.join().unwrap();
        assert_eq!(t.count("hello"), 1);
        assert_eq!(t.handles(), 1);
    }

    #[test]
    fn many_threads_share_one_tally() {
        let t = SharedTally::new();
        let workers: Vec<_> = (0..16)
            .map(|i| {
                let t = t.clone();
                thread::spawn(move || {
                    for _ in 0..1000 {
                        t.add("common");
                    }
                    t.add(&format!("thread-{i}"));
                })
            })
            .collect();
        for w in workers {
            w.join().unwrap();
        }
        assert_eq!(t.count("common"), 16_000);
        assert_eq!(t.count("thread-7"), 1);
        assert_eq!(t.total(), 16_016);
    }
}
