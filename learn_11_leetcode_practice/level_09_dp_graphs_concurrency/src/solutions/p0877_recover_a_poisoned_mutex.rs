//! # Problem 0877: Recover a Poisoned Mutex
//!
//! - **Difficulty:** Medium
//! - **Topics:** mutex poisoning, `LockResult`, `PoisonError::into_inner`, `Mutex::is_poisoned`,
//!   `Mutex::clear_poison`
//! - **Source:** Original · [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) ·
//!   [`PoisonError`](https://doc.rust-lang.org/std/sync/struct.PoisonError.html)
//!
//! ## Problem
//!
//! If a thread panics while it holds a `MutexGuard`, the mutex becomes **poisoned**: every later
//! `lock()` returns `Err(PoisonError)`, as a warning that the data might be half-updated. The
//! error still contains the guard, so a program that knows its data is fine can recover it.
//!
//! 1. `total_with_crashes(jobs: Vec<i64>) -> (i64, bool)`: one shared `Mutex<i64>` starts at `0`.
//!    Run every job on its own thread. A job locks the mutex and adds its value; if the value is
//!    **negative**, the job then panics while still holding the lock (after adding). Jobs must keep
//!    working even if the mutex was poisoned by an earlier job. After all threads have finished,
//!    return the final total and whether the mutex ended up poisoned.
//! 2. `reset_after_crash() -> (bool, bool)`: poison a fresh `Mutex` on purpose (a thread panics
//!    while holding it), then clear the poison flag. Return `(poisoned before clearing, poisoned
//!    after clearing)`.
//!
//! ## Examples
//!
//! ```text
//! total_with_crashes(vec![1, 2, 3])   -> (6, false)
//! total_with_crashes(vec![5, -1, 10]) -> (14, true)     // -1 is added before the panic
//! total_with_crashes(vec![])          -> (0, false)
//! reset_after_crash()                 -> (true, false)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= jobs.len() <= 100`; totals fit in an `i64`.
//! - (The panic messages appear on the test output; that is expected.)
//!
//! ## Hints
//!
//! 1. `mutex.lock().unwrap_or_else(PoisonError::into_inner)` returns the guard in both cases.
//! 2. Joining a panicked thread returns `Err`; that is expected here, so do not `unwrap` it.
//! 3. `Mutex::into_inner` also returns a `LockResult`; recover from it the same way.
//!    `clear_poison` (stable since Rust 1.77) removes the flag.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** share `Arc<Mutex<i64>>`; each job locks with recovery, adds, and panics while the guard
//! is alive if its value is negative. After joining every thread (ignoring the expected `Err`s),
//! read `is_poisoned()` and take the value out with `Arc::into_inner` and `into_inner`, recovering
//! once more from a possible poison error. For the second function, poison a mutex from a scoped
//! thread, then call `clear_poison`.
//!
//! **Rust concepts:**
//! - Poisoning is advisory: Rust does not know whether your invariant broke, so it hands the
//!   decision to you instead of silently ignoring the panic.
//! - `PoisonError<T>::into_inner(self) -> T` gives back what `lock()` would have returned.
//! - `thread::scope` re-raises a panic from one of its threads unless that thread was joined
//!   explicitly; joining the handle and dropping the `Err` keeps the panic contained.
// @solution-end

use std::sync::{Arc, Mutex, PoisonError};
use std::thread;

pub fn total_with_crashes(jobs: Vec<i64>) -> (i64, bool) {
    // @solution-start
    let total = Arc::new(Mutex::new(0_i64));
    let handles: Vec<_> = jobs
        .into_iter()
        .map(|value| {
            let total = Arc::clone(&total);
            thread::spawn(move || {
                let mut guard = total.lock().unwrap_or_else(PoisonError::into_inner);
                *guard += value;
                if value < 0 {
                    panic!("job {value} crashed while holding the lock");
                }
            })
        })
        .collect();
    for h in handles {
        let _ = h.join(); // `Err` is expected for the crashing jobs
    }
    let poisoned = total.is_poisoned();
    let value = Arc::into_inner(total)
        .expect("all jobs have finished, so this is the last Arc")
        .into_inner()
        .unwrap_or_else(PoisonError::into_inner);
    (value, poisoned)
    // @solution-end
}

pub fn reset_after_crash() -> (bool, bool) {
    // @solution-start
    let mutex = Mutex::new(0_u32);
    thread::scope(|s| {
        let crash = s.spawn(|| {
            let _guard = mutex.lock().unwrap_or_else(PoisonError::into_inner);
            panic!("crash while holding the lock");
        });
        let _ = crash.join(); // contain the panic
    });
    let before = mutex.is_poisoned();
    mutex.clear_poison();
    (before, mutex.is_poisoned())
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_no_crash() {
        assert_eq!(total_with_crashes(vec![1, 2, 3]), (6, false));
    }

    #[test]
    fn example_one_crash() {
        assert_eq!(total_with_crashes(vec![5, -1, 10]), (14, true));
    }

    #[test]
    fn example_no_jobs() {
        assert_eq!(total_with_crashes(vec![]), (0, false));
    }

    #[test]
    fn example_reset() {
        assert_eq!(reset_after_crash(), (true, false));
    }

    #[test]
    fn only_a_crashing_job() {
        assert_eq!(total_with_crashes(vec![-7]), (-7, true));
    }

    #[test]
    fn many_jobs_with_several_crashes() {
        // Every tenth job is negative; all values are still added.
        let jobs: Vec<i64> = (1..=100)
            .map(|i| if i % 10 == 0 { -i } else { i })
            .collect();
        let want: i64 = jobs.iter().sum();
        assert_eq!(total_with_crashes(jobs), (want, true));
    }
}
