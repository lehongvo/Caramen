// ==================================================================
// PRACTICE 0653: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0653_reorder_list.md
// Run the tests:          cargo test practice::p0653
// ==================================================================

//! # Problem 0653: Reorder List
//!
//! - **Difficulty:** Medium
//! - **Topics:** `&mut Option<Box<ListNode>>` parameters, splitting, reversing and weaving owned lists
//! - **Source:** [LeetCode 143. Reorder List](https://leetcode.com/problems/reorder-list/)
//!
//! ## Problem
//!
//! A list `L0 -> L1 -> ... -> Ln-1 -> Ln` must be rearranged **in place** into
//! `L0 -> Ln -> L1 -> Ln-1 -> L2 -> Ln-2 -> ...`: first node, last node, second node, second to
//! last node, and so on. Move the nodes; do not change their values.
//!
//! Implement `Solution::reorder_list(head: &mut Option<Box<ListNode>>)`. The function returns
//! nothing: it rearranges the list that `head` points to.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 2, 3, 4]
//! After:  head = [1, 4, 2, 3]
//!
//! Input:  head = [1, 2, 3, 4, 5]
//! After:  head = [1, 5, 2, 4, 3]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 5 * 10^4`, `1 <= val <= 1000`
//!
//! ## Hints
//!
//! 1. Split the list in two halves: the first `n - n / 2` nodes stay in the first half.
//! 2. Reverse the second half (problem 0630). Now the "last" nodes come first.
//! 3. Weave: take one node from the first half, one from the reversed second half, and so on.
//!    Rebuild the result directly into `*head` with a tail cursor; `head.take()` gives you the
//!    original list to work with.

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    fn reordered(values: &[i32]) -> Vec<i32> {
        let mut head = list(values);
        Solution::reorder_list(&mut head);
        list_to_vec(&head)
    }

    #[test]
    fn even_length() {
        assert_eq!(reordered(&[1, 2, 3, 4]), vec![1, 4, 2, 3]);
    }

    #[test]
    fn odd_length() {
        assert_eq!(reordered(&[1, 2, 3, 4, 5]), vec![1, 5, 2, 4, 3]);
    }

    #[test]
    fn short_lists_are_unchanged() {
        assert_eq!(reordered(&[1]), vec![1]);
        assert_eq!(reordered(&[1, 2]), vec![1, 2]);
        assert_eq!(reordered(&[1, 2, 3]), vec![1, 3, 2]);
    }

    #[test]
    fn long_list() {
        let values: Vec<i32> = (0..5_000).collect();
        let result = reordered(&values);
        let mut expected = Vec::new();
        let (mut lo, mut hi) = (0, 4_999);
        while lo <= hi {
            expected.push(lo);
            if lo != hi {
                expected.push(hi);
            }
            lo += 1;
            hi -= 1;
        }
        assert_eq!(result, expected);
    }
}
