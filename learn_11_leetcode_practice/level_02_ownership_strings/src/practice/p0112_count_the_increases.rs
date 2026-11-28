// ==================================================================
// PRACTICE 0112: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0112_count_the_increases.md
// Run the tests:          cargo test practice::p0112
// ==================================================================

//! # Problem 0112: Count the Increases
//!
//! - **Difficulty:** Easy
//! - **Topics:** slices `&[T]`, `windows(2)`, slice patterns, `filter().count()`
//! - **Source:** Original · [The Rust Book 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html) · [`slice::windows`](https://doc.rust-lang.org/std/primitive.slice.html#method.windows)
//!
//! ## Problem
//!
//! A sensor records one depth measurement per second. Implement
//! `count_increases(depths: &[i32]) -> usize`, which returns how many measurements are **strictly
//! greater** than the measurement right before them.
//!
//! The first measurement has no predecessor, so it is never counted. A slice with fewer than two
//! elements therefore has `0` increases.
//!
//! The function borrows the data as a slice `&[i32]`, so it works with a `Vec<i32>`, an array or a
//! part of either (`&v[10..20]`).
//!
//! ## Examples
//!
//! ```text
//! Input:  depths = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
//! Output: 7
//! Explanation: 200, 208, 210, 207, 240, 269 and 263 are greater than the value before them.
//!
//! Input:  depths = [5, 5, 4]
//! Output: 0           (equal is not an increase)
//!
//! Input:  depths = [42]
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `0 <= depths.len() <= 10^5`
//! - `-10^9 <= depths[i] <= 10^9`
//!
//! ## Hints
//!
//! 1. `depths.windows(2)` yields every overlapping sub-slice of length 2: `[a, b]`, `[b, c]`, ...
//! 2. Each window is a `&[i32]`; compare `w[0] < w[1]`, then count with `filter(...).count()`.

pub fn count_increases(depths: &[i32]) -> usize {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sonar_sweep() {
        let depths = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases(&depths), 7);
    }

    #[test]
    fn example_equal_is_not_an_increase() {
        assert_eq!(count_increases(&[5, 5, 4]), 0);
    }

    #[test]
    fn example_single_value() {
        assert_eq!(count_increases(&[42]), 0);
    }

    #[test]
    fn empty_slice() {
        assert_eq!(count_increases(&[]), 0);
    }

    #[test]
    fn works_on_a_sub_slice_of_a_vec() {
        let depths = vec![1, 2, 3, 2, 1, 2];
        assert_eq!(count_increases(&depths), 3);
        assert_eq!(count_increases(&depths[2..5]), 0);
    }

    #[test]
    fn large_strictly_increasing() {
        let depths: Vec<i32> = (0..100_000).collect();
        assert_eq!(count_increases(&depths), 99_999);
    }
}
