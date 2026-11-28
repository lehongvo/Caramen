// ==================================================================
// PRACTICE 0364: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0364_process_jobs_by_priority.md
// Run the tests:          cargo test practice::p0364
// ==================================================================

//! # Problem 0364: Process Jobs by Priority
//!
//! - **Difficulty:** Easy
//! - **Topics:** `BinaryHeap` (a max-heap): `push`, `pop`, `peek`, `into_sorted_vec`, tuples as priorities, `Reverse`
//! - **Source:** Original · [`BinaryHeap`](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html) · [`std::cmp::Reverse`](https://doc.rust-lang.org/std/cmp/struct.Reverse.html)
//!
//! ## Problem
//!
//! `std::collections::BinaryHeap` is a priority queue: `pop` always returns the **largest** element
//! (according to `Ord`), in `O(log n)`, and `peek` looks at it in `O(1)`. Use it to implement:
//!
//! 1. `highest_priority(jobs: &[(u32, &str)]) -> Option<u32>`: the largest priority among the jobs
//!    `(priority, name)`, or `None` for no jobs.
//! 2. `processing_order(jobs: &[(u32, &str)]) -> Vec<String>`: the job names in the order a
//!    scheduler runs them: higher priority first, and among equal priorities, the job that appears
//!    **earlier** in `jobs` first.
//! 3. `largest_k(values: &[i32], k: usize) -> Vec<i32>`: the `k` largest values in decreasing order
//!    (all values if there are fewer than `k`), keeping duplicates.
//! 4. `ascending(values: &[i32]) -> Vec<i32>`: all values in increasing order, produced by the heap.
//!
//! ## Examples
//!
//! ```text
//! jobs = [(2, "email"), (5, "deploy"), (2, "backup"), (9, "alert")]
//! highest_priority(jobs) => Some(9)
//! processing_order(jobs) => ["alert", "deploy", "email", "backup"]   (email came before backup)
//!
//! largest_k([4, 1, 7, 7, 3], 3) => [7, 7, 4]
//! ascending([4, 1, 7, 7, 3])    => [1, 3, 4, 7, 7]
//! ```
//!
//! ## Constraints
//!
//! - At most `10^5` jobs or values.
//!
//! ## Hints
//!
//! 1. Tuples compare element by element, so a heap of `(priority, ...)` pops the highest priority
//!    first.
//! 2. To break ties by "earlier index first" in a **max**-heap, the smaller index must compare as
//!    **larger**: wrap it in `std::cmp::Reverse(index)`.
//! 3. `BinaryHeap::from(vec)` builds a heap in `O(n)`; `into_sorted_vec()` returns its elements in
//!    ascending order.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// The largest priority among the jobs.
pub fn highest_priority(jobs: &[(u32, &str)]) -> Option<u32> {
    todo!("Write your solution here")
}

/// Job names by priority (highest first), ties by input order.
pub fn processing_order(jobs: &[(u32, &str)]) -> Vec<String> {
    todo!("Write your solution here")
}

/// The `k` largest values in decreasing order.
pub fn largest_k(values: &[i32], k: usize) -> Vec<i32> {
    todo!("Write your solution here")
}

/// All values in increasing order.
pub fn ascending(values: &[i32]) -> Vec<i32> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    const JOBS: [(u32, &str); 4] = [(2, "email"), (5, "deploy"), (2, "backup"), (9, "alert")];

    #[test]
    fn highest_priority_example() {
        assert_eq!(highest_priority(&JOBS), Some(9));
        assert_eq!(highest_priority(&[]), None);
    }

    #[test]
    fn processing_order_example() {
        assert_eq!(
            processing_order(&JOBS),
            vec!["alert", "deploy", "email", "backup"]
        );
        assert_eq!(processing_order(&[]), Vec::<String>::new());
    }

    #[test]
    fn largest_k_examples() {
        assert_eq!(largest_k(&[4, 1, 7, 7, 3], 3), vec![7, 7, 4]);
        assert_eq!(largest_k(&[4, 1], 5), vec![4, 1]);
        assert_eq!(largest_k(&[4, 1], 0), Vec::<i32>::new());
    }

    #[test]
    fn ascending_example() {
        assert_eq!(ascending(&[4, 1, 7, 7, 3]), vec![1, 3, 4, 7, 7]);
        assert_eq!(ascending(&[]), Vec::<i32>::new());
    }

    #[test]
    fn many_jobs_with_equal_priorities() {
        let names: Vec<String> = (0..10_000).map(|i| format!("job{i:05}")).collect();
        let jobs: Vec<(u32, &str)> = names
            .iter()
            .enumerate()
            .map(|(i, n)| ((i % 3) as u32, n.as_str()))
            .collect();
        let order = processing_order(&jobs);
        // Priority 2 first (indices 2, 5, 8, ...), each group in input order.
        assert_eq!(order[0], "job00002");
        assert_eq!(order[1], "job00005");
        assert_eq!(order[9_999], "job09999");
        let values: Vec<i32> = (0..100_000).map(|i| (i * 7_919) % 100_000).collect();
        assert_eq!(largest_k(&values, 3), vec![99_999, 99_998, 99_997]);
    }
}
