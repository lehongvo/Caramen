//! # Problem 0890: Phased Work with a Barrier
//!
//! - **Difficulty:** Medium
//! - **Topics:** `std::sync::Barrier` to separate computation rounds, reusing a barrier, double
//!   buffering with `Vec<AtomicI64>`, `BarrierWaitResult::is_leader`
//! - **Source:** Original · [`Barrier`](https://doc.rust-lang.org/std/sync/struct.Barrier.html)
//!
//! ## Problem
//!
//! A `Barrier::new(n)` makes threads wait at `wait()` until all `n` of them have arrived, then
//! releases them together. It can be reused for the next round. Exactly one thread per round gets a
//! result whose `is_leader()` is `true`.
//!
//! 1. `smooth(values: &[i64], rounds: usize) -> Vec<i64>`: start **one scoped thread per element**.
//!    In every round, thread `i` computes `new[i] = old[i - 1] + old[i] + old[i + 1]` (a missing
//!    neighbour counts as `0`), where `old` are the values of the previous round. All threads must
//!    finish reading round `r` before anybody's round-`r + 1` result can overwrite it, so use a
//!    `Barrier` between rounds. Return the values after `rounds` rounds (`rounds = 0` returns the
//!    input unchanged).
//! 2. `count_leaders(threads: usize, rounds: usize) -> usize`: `threads` scoped threads wait on one
//!    shared barrier `rounds` times each; return how many `wait()` calls in total reported
//!    `is_leader()`.
//!
//! ## Examples
//!
//! ```text
//! smooth(&[1, 2, 3], 1)   -> [3, 6, 5]
//! smooth(&[1, 2, 3], 2)   -> [9, 14, 11]
//! smooth(&[], 5)          -> []
//! count_leaders(4, 10)    -> 10      (one leader per round)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= values.len() <= 64`, `|values[i]| <= 1000`, `0 <= rounds <= 20` (results fit in `i64`)
//! - `1 <= threads <= 32`, `0 <= rounds <= 1000` in `count_leaders`
//!
//! ## Hints
//!
//! 1. Keep two buffers of atomics: round `r` reads buffer `r % 2` and writes buffer `(r + 1) % 2`.
//! 2. After writing, wait on the barrier. When a thread passes the barrier, every thread has
//!    finished the reads *and* writes of that round, so the next round can start safely.
//! 3. `Barrier::wait` synchronizes memory: writes made before `wait` are visible to the other
//!    threads after their `wait`, so `Ordering::Relaxed` atomics are enough here.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** two `Vec<AtomicI64>` buffers, the first initialised with the input. Each scoped thread
//! runs the rounds: read its neighbours from the current buffer, store the sum in the other buffer,
//! wait on the barrier. Because a thread only writes the buffer nobody reads in this round, and
//! the barrier ends the round for everybody, one barrier per round is enough. The final values are
//! in buffer `rounds % 2`.
//!
//! **Rust concepts:**
//! - A `Barrier` must be created for the exact number of participating threads; one thread too few
//!   and everyone waits forever.
//! - `AtomicI64` gives shared mutable cells without locks; the barrier provides the ordering.
//! - Returning `is_leader()` from every `wait()` lets the caller count leaders after joining.
// @solution-end

use std::sync::Barrier;
use std::sync::atomic::{AtomicI64, Ordering};
use std::thread;

pub fn smooth(values: &[i64], rounds: usize) -> Vec<i64> {
    // @solution-start
    let n = values.len();
    if n == 0 || rounds == 0 {
        return values.to_vec();
    }
    let buffers: [Vec<AtomicI64>; 2] = [
        values.iter().map(|&v| AtomicI64::new(v)).collect(),
        (0..n).map(|_| AtomicI64::new(0)).collect(),
    ];
    let barrier = Barrier::new(n);
    thread::scope(|s| {
        for i in 0..n {
            let (buffers, barrier) = (&buffers, &barrier);
            s.spawn(move || {
                for r in 0..rounds {
                    let (old, new) = (&buffers[r % 2], &buffers[(r + 1) % 2]);
                    let read = |j: Option<usize>| {
                        j.and_then(|j| old.get(j))
                            .map_or(0, |a| a.load(Ordering::Relaxed))
                    };
                    let sum = read(i.checked_sub(1)) + read(Some(i)) + read(Some(i + 1));
                    new[i].store(sum, Ordering::Relaxed);
                    barrier.wait();
                }
            });
        }
    });
    buffers[rounds % 2]
        .iter()
        .map(|a| a.load(Ordering::Relaxed))
        .collect()
    // @solution-end
}

pub fn count_leaders(threads: usize, rounds: usize) -> usize {
    // @solution-start
    let barrier = Barrier::new(threads);
    thread::scope(|s| {
        let handles: Vec<_> = (0..threads)
            .map(|_| s.spawn(|| (0..rounds).filter(|_| barrier.wait().is_leader()).count()))
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("waiting on a barrier cannot panic"))
            .sum()
    })
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn smooth_sequential(values: &[i64], rounds: usize) -> Vec<i64> {
        let mut cur = values.to_vec();
        for _ in 0..rounds {
            cur = (0..cur.len())
                .map(|i| {
                    let left = if i > 0 { cur[i - 1] } else { 0 };
                    let right = cur.get(i + 1).copied().unwrap_or(0);
                    left + cur[i] + right
                })
                .collect();
        }
        cur
    }

    #[test]
    fn example_one_round() {
        assert_eq!(smooth(&[1, 2, 3], 1), vec![3, 6, 5]);
    }

    #[test]
    fn example_two_rounds() {
        assert_eq!(smooth(&[1, 2, 3], 2), vec![9, 14, 11]);
    }

    #[test]
    fn example_empty_and_zero_rounds() {
        assert!(smooth(&[], 5).is_empty());
        assert_eq!(smooth(&[4, -4], 0), vec![4, -4]);
    }

    #[test]
    fn example_leaders() {
        assert_eq!(count_leaders(4, 10), 10);
    }

    #[test]
    fn single_thread_barrier() {
        assert_eq!(smooth(&[7], 3), vec![7]);
        assert_eq!(count_leaders(1, 5), 5);
        assert_eq!(count_leaders(8, 0), 0);
    }

    #[test]
    fn many_threads_many_rounds() {
        let values: Vec<i64> = (0..64).map(|i| (i * 37 % 2001) - 1000).collect();
        assert_eq!(smooth(&values, 20), smooth_sequential(&values, 20));
        assert_eq!(count_leaders(32, 1000), 1000);
    }
}
