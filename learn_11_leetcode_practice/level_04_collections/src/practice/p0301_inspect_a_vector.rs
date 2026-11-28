// ==================================================================
// PRACTICE 0301: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0301_inspect_a_vector.md
// Run the tests:          cargo test practice::p0301
// ==================================================================

//! # Problem 0301: Inspect a Vector
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Vec<T>`, `len`, `first`, `last`, `get` vs indexing, `contains`, `iter().position`
//! - **Source:** Original · [The Rust Book 8.1](https://doc.rust-lang.org/book/ch08-01-vectors.html) · [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
//!
//! ## Problem
//!
//! A weather station stores one temperature per hour in a vector. Write four small functions that
//! read the vector **without ever panicking**, even when it is empty or an index is too large.
//!
//! 1. `describe(values: &[i32]) -> (usize, Option<i32>, Option<i32>)` returns the number of
//!    readings, the first reading and the last reading. The two options are `None` when the
//!    slice is empty.
//! 2. `value_or(values: &[i32], index: usize, fallback: i32) -> i32` returns the reading at
//!    `index`, or `fallback` when `index` is out of bounds.
//! 3. `first_position(values: &[i32], target: i32) -> Option<usize>` returns the index of the
//!    first reading equal to `target`, or `None` if there is none.
//! 4. `contains_all(values: &[i32], wanted: &[i32]) -> bool` returns `true` when every number in
//!    `wanted` appears somewhere in `values` (an empty `wanted` is always satisfied).
//!
//! ## Examples
//!
//! ```text
//! describe(&[12, 15, 9])           => (3, Some(12), Some(9))
//! describe(&[])                    => (0, None, None)
//! value_or(&[12, 15, 9], 1, -99)   => 15
//! value_or(&[12, 15, 9], 7, -99)   => -99      (index 7 does not exist)
//! first_position(&[4, 8, 4], 4)    => Some(0)
//! first_position(&[4, 8, 4], 5)    => None
//! contains_all(&[4, 8, 4], &[8, 4]) => true
//! contains_all(&[4, 8, 4], &[8, 1]) => false
//! ```
//!
//! ## Constraints
//!
//! - `0 <= values.len(), wanted.len() <= 1000`
//! - `index` can be any `usize`, including values far beyond the length.
//!
//! ## Hints
//!
//! 1. `values[index]` panics when `index` is out of bounds; `values.get(index)` returns an
//!    `Option<&i32>` instead.
//! 2. `first()` and `last()` also return `Option<&T>`. Turn `Option<&i32>` into `Option<i32>` with
//!    `.copied()`.
//! 3. `iter().position(|&x| ...)` finds an index; `contains(&x)` answers yes/no.

/// Returns `(number of readings, first reading, last reading)`.
pub fn describe(values: &[i32]) -> (usize, Option<i32>, Option<i32>) {
    todo!("Write your solution here")
}

/// Returns the reading at `index`, or `fallback` when the index is out of bounds.
pub fn value_or(values: &[i32], index: usize, fallback: i32) -> i32 {
    todo!("Write your solution here")
}

/// Returns the index of the first reading equal to `target`.
pub fn first_position(values: &[i32], target: i32) -> Option<usize> {
    todo!("Write your solution here")
}

/// Returns `true` when every value of `wanted` appears in `values`.
pub fn contains_all(values: &[i32], wanted: &[i32]) -> bool {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describe_examples() {
        assert_eq!(describe(&[12, 15, 9]), (3, Some(12), Some(9)));
        assert_eq!(describe(&[]), (0, None, None));
        assert_eq!(describe(&[7]), (1, Some(7), Some(7)));
    }

    #[test]
    fn value_or_in_and_out_of_bounds() {
        let v = vec![12, 15, 9];
        assert_eq!(value_or(&v, 1, -99), 15);
        assert_eq!(value_or(&v, 3, -99), -99);
        assert_eq!(value_or(&v, usize::MAX, 0), 0);
        assert_eq!(value_or(&[], 0, 42), 42);
    }

    #[test]
    fn first_position_finds_the_first_match() {
        assert_eq!(first_position(&[4, 8, 4], 4), Some(0));
        assert_eq!(first_position(&[4, 8, 4], 8), Some(1));
        assert_eq!(first_position(&[4, 8, 4], 5), None);
        assert_eq!(first_position(&[], 5), None);
    }

    #[test]
    fn contains_all_examples() {
        assert!(contains_all(&[4, 8, 4], &[8, 4]));
        assert!(!contains_all(&[4, 8, 4], &[8, 1]));
        assert!(contains_all(&[], &[]));
        assert!(contains_all(&[1, 2], &[]));
        assert!(!contains_all(&[], &[1]));
    }

    #[test]
    fn large_vector() {
        let v: Vec<i32> = (0..1000).collect();
        assert_eq!(describe(&v), (1000, Some(0), Some(999)));
        assert_eq!(first_position(&v, 999), Some(999));
        assert!(contains_all(&v, &[0, 500, 999]));
    }
}
