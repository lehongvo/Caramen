// ==================================================================
// PRACTICE 0882: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0882_parallel_merge_sort.md
// Run the tests:          cargo test practice::p0882
// ==================================================================

//! # Problem 0882: Parallel Merge Sort
//!
//! - **Difficulty:** Medium
//! - **Topics:** divide and conquer across threads, `split_at_mut`, recursive `thread::scope`,
//!   limiting the number of threads with a depth budget, generic `T: Ord + Clone + Send`
//! - **Source:** Original · [`thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html) ·
//!   [slices](https://doc.rust-lang.org/std/primitive.slice.html)
//!
//! ## Problem
//!
//! Implement `parallel_merge_sort<T: Ord + Clone + Send>(data: &mut [T], depth: usize)`, which sorts
//! `data` in ascending order with merge sort:
//!
//! - if `data.len() <= 1`, there is nothing to do;
//! - if `depth == 0`, sort the slice on the current thread (you may call `data.sort()`);
//! - otherwise split `data` into two halves, sort the left half on a **new scoped thread** and the
//!   right half on the current thread, both recursively with `depth - 1`, then **merge** the two
//!   sorted halves back into `data`.
//!
//! With depth `d` at most `2^d - 1` extra threads are created, which keeps the thread count under
//! control. The result must be identical for every depth.
//!
//! ## Examples
//!
//! ```text
//! data = [5, 2, 9, 1, 5, 6]; parallel_merge_sort(&mut data, 2) -> data = [1, 2, 5, 5, 6, 9]
//! data = ["pear", "apple", "fig"]; parallel_merge_sort(&mut data, 1) -> ["apple", "fig", "pear"]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= data.len() <= 10^6`; `0 <= depth <= 6`
//!
//! ## Hints
//!
//! 1. `let (left, right) = data.split_at_mut(mid);` gives two disjoint `&mut` halves, so one can go
//!    to another thread while the current thread works on the other.
//! 2. `thread::scope(|s| { s.spawn(|| sort(left)); sort(right); });` waits for the spawned thread
//!    before returning.
//! 3. Merging in place is hard; merge into a temporary `Vec<T>` (cloning the elements) and copy it
//!    back with `clone_from_slice`.

use std::thread;

pub fn parallel_merge_sort<T: Ord + Clone + Send>(data: &mut [T], depth: usize) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic pseudo-random numbers (a linear congruential generator).
    fn pseudo_random(n: usize, seed: u64) -> Vec<u64> {
        let mut state = seed;
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                state >> 33
            })
            .collect()
    }

    #[test]
    fn example_numbers() {
        let mut data = vec![5, 2, 9, 1, 5, 6];
        parallel_merge_sort(&mut data, 2);
        assert_eq!(data, vec![1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn example_strings() {
        let mut data = vec!["pear".to_string(), "apple".to_string(), "fig".to_string()];
        parallel_merge_sort(&mut data, 1);
        assert_eq!(data, vec!["apple", "fig", "pear"]);
    }

    #[test]
    fn empty_and_single() {
        let mut empty: Vec<i32> = Vec::new();
        parallel_merge_sort(&mut empty, 3);
        assert!(empty.is_empty());
        let mut one = vec![42];
        parallel_merge_sort(&mut one, 3);
        assert_eq!(one, vec![42]);
    }

    #[test]
    fn every_depth_gives_the_same_result() {
        let original = pseudo_random(10_000, 7);
        let mut want = original.clone();
        want.sort();
        for depth in 0..=6 {
            let mut data = original.clone();
            parallel_merge_sort(&mut data, depth);
            assert_eq!(data, want, "depth {depth}");
        }
    }

    #[test]
    fn large_input_with_duplicates() {
        let mut data: Vec<u64> = pseudo_random(1_000_000, 42)
            .into_iter()
            .map(|x| x % 1000)
            .collect();
        parallel_merge_sort(&mut data, 4);
        assert!(data.windows(2).all(|w| w[0] <= w[1]));
        assert_eq!(data.len(), 1_000_000);
    }
}
