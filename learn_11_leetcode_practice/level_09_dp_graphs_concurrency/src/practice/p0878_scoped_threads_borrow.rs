// ==================================================================
// PRACTICE 0878: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0878_scoped_threads_borrow.md
// Run the tests:          cargo test practice::p0878
// ==================================================================

//! # Problem 0878: Borrow Data with Scoped Threads
//!
//! - **Difficulty:** Easy
//! - **Topics:** `std::thread::scope`, threads that borrow local data (no `Arc`, no cloning),
//!   `ScopedJoinHandle`
//! - **Source:** Original · [`thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html)
//!
//! ## Problem
//!
//! Threads started with `thread::spawn` may outlive the function that started them, so they cannot
//! borrow its local variables. `thread::scope(|s| { ... })` creates a scope that **joins every
//! thread spawned inside it before returning**, so those threads may borrow anything that outlives
//! the scope.
//!
//! Implement, using scoped threads and without copying the input:
//!
//! 1. `scoped_min_max_sum(data: &[i32], parts: usize) -> Option<(i32, i32, i64)>`: split `data`
//!    into at most `parts` contiguous chunks of `ceil(len / parts)` elements, compute the minimum,
//!    the maximum and the sum of each chunk on its own scoped thread, and combine them. Return
//!    `None` for an empty slice.
//! 2. `count_each(text: &str, needles: &[char]) -> Vec<usize>`: start one scoped thread per needle;
//!    thread `i` counts how many times `needles[i]` occurs in `text`. All threads read the same
//!    borrowed `text`. Return the counts in needle order.
//!
//! ## Examples
//!
//! ```text
//! scoped_min_max_sum(&[3, -1, 4, 1, 5], 2) -> Some((-1, 5, 12))
//! scoped_min_max_sum(&[], 4)               -> None
//! count_each("banana", &['a', 'n', 'z'])   -> [3, 2, 0]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= parts <= 64`; `0 <= data.len() <= 10^6`
//! - `0 <= needles.len() <= 32`; `text` may contain any Unicode characters.
//!
//! ## Hints
//!
//! 1. `thread::scope(|s| { let h = s.spawn(|| work(&data)); h.join() })`: the closure passed to
//!    `s.spawn` can capture references because the scope outlives it.
//! 2. `data.chunks(size)` yields borrowed sub-slices; spawn one thread per chunk and collect the
//!    handles before joining.
//! 3. Round the chunk size up with `len.div_ceil(parts)` (stable for unsigned integers since
//!    Rust 1.73).

use std::thread;

pub fn scoped_min_max_sum(data: &[i32], parts: usize) -> Option<(i32, i32, i64)> {
    todo!("Write your solution here")
}

pub fn count_each(text: &str, needles: &[char]) -> Vec<usize> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_parts() {
        assert_eq!(scoped_min_max_sum(&[3, -1, 4, 1, 5], 2), Some((-1, 5, 12)));
    }

    #[test]
    fn example_empty_slice() {
        assert_eq!(scoped_min_max_sum(&[], 4), None);
    }

    #[test]
    fn example_count_each() {
        assert_eq!(count_each("banana", &['a', 'n', 'z']), vec![3, 2, 0]);
    }

    #[test]
    fn more_parts_than_elements() {
        assert_eq!(scoped_min_max_sum(&[7], 64), Some((7, 7, 7)));
        assert_eq!(scoped_min_max_sum(&[2, 9], 64), Some((2, 9, 11)));
    }

    #[test]
    fn unicode_and_no_needles() {
        assert_eq!(count_each("ñoño über ñ", &['ñ', 'ü', ' ']), vec![3, 1, 2]);
        assert!(count_each("abc", &[]).is_empty());
    }

    #[test]
    fn large_input_does_not_overflow() {
        let data = vec![i32::MAX; 1_000_000];
        let got = scoped_min_max_sum(&data, 8);
        assert_eq!(
            got,
            Some((i32::MAX, i32::MAX, i64::from(i32::MAX) * 1_000_000))
        );
        let mixed: Vec<i32> = (-500_000..500_000).collect();
        assert_eq!(
            scoped_min_max_sum(&mixed, 7),
            Some((-500_000, 499_999, -500_000))
        );
    }
}
