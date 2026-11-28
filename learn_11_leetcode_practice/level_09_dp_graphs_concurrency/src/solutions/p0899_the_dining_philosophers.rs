//! # Problem 0899: The Dining Philosophers
//!
//! - **Difficulty:** Medium
//! - **Topics:** deadlock avoidance by resource ordering, an array of `Mutex<()>` used as plain
//!   locks, holding two guards, `std::array::from_fn`
//! - **Source:** Adapted from [LeetCode 1226. The Dining Philosophers](https://leetcode.com/problems/the-dining-philosophers/)
//!
//! ## Problem
//!
//! LeetCode has no Rust template for this problem, so the API is defined here.
//!
//! Five philosophers (`0` to `4`) sit around a round table with one fork between each pair of
//! neighbours. Philosopher `p` uses fork `p` as the **left** fork and fork `(p + 1) % 5` as the
//! **right** fork, and needs both to eat. Each philosopher runs on its own thread and calls
//! `wants_to_eat` many times:
//!
//! ```text
//! pub fn wants_to_eat(
//!     &self,
//!     philosopher: usize,
//!     pick_left_fork: impl FnOnce(),
//!     pick_right_fork: impl FnOnce(),
//!     eat: impl FnOnce(),
//!     put_left_fork: impl FnOnce(),
//!     put_right_fork: impl FnOnce(),
//! );
//! ```
//!
//! While the philosopher really holds both forks (no neighbour can hold them), call
//! `pick_left_fork`, `pick_right_fork` (in any order), then `eat`, then `put_left_fork`,
//! `put_right_fork` (in any order). The callbacks only *report* what happens; your code owns the
//! real synchronization. The solution must never deadlock, however the threads are scheduled.
//!
//! ## Examples
//!
//! ```text
//! 5 threads, each philosopher eats n = 1 time
//! -> 5 "eat" calls in total; no two neighbours eat at the same time.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 100` meals per philosopher.
//!
//! ## Hints
//!
//! 1. If every philosopher grabs the left fork first, all five can hold their left fork and wait
//!    forever for the right one: a circular wait, i.e. a deadlock.
//! 2. Break the circle by numbering the forks and always locking the **lower-numbered** fork first.
//!    Philosopher 4 then reaches for fork 0 before fork 4, so the five can never wait in a cycle.
//! 3. A `Mutex<()>` is a lock with no data; its guard is what "holding the fork" means.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `forks: [Mutex<()>; 5]`. In `wants_to_eat`, compute the two fork indices, lock the
//! smaller index first and the larger second, then run the five callbacks while both guards are
//! alive, and finally drop the guards. A global order on resources makes a cycle of waiting threads
//! impossible, which is one of the necessary conditions for deadlock.
//!
//! **Complexity:** `O(1)` work per call (two lock operations and five callbacks); how long a philosopher
//! waits depends only on its two neighbours.
//!
//! **Rust concepts:**
//! - `std::array::from_fn(|_| Mutex::new(()))` builds an array of non-`Copy` values.
//! - Guards are released in reverse declaration order when they go out of scope; the explicit
//!   block makes the critical section visible.
//! - Rust's type system prevents data races, but deadlock freedom is a property of the design.
// @solution-end

use std::sync::{Mutex, PoisonError};

pub struct DiningPhilosophers {
    // @solution-start remove
    forks: [Mutex<()>; 5],
    // @solution-end
}

impl DiningPhilosophers {
    pub fn new() -> Self {
        // @solution-start
        Self {
            forks: std::array::from_fn(|_| Mutex::new(())),
        }
        // @solution-end
    }

    pub fn wants_to_eat(
        &self,
        philosopher: usize,
        pick_left_fork: impl FnOnce(),
        pick_right_fork: impl FnOnce(),
        eat: impl FnOnce(),
        put_left_fork: impl FnOnce(),
        put_right_fork: impl FnOnce(),
    ) {
        // @solution-start
        let (left, right) = (philosopher, (philosopher + 1) % 5);
        let (first, second) = (left.min(right), left.max(right));
        let _first = self.forks[first]
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let _second = self.forks[second]
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        pick_left_fork();
        pick_right_fork();
        eat();
        put_left_fork();
        put_right_fork();
        // @solution-end
    }
}

impl Default for DiningPhilosophers {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    /// Records who holds each fork, according to the callbacks, and checks every rule.
    struct Table {
        holder: Mutex<[Option<usize>; 5]>,
        meals: AtomicUsize,
    }

    impl Table {
        fn pick(&self, p: usize, fork: usize) {
            let mut holder = self.holder.lock().unwrap();
            assert_eq!(holder[fork], None, "fork {fork} is already taken");
            holder[fork] = Some(p);
        }

        fn eat(&self, p: usize) {
            let holder = self.holder.lock().unwrap();
            assert_eq!(
                holder[p],
                Some(p),
                "philosopher {p} eats without the left fork"
            );
            assert_eq!(
                holder[(p + 1) % 5],
                Some(p),
                "philosopher {p} eats without the right fork"
            );
            self.meals.fetch_add(1, Ordering::SeqCst);
        }

        fn put(&self, p: usize, fork: usize) {
            let mut holder = self.holder.lock().unwrap();
            assert_eq!(
                holder[fork],
                Some(p),
                "philosopher {p} puts down a fork it does not hold"
            );
            holder[fork] = None;
        }
    }

    fn dinner(n: usize) -> usize {
        let philosophers = DiningPhilosophers::new();
        let table = Table {
            holder: Mutex::new([None; 5]),
            meals: AtomicUsize::new(0),
        };
        thread::scope(|s| {
            for p in 0..5 {
                let (philosophers, table) = (&philosophers, &table);
                s.spawn(move || {
                    let (left, right) = (p, (p + 1) % 5);
                    for _ in 0..n {
                        philosophers.wants_to_eat(
                            p,
                            || table.pick(p, left),
                            || table.pick(p, right),
                            || table.eat(p),
                            || table.put(p, left),
                            || table.put(p, right),
                        );
                    }
                });
            }
        });
        table.meals.load(Ordering::SeqCst)
    }

    #[test]
    fn example_one_meal_each() {
        assert_eq!(dinner(1), 5);
    }

    #[test]
    fn several_meals() {
        assert_eq!(dinner(10), 50);
    }

    #[test]
    fn long_dinner_without_deadlock() {
        assert_eq!(dinner(100), 500);
        for _ in 0..20 {
            assert_eq!(dinner(20), 100);
        }
    }
}
