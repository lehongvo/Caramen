// ==================================================================
// PRACTICE 0115: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0115_split_at_the_first_zero.md
// Run the tests:          cargo test practice::p0115
// ==================================================================

//! # Problem 0115: Split at the First Zero
//!
//! - **Difficulty:** Easy
//! - **Topics:** returning two borrowed slices, `position`, `split_at`, `Option` with `match`
//! - **Source:** Original · [The Rust Book 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html) · [`slice::split_at`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at)
//!
//! ## Problem
//!
//! A packet is a list of numbers where the first `0` acts as a separator between a header and a
//! body. Implement `split_at_first_zero(values: &[i32]) -> (&[i32], &[i32])`:
//!
//! - if `values` contains a `0`, return `(before, after)` where `before` holds the elements before
//!   the **first** zero and `after` holds the elements after it (the separator itself belongs to
//!   neither part; later zeros stay in `after`);
//! - if `values` contains no zero, return `(values, &[])`.
//!
//! Both returned slices borrow from `values`; do not allocate.
//!
//! ## Examples
//!
//! ```text
//! Input:  values = [3, 1, 0, 4, 0, 5]
//! Output: ([3, 1], [4, 0, 5])
//!
//! Input:  values = [0, 9]
//! Output: ([], [9])
//!
//! Input:  values = [1, 2, 3]
//! Output: ([1, 2, 3], [])
//! ```
//!
//! ## Constraints
//!
//! - `0 <= values.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `values.iter().position(|&v| v == 0)` gives the index of the first zero, if any.
//! 2. With index `i`, `&values[..i]` is the part before and `&values[i + 1..]` the part after.
//! 3. `values.split_at(i)` returns `(&values[..i], &values[i..])` in one call.

pub fn split_at_first_zero(values: &[i32]) -> (&[i32], &[i32]) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_zeros() {
        let (before, after) = split_at_first_zero(&[3, 1, 0, 4, 0, 5]);
        assert_eq!(before, &[3, 1]);
        assert_eq!(after, &[4, 0, 5]);
    }

    #[test]
    fn example_zero_first() {
        let (before, after) = split_at_first_zero(&[0, 9]);
        assert!(before.is_empty());
        assert_eq!(after, &[9]);
    }

    #[test]
    fn example_no_zero() {
        let (before, after) = split_at_first_zero(&[1, 2, 3]);
        assert_eq!(before, &[1, 2, 3]);
        assert!(after.is_empty());
    }

    #[test]
    fn zero_last_and_only_zero() {
        let (before, after) = split_at_first_zero(&[5, 0]);
        assert_eq!((before, after), (&[5][..], &[][..]));
        let (before, after) = split_at_first_zero(&[0]);
        assert!(before.is_empty() && after.is_empty());
    }

    #[test]
    fn empty_input() {
        let (before, after) = split_at_first_zero(&[]);
        assert!(before.is_empty() && after.is_empty());
    }

    #[test]
    fn large_input() {
        let mut values: Vec<i32> = (1..=100_000).collect();
        values[60_000] = 0;
        let (before, after) = split_at_first_zero(&values);
        assert_eq!(before.len(), 60_000);
        assert_eq!(after.len(), 39_999);
        assert_eq!(after[0], 60_002);
    }
}
