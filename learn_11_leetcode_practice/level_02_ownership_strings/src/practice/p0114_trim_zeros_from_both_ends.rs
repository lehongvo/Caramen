// ==================================================================
// PRACTICE 0114: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0114_trim_zeros_from_both_ends.md
// Run the tests:          cargo test practice::p0114
// ==================================================================

//! # Problem 0114: Trim Zeros From Both Ends
//!
//! - **Difficulty:** Easy
//! - **Topics:** returning a sub-slice `&[T]` that borrows from the input, `position`, `rposition`
//! - **Source:** Original · [The Rust Book 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html)
//!
//! ## Problem
//!
//! `str::trim` removes whitespace from both ends of a string and returns a slice of the original
//! text. Write the same idea for numbers.
//!
//! Implement `trim_zeros(values: &[i32]) -> &[i32]`. It returns the part of `values` that remains
//! after removing all leading zeros and all trailing zeros. Zeros **between** non-zero values stay.
//! If `values` contains only zeros (or is empty), return an empty slice.
//!
//! The result must be a sub-slice **of the input** (no new `Vec`): the returned reference borrows
//! from `values`. Because there is only one reference parameter, Rust's lifetime elision rules
//! connect the output to it automatically; no annotation is needed.
//!
//! ## Examples
//!
//! ```text
//! Input:  values = [0, 0, 3, 0, 5, 0]
//! Output: [3, 0, 5]
//!
//! Input:  values = [7, 8]
//! Output: [7, 8]
//!
//! Input:  values = [0, 0, 0]
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= values.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `iter().position(|&v| v != 0)` finds the first non-zero index (an `Option<usize>`).
//! 2. `iter().rposition(...)` searches from the back and returns the index of the last match.
//! 3. If there is a first non-zero value, there is also a last one; return `&values[first..=last]`.

pub fn trim_zeros(values: &[i32]) -> &[i32] {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_zeros_on_both_sides() {
        assert_eq!(trim_zeros(&[0, 0, 3, 0, 5, 0]), &[3, 0, 5]);
    }

    #[test]
    fn example_nothing_to_trim() {
        assert_eq!(trim_zeros(&[7, 8]), &[7, 8]);
    }

    #[test]
    fn example_only_zeros() {
        assert!(trim_zeros(&[0, 0, 0]).is_empty());
    }

    #[test]
    fn empty_input() {
        assert!(trim_zeros(&[]).is_empty());
    }

    #[test]
    fn single_non_zero_and_negative_values() {
        assert_eq!(trim_zeros(&[0, -4, 0]), &[-4]);
        assert_eq!(trim_zeros(&[-1, 0, 0]), &[-1]);
    }

    #[test]
    fn result_borrows_from_the_input() {
        let values = vec![0, 1, 2, 0];
        let inner = trim_zeros(&values);
        // The returned slice points into `values`: same address as values[1].
        assert_eq!(inner.as_ptr(), values[1..].as_ptr());
        assert_eq!(inner.len(), 2);
    }

    #[test]
    fn large_input() {
        let mut values = vec![0; 100_000];
        values[10] = 1;
        values[99_000] = 2;
        let inner = trim_zeros(&values);
        assert_eq!(inner.len(), 98_991);
        assert_eq!(inner[0], 1);
        assert_eq!(inner[inner.len() - 1], 2);
    }
}
