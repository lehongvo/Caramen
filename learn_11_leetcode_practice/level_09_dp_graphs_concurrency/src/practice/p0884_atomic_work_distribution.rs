// ==================================================================
// PRACTICE 0884: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0884_atomic_work_distribution.md
// Run the tests:          cargo test practice::p0884
// ==================================================================

//! # Problem 0884: Atomic Work Distribution
//!
//! - **Difficulty:** Medium
//! - **Topics:** dynamic load balancing with a shared `AtomicU64` cursor, claiming blocks of work
//!   with `fetch_add`, scoped threads returning partial results
//! - **Source:** Original · [`std::sync::atomic`](https://doc.rust-lang.org/std/sync/atomic/index.html)
//!
//! ## Problem
//!
//! Splitting work into equal chunks up front (problems 0868, 0878) is unfair when some items cost
//! much more than others: testing whether a large number is prime takes longer than a small one.
//! A better strategy is to let workers **claim** the next piece of work when they are ready.
//!
//! Implement `count_primes_parallel(limit: u64, workers: usize) -> u64`, the number of primes `p`
//! with `p < limit`, computed by `workers` scoped threads that share one atomic cursor:
//!
//! - a worker claims the next block of `BLOCK = 1024` numbers with a single `fetch_add(BLOCK)` on
//!   the cursor, which returns the block's start;
//! - it tests the numbers `start..min(start + BLOCK, limit)` with trial division and counts the
//!   primes;
//! - it stops as soon as a claimed block starts at or after `limit`.
//!
//! Every number below `limit` is tested by exactly one worker; the result is the same for any
//! number of workers.
//!
//! ## Examples
//!
//! ```text
//! count_primes_parallel(10, 4)     -> 4      (2, 3, 5, 7)
//! count_primes_parallel(100, 3)    -> 25
//! count_primes_parallel(2, 8)      -> 0
//! ```
//!
//! ## Constraints
//!
//! - `0 <= limit <= 10^6`; `1 <= workers <= 32`
//!
//! ## Hints
//!
//! 1. `let next = AtomicU64::new(0);` lives on the stack; scoped threads can borrow it.
//! 2. `fetch_add` is a single indivisible read-modify-write: two workers can never receive the same
//!    start value.
//! 3. Trial division: `n` is prime if `n >= 2` and no `d` with `d * d <= n` divides it.

use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

/// How many numbers a worker claims at a time.
pub const BLOCK: u64 = 1024;

pub fn count_primes_parallel(limit: u64, workers: usize) -> u64 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_below_ten() {
        assert_eq!(count_primes_parallel(10, 4), 4);
    }

    #[test]
    fn example_below_hundred() {
        assert_eq!(count_primes_parallel(100, 3), 25);
    }

    #[test]
    fn example_tiny_limits() {
        assert_eq!(count_primes_parallel(2, 8), 0);
        assert_eq!(count_primes_parallel(0, 1), 0);
        assert_eq!(count_primes_parallel(3, 2), 1);
    }

    #[test]
    fn limit_on_a_block_boundary() {
        // 1024 is exactly one block; there are 172 primes below 1024.
        assert_eq!(count_primes_parallel(BLOCK, 5), 172);
        assert_eq!(count_primes_parallel(BLOCK + 1, 5), 172);
    }

    #[test]
    fn same_answer_for_any_worker_count() {
        for workers in [1, 2, 7, 32] {
            assert_eq!(
                count_primes_parallel(100_000, workers),
                9592,
                "{workers} workers"
            );
        }
        assert_eq!(count_primes_parallel(200_000, 8), 17_984);
    }
}
