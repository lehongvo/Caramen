// ==================================================================
// PRACTICE 0876: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0876_make_it_send_and_sync.md
// Run the tests:          cargo test practice::p0876
// ==================================================================

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

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A word counter whose clones share the same counts, usable from many threads.
#[derive(Clone, Default)]
pub struct SharedTally {}

impl SharedTally {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn add(&self, word: &str) {
        todo!("Write your solution here")
    }

    pub fn count(&self, word: &str) -> u32 {
        todo!("Write your solution here")
    }

    pub fn total(&self) -> u32 {
        todo!("Write your solution here")
    }

    pub fn handles(&self) -> usize {
        todo!("Write your solution here")
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
