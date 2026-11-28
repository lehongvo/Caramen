// ==================================================================
// PRACTICE 0642: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0642_add_two_numbers_ii.md
// Run the tests:          cargo test practice::p0642
// ==================================================================

//! # Problem 0642: Add Two Numbers II
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Vec` as a stack of digits, prepending nodes (`next: result`), carry arithmetic
//! - **Source:** [LeetCode 445. Add Two Numbers II](https://leetcode.com/problems/add-two-numbers-ii/)
//!
//! ## Problem
//!
//! Two non-negative integers are stored as linked lists of decimal digits, this time with the
//! **most significant digit first**. Return their sum as a list in the same order. The numbers
//! have no leading zeros, except `0` itself.
//!
//! Implement
//! `Solution::add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  l1 = [7, 2, 4, 3], l2 = [5, 6, 4]
//! Output: [7, 8, 0, 7]              // 7243 + 564 = 7807
//!
//! Input:  l1 = [2, 4, 3], l2 = [5, 6, 4]
//! Output: [8, 0, 7]
//!
//! Input:  l1 = [0], l2 = [0]
//! Output: [0]
//! ```
//!
//! ## Constraints
//!
//! - Each list has `1` to `100` nodes, and every `val` is a digit `0..=9`.
//! - The numbers have no leading zeros (except `0` itself).
//!
//! ## Hints
//!
//! 1. Addition starts at the ones digit, which is at the **end** here. Copy the digits of each
//!    list into a `Vec`; `pop()` then yields them from the least significant one.
//! 2. The result digits are also produced least significant first, so **prepend** each new node:
//!    `result = Some(Box::new(ListNode { val: digit, next: result }));`.

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn example_one() {
        let sum = Solution::add_two_numbers(list(&[7, 2, 4, 3]), list(&[5, 6, 4]));
        assert_eq!(list_to_vec(&sum), vec![7, 8, 0, 7]);
    }

    #[test]
    fn example_two() {
        let sum = Solution::add_two_numbers(list(&[2, 4, 3]), list(&[5, 6, 4]));
        assert_eq!(list_to_vec(&sum), vec![8, 0, 7]);
    }

    #[test]
    fn zeros() {
        assert_eq!(
            Solution::add_two_numbers(list(&[0]), list(&[0])),
            list(&[0])
        );
    }

    #[test]
    fn carry_creates_a_new_leading_digit() {
        let sum = Solution::add_two_numbers(list(&[9; 100]), list(&[1]));
        let mut expected = vec![1];
        expected.extend([0; 100]);
        assert_eq!(list_to_vec(&sum), expected);
    }
}
