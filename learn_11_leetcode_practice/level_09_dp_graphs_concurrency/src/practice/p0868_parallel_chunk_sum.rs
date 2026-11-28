// ==================================================================
// PRACTICE 0868: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0868_parallel_chunk_sum.md
// Run the tests:          cargo test practice::p0868
// ==================================================================

//! # Problem 0868: Parallel Chunk Sum
//!
//! - **Difficulty:** Easy
//! - **Topics:** splitting owned data into balanced chunks, moving each chunk into a thread,
//!   `Iterator::by_ref` + `take`
//! - **Source:** Original · [The Rust Book 16.1](https://doc.rust-lang.org/book/ch16-01-threads.html)
//!
//! ## Problem
//!
//! Implement `partial_sums(data: Vec<i64>, workers: usize) -> Vec<i64>`.
//!
//! Split `data` into `k = min(workers, data.len())` **contiguous** chunks whose sizes differ by at
//! most one, with the larger chunks first: if `data.len() = q * k + r`, the first `r` chunks have
//! `q + 1` elements and the others `q`. Sum each chunk **on its own thread** (one spawned thread per
//! chunk) and return the partial sums in chunk order. An empty `data` gives an empty result.
//!
//! The threads are created with `std::thread::spawn`, so they cannot borrow `data`: each chunk has
//! to be **moved** into its thread as an owned `Vec<i64>`.
//!
//! ## Examples
//!
//! ```text
//! partial_sums(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3) -> [10, 18, 27]
//! Explanation: chunk sizes 4, 3, 3: [1, 2, 3, 4], [5, 6, 7], [8, 9, 10].
//!
//! partial_sums(vec![5, 5], 4)  -> [5, 5]       (only two chunks are possible)
//! partial_sums(vec![], 3)      -> []
//! ```
//!
//! ## Constraints
//!
//! - `1 <= workers <= 64`; `0 <= data.len() <= 10^6`; sums fit in an `i64`.
//!
//! ## Hints
//!
//! 1. `q = len / k` and `r = len % k` give the chunk sizes: chunk `i` has `q + 1` elements when
//!    `i < r`.
//! 2. Turning `data` into an iterator with `into_iter()` lets you take owned chunks one after the
//!    other: `rest.by_ref().take(size).collect::<Vec<_>>()` takes the next `size` elements without
//!    consuming the whole iterator.
//! 3. Spawn every thread first, then join them in order.

use std::thread;

pub fn partial_sums(data: Vec<i64>, workers: usize) -> Vec<i64> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_chunks() {
        let data: Vec<i64> = (1..=10).collect();
        assert_eq!(partial_sums(data, 3), vec![10, 18, 27]);
    }

    #[test]
    fn example_more_workers_than_items() {
        assert_eq!(partial_sums(vec![5, 5], 4), vec![5, 5]);
    }

    #[test]
    fn example_empty_data() {
        assert_eq!(partial_sums(vec![], 3), Vec::<i64>::new());
    }

    #[test]
    fn single_worker_sums_everything() {
        assert_eq!(partial_sums(vec![-3, 7, 10], 1), vec![14]);
    }

    #[test]
    fn exact_division() {
        assert_eq!(partial_sums(vec![1; 12], 4), vec![3, 3, 3, 3]);
    }

    #[test]
    fn large_input_matches_sequential_sum() {
        let data: Vec<i64> = (0..1_000_000).collect();
        let parts = partial_sums(data, 7);
        assert_eq!(parts.len(), 7);
        assert_eq!(parts.iter().sum::<i64>(), 999_999 * 1_000_000 / 2);
        // 1_000_000 = 7 * 142_857 + 1: the first chunk has one extra element.
        assert_eq!(parts[0], (0..142_858).sum::<i64>());
    }
}
