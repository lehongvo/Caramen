//! # Problem 0892: Print in Order
//!
//! - **Difficulty:** Easy
//! - **Topics:** ordering work across threads with a `Mutex<u8>` "turn" and a `Condvar`, callbacks
//!   passed as `impl FnOnce()`
//! - **Source:** Adapted from [LeetCode 1114. Print in Order](https://leetcode.com/problems/print-in-order/)
//!
//! ## Problem
//!
//! LeetCode has no Rust template for its concurrency problems, so this version defines its own API.
//!
//! One shared `Foo` value is used by three threads: thread A calls `first`, thread B calls
//! `second`, thread C calls `third`. The threads may be started in **any** order and scheduled in
//! any way. Make sure the three callbacks always run in the order `print_first`, `print_second`,
//! `print_third`:
//!
//! ```text
//! pub struct Foo { /* your fields */ }
//! impl Foo {
//!     pub fn new() -> Self;
//!     pub fn first(&self, print_first: impl FnOnce());
//!     pub fn second(&self, print_second: impl FnOnce());   // waits until first is done
//!     pub fn third(&self, print_third: impl FnOnce());     // waits until second is done
//! }
//! ```
//!
//! Each method is called exactly once per `Foo`. Do not busy-wait (no spinning loops): a waiting
//! thread must sleep until it is its turn.
//!
//! ## Examples
//!
//! ```text
//! Threads started in the order: third, first, second
//! Callbacks append "first", "second", "third" to a shared string
//! Output: "firstsecondthird"
//! ```
//!
//! ## Constraints
//!
//! - The three methods are called from three different threads, in any order.
//!
//! ## Hints
//!
//! 1. Keep a counter `turn` (`1`, `2`, `3`) in a `Mutex`. Method `k` waits until `turn == k`, runs
//!    its callback, sets `turn = k + 1` and wakes the others.
//! 2. `condvar.wait_while(guard, |turn| *turn != k)` sleeps until the condition is false.
//! 3. Use `notify_all`: the thread that should run next may not be the one `notify_one` would wake.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** one private helper `run_turn(k, callback)` does the work for all three methods: lock
//! the turn, wait while it is not `k`, run the callback, advance the turn, `notify_all`. Running the
//! callback while holding the lock keeps it simple; nothing else needs the lock at that time.
//!
//! **Complexity:** `O(1)` work per call; each waiting thread is woken at most twice (once per earlier turn).
//!
//! **Rust concepts:**
//! - `impl FnOnce()` in argument position is a generic parameter: each call can pass a different
//!   closure type, and the closure may move captured values because it runs only once.
//! - `Mutex` + `Condvar` is the general tool for "wait until some state holds"; problems 0895-0899
//!   use the same pattern with richer state.
// @solution-end

use std::sync::{Condvar, Mutex, PoisonError};

pub struct Foo {
    // @solution-start remove
    turn: Mutex<u8>,
    changed: Condvar,
    // @solution-end
}

impl Foo {
    pub fn new() -> Self {
        // @solution-start
        Self {
            turn: Mutex::new(1),
            changed: Condvar::new(),
        }
        // @solution-end
    }

    // @solution-start remove
    fn run_turn(&self, my_turn: u8, callback: impl FnOnce()) {
        let guard = self.turn.lock().unwrap_or_else(PoisonError::into_inner);
        let mut turn = self
            .changed
            .wait_while(guard, |turn| *turn != my_turn)
            .unwrap_or_else(PoisonError::into_inner);
        callback();
        *turn += 1;
        self.changed.notify_all();
    }
    // @solution-end

    pub fn first(&self, print_first: impl FnOnce()) {
        // @solution-start
        self.run_turn(1, print_first);
        // @solution-end
    }

    pub fn second(&self, print_second: impl FnOnce()) {
        // @solution-start
        self.run_turn(2, print_second);
        // @solution-end
    }

    pub fn third(&self, print_third: impl FnOnce()) {
        // @solution-start
        self.run_turn(3, print_third);
        // @solution-end
    }
}

impl Default for Foo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    /// Starts the three threads in the given order (1 = first, 2 = second, 3 = third).
    fn run(order: [u8; 3]) -> String {
        let foo = Foo::new();
        let out = Mutex::new(String::new());
        thread::scope(|s| {
            for k in order {
                let (foo, out) = (&foo, &out);
                s.spawn(move || {
                    let print = |text: &str| out.lock().unwrap().push_str(text);
                    match k {
                        1 => foo.first(|| print("first")),
                        2 => foo.second(|| print("second")),
                        _ => foo.third(|| print("third")),
                    }
                });
            }
        });
        out.into_inner().unwrap()
    }

    #[test]
    fn example_third_first_second() {
        assert_eq!(run([3, 1, 2]), "firstsecondthird");
    }

    #[test]
    fn natural_order() {
        assert_eq!(run([1, 2, 3]), "firstsecondthird");
    }

    #[test]
    fn reversed_order() {
        assert_eq!(run([3, 2, 1]), "firstsecondthird");
    }

    #[test]
    fn every_start_order_many_times() {
        let orders = [
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1],
        ];
        for _ in 0..50 {
            for order in orders {
                assert_eq!(run(order), "firstsecondthird", "start order {order:?}");
            }
        }
    }
}
