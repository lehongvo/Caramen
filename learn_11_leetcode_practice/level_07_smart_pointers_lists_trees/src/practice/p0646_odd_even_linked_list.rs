// ==================================================================
// PRACTICE 0646: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0646_odd_even_linked_list.md
// Run the tests:          cargo test practice::p0646
// ==================================================================

//! # Problem 0646: Odd Even Linked List
//!
//! - **Difficulty:** Medium
//! - **Topics:** splitting by position with two tail cursors, `Option::insert`, a flipping `bool`, joining lists
//! - **Source:** [LeetCode 328. Odd Even Linked List](https://leetcode.com/problems/odd-even-linked-list/)
//!
//! ## Problem
//!
//! Number the nodes of a linked list by **position**, starting at `1`. Reorder the list so that
//! all nodes at odd positions come first, followed by all nodes at even positions; each group keeps
//! its original order. (The node values do not matter, only their positions.) Use `O(1)` extra
//! space besides the nodes themselves.
//!
//! Implement `Solution::odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 2, 3, 4, 5]
//! Output: [1, 3, 5, 2, 4]
//!
//! Input:  head = [2, 1, 3, 5, 6, 4, 7]
//! Output: [2, 3, 6, 7, 1, 5, 4]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 10^4`, `-10^6 <= val <= 10^6`
//!
//! ## Hints
//!
//! 1. This is problem 0645 with a different rule: the position decides the group, not the value.
//! 2. Keep a counter (or a `bool` that flips) while you detach the nodes one by one.

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn example_one() {
        let result = Solution::odd_even_list(list(&[1, 2, 3, 4, 5]));
        assert_eq!(list_to_vec(&result), vec![1, 3, 5, 2, 4]);
    }

    #[test]
    fn example_two() {
        let result = Solution::odd_even_list(list(&[2, 1, 3, 5, 6, 4, 7]));
        assert_eq!(list_to_vec(&result), vec![2, 3, 6, 7, 1, 5, 4]);
    }

    #[test]
    fn short_lists() {
        assert_eq!(Solution::odd_even_list(None), None);
        assert_eq!(Solution::odd_even_list(list(&[1])), list(&[1]));
        assert_eq!(Solution::odd_even_list(list(&[1, 2])), list(&[1, 2]));
    }

    #[test]
    fn long_list() {
        let values: Vec<i32> = (1..=5_000).collect();
        let result = list_to_vec(&Solution::odd_even_list(list(&values)));
        let mut expected: Vec<i32> = (1..=5_000).step_by(2).collect();
        expected.extend((2..=5_000).step_by(2));
        assert_eq!(result, expected);
    }
}
