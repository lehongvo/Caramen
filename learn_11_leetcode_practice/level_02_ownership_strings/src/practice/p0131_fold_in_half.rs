// ==================================================================
// PRACTICE 0131: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0131_fold_in_half.md
// Run the tests:          cargo test practice::p0131
// ==================================================================

//! # Problem 0131: Fold a Slice in Half
//!
//! - **Difficulty:** Easy
//! - **Topics:** `split_at_mut`, a shared and a mutable view of one slice, `zip`, `rev`
//! - **Source:** Original · [`slice::split_at_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut) · [The Rust Book 4.3](https://doc.rust-lang.org/book/ch04-03-slices.html)
//!
//! ## Problem
//!
//! Imagine writing a list of numbers on a strip of paper and folding it in the middle so that the
//! first element lies on top of the last one, the second on top of the second to last, and so on.
//!
//! Implement `fold_in_half(values: &mut [i32])`. Let `n = values.len()`. For every
//! `i < n / 2`, add `values[i]` to its mirror element `values[n - 1 - i]`. The first half itself does
//! not change, and for an odd `n` the middle element (which has no partner) does not change either.
//!
//! You can solve this with indices, but try the iterator version: you need to **read** the left half
//! while **writing** the right half of the same slice. `values.iter()` and `values.iter_mut()` cannot
//! coexist, but `split_at_mut` gives you two non-overlapping halves.
//!
//! ## Examples
//!
//! ```text
//! Input:  values = [1, 2, 3, 4]
//! Output: values = [1, 2, 5, 5]          (4 + 1, 3 + 2)
//!
//! Input:  values = [1, 2, 3, 4, 5]
//! Output: values = [1, 2, 3, 6, 6]       (5 + 1, 4 + 2, the middle 3 stays)
//!
//! Input:  values = [7]
//! Output: values = [7]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= values.len() <= 10^5`
//! - `-10^8 <= values[i] <= 10^8`
//!
//! ## Hints
//!
//! 1. `let (left, right) = values.split_at_mut(n / 2);` gives `left: &mut [i32]` (the first `n / 2`
//!    elements) and `right: &mut [i32]` (the rest, including the middle element when `n` is odd).
//! 2. Pair `left.iter()` with `right.iter_mut().rev()` using `zip`; `zip` stops at the shorter side,
//!    which leaves the middle element alone.

pub fn fold_in_half(values: &mut [i32]) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn folded(mut values: Vec<i32>) -> Vec<i32> {
        fold_in_half(&mut values);
        values
    }

    #[test]
    fn example_even_length() {
        assert_eq!(folded(vec![1, 2, 3, 4]), vec![1, 2, 5, 5]);
    }

    #[test]
    fn example_odd_length() {
        assert_eq!(folded(vec![1, 2, 3, 4, 5]), vec![1, 2, 3, 6, 6]);
    }

    #[test]
    fn example_single_element() {
        assert_eq!(folded(vec![7]), vec![7]);
    }

    #[test]
    fn empty_and_two_elements() {
        assert_eq!(folded(vec![]), Vec::<i32>::new());
        assert_eq!(folded(vec![-3, 10]), vec![-3, 7]);
    }

    #[test]
    fn large_input_matches_index_version() {
        let values: Vec<i32> = (0..100_001).map(|i| (i % 1000) - 500).collect();
        let mut expected = values.clone();
        let n = expected.len();
        for i in 0..n / 2 {
            expected[n - 1 - i] += expected[i];
        }
        assert_eq!(folded(values), expected);
    }
}
