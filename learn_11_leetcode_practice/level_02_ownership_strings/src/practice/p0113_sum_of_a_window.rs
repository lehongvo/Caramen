// ==================================================================
// PRACTICE 0113: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0113_sum_of_a_window.md
// Run the tests:          cargo test practice::p0113
// ==================================================================

//! # Problem 0113: Sum of a Window
//!
//! - **Difficulty:** Easy
//! - **Topics:** range slicing `&v[a..b]`, `slice::get` with a range, `Option`, avoiding panics
//! - **Source:** Original · [The Rust Book 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html) · [`slice::get`](https://doc.rust-lang.org/std/primitive.slice.html#method.get)
//!
//! ## Problem
//!
//! Implement `window_sum(values: &[i32], start: usize, len: usize) -> Option<i32>`. It returns the
//! sum of the `len` consecutive elements that begin at index `start`, that is, of the sub-slice
//! `values[start..start + len]`.
//!
//! If that window does not fit inside `values` (it would start or end past the last element),
//! return `None` instead of panicking. A window of length `0` that fits (`start <= values.len()`)
//! has sum `0`.
//!
//! Writing `&values[start..start + len]` directly would **panic** for an out-of-range window. Use a
//! checked alternative.
//!
//! ## Examples
//!
//! ```text
//! Input:  values = [4, 1, 7, 3, 9], start = 1, len = 3
//! Output: Some(11)        (1 + 7 + 3)
//!
//! Input:  values = [4, 1, 7, 3, 9], start = 3, len = 3
//! Output: None            (the window would need index 5)
//!
//! Input:  values = [4, 1], start = 2, len = 0
//! Output: Some(0)         (an empty window right at the end still fits)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= values.len() <= 10^5`
//! - `0 <= start, len <= 10^5`
//! - The sum of any window fits in an `i32`.
//!
//! ## Hints
//!
//! 1. `values.get(a..b)` returns `Option<&[i32]>`: `Some(slice)` if the range is valid, else `None`.
//! 2. `Option::map` lets you turn the `Some(window)` into `Some(sum)`.

pub fn window_sum(values: &[i32], start: usize, len: usize) -> Option<i32> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_window_inside() {
        assert_eq!(window_sum(&[4, 1, 7, 3, 9], 1, 3), Some(11));
    }

    #[test]
    fn example_window_too_long() {
        assert_eq!(window_sum(&[4, 1, 7, 3, 9], 3, 3), None);
    }

    #[test]
    fn example_empty_window_at_the_end() {
        assert_eq!(window_sum(&[4, 1], 2, 0), Some(0));
    }

    #[test]
    fn start_past_the_end() {
        assert_eq!(window_sum(&[4, 1], 3, 0), None);
        assert_eq!(window_sum(&[], 1, 1), None);
    }

    #[test]
    fn whole_slice_and_negative_values() {
        assert_eq!(window_sum(&[-2, 5, -3], 0, 3), Some(0));
        assert_eq!(window_sum(&[-2, 5, -3], 2, 1), Some(-3));
    }

    #[test]
    fn large_input() {
        let values: Vec<i32> = (1..=100_000).map(|i| i % 3).collect();
        // 33_333 full cycles of (1, 2, 0) sum to 99_999, and the last value is 100_000 % 3 = 1.
        assert_eq!(window_sum(&values, 0, 100_000), Some(100_000));
        assert_eq!(window_sum(&values, 1, 100_000), None);
    }
}
