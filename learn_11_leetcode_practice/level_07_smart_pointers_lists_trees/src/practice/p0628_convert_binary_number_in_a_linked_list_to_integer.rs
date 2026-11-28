// ==================================================================
// PRACTICE 0628: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0628_convert_binary_number_in_a_linked_list_to_integer.md
// Run the tests:          cargo test practice::p0628
// ==================================================================

//! # Problem 0628: Convert Binary Number in a Linked List to Integer
//!
//! - **Difficulty:** Easy
//! - **Topics:** walking `Option<Box<ListNode>>` with `as_deref`, bit shifting, `std::iter::successors`
//! - **Source:** [LeetCode 1290. Convert Binary Number in a Linked List to Integer](https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/)
//!
//! ## Problem
//!
//! A linked list stores the bits of a non-negative binary number, **most significant bit first**:
//! every node's `val` is `0` or `1`. Return the number as an `i32`.
//!
//! Implement `Solution::get_decimal_value(head: Option<Box<ListNode>>) -> i32`
//! (`ListNode` is in `crate::common`).
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 0, 1]
//! Output: 5          // binary 101
//!
//! Input:  head = [0]
//! Output: 0
//!
//! Input:  head = [1, 1, 1, 1]
//! Output: 15
//! ```
//!
//! ## Constraints
//!
//! - The list is not empty and has at most `30` nodes.
//! - Every `val` is `0` or `1`.
//!
//! ## Hints
//!
//! 1. Reading digits from the most significant end: `result = result * 2 + bit`
//!    (or `(result << 1) | bit`).
//! 2. `head.as_deref()` gives an `Option<&ListNode>` to walk the list without consuming it.

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::list;

    #[test]
    fn example_101() {
        assert_eq!(Solution::get_decimal_value(list(&[1, 0, 1])), 5);
    }

    #[test]
    fn single_zero_and_one() {
        assert_eq!(Solution::get_decimal_value(list(&[0])), 0);
        assert_eq!(Solution::get_decimal_value(list(&[1])), 1);
    }

    #[test]
    fn all_ones() {
        assert_eq!(Solution::get_decimal_value(list(&[1, 1, 1, 1])), 15);
    }

    #[test]
    fn leading_zeros_and_thirty_bits() {
        assert_eq!(Solution::get_decimal_value(list(&[0, 0, 1, 0])), 2);
        let bits = [1; 30];
        assert_eq!(Solution::get_decimal_value(list(&bits)), (1 << 30) - 1);
    }
}
