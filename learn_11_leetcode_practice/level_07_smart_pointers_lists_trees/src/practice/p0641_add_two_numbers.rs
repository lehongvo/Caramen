// ==================================================================
// PRACTICE 0641: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0641_add_two_numbers.md
// Run the tests:          cargo test practice::p0641
// ==================================================================

//! # Problem 0641: Add Two Numbers
//!
//! - **Difficulty:** Medium
//! - **Topics:** consuming two lists with `Option::take`, carry arithmetic, tail cursor
//! - **Source:** [LeetCode 2. Add Two Numbers](https://leetcode.com/problems/add-two-numbers/)
//!
//! ## Problem
//!
//! Two non-negative integers are stored as linked lists of decimal digits in **reverse** order:
//! the head holds the ones digit, the next node the tens digit, and so on. Neither number has
//! leading zeros, except the number `0` itself. Return their sum as a list in the same format.
//!
//! Implement
//! `Solution::add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  l1 = [2, 4, 3], l2 = [5, 6, 4]
//! Output: [7, 0, 8]                 // 342 + 465 = 807
//!
//! Input:  l1 = [0], l2 = [0]
//! Output: [0]
//!
//! Input:  l1 = [9, 9, 9, 9, 9, 9, 9], l2 = [9, 9, 9, 9]
//! Output: [8, 9, 9, 9, 0, 0, 0, 1]  // the final carry adds a digit
//! ```
//!
//! ## Constraints
//!
//! - Each list has `1` to `100` nodes, and every `val` is a digit `0..=9`.
//! - The numbers have no leading zeros (except `0` itself).
//!
//! ## Hints
//!
//! 1. Add digit by digit, like on paper, keeping a `carry` (`0` or `1`).
//! 2. Keep going while **either** list has digits left **or** the carry is not zero.
//! 3. `if let Some(node) = a.take() { sum += node.val; a = node.next; }` consumes one digit.

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
        let sum = Solution::add_two_numbers(list(&[2, 4, 3]), list(&[5, 6, 4]));
        assert_eq!(list_to_vec(&sum), vec![7, 0, 8]);
    }

    #[test]
    fn zeros() {
        assert_eq!(
            Solution::add_two_numbers(list(&[0]), list(&[0])),
            list(&[0])
        );
    }

    #[test]
    fn final_carry_adds_a_digit() {
        let sum = Solution::add_two_numbers(list(&[9, 9, 9, 9, 9, 9, 9]), list(&[9, 9, 9, 9]));
        assert_eq!(list_to_vec(&sum), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }

    #[test]
    fn hundred_digit_numbers() {
        // 10^100 - 1 plus 1 = 10^100.
        let sum = Solution::add_two_numbers(list(&[9; 100]), list(&[1]));
        let mut expected = vec![0; 100];
        expected.push(1);
        assert_eq!(list_to_vec(&sum), expected);
    }
}
