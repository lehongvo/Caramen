// ==================================================================
// PRACTICE 0633: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0633_remove_linked_list_elements.md
// Run the tests:          cargo test practice::p0633
// ==================================================================

//! # Problem 0633: Remove Linked List Elements
//!
//! - **Difficulty:** Easy
//! - **Topics:** rebuilding a list with a tail cursor, `Option::take`, why the "cursor on the current slot" loop fights the borrow checker
//! - **Source:** [LeetCode 203. Remove Linked List Elements](https://leetcode.com/problems/remove-linked-list-elements/)
//!
//! ## Problem
//!
//! Given the head of a linked list and an integer `val`, remove **every** node whose value equals
//! `val` and return the new head. The remaining nodes keep their order. The head itself may be
//! removed, and so may all nodes.
//!
//! Implement `Solution::remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 2, 6, 3, 4, 5, 6], val = 6
//! Output: [1, 2, 3, 4, 5]
//!
//! Input:  head = [], val = 1
//! Output: []
//!
//! Input:  head = [7, 7, 7, 7], val = 7
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 10^4`, `1 <= node.val <= 50`, `0 <= val <= 50`
//!
//! ## Hints
//!
//! 1. Instead of unlinking in place, move every node into a new list and only *keep* the ones
//!    you want: `while let Some(mut node) = rest { rest = node.next.take(); ... }`.
//! 2. Append kept nodes with a tail cursor: `tail = &mut tail.insert(node).next;`. A rejected
//!    `node` is simply dropped at the end of the loop body.

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn example_one() {
        let result = Solution::remove_elements(list(&[1, 2, 6, 3, 4, 5, 6]), 6);
        assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn empty_list() {
        assert_eq!(Solution::remove_elements(None, 1), None);
    }

    #[test]
    fn every_node_removed() {
        assert_eq!(Solution::remove_elements(list(&[7, 7, 7, 7]), 7), None);
    }

    #[test]
    fn head_removed_and_value_absent() {
        let result = Solution::remove_elements(list(&[2, 2, 1, 2]), 2);
        assert_eq!(list_to_vec(&result), vec![1]);
        let result = Solution::remove_elements(list(&[1, 2, 3]), 0);
        assert_eq!(list_to_vec(&result), vec![1, 2, 3]);
    }

    #[test]
    fn long_alternating_list() {
        let values: Vec<i32> = (0..5_000)
            .map(|i| if i % 2 == 0 { 9 } else { i % 50 + 1 })
            .collect();
        let result = Solution::remove_elements(list(&values), 9);
        let expected: Vec<i32> = values.into_iter().filter(|&v| v != 9).collect();
        assert_eq!(list_to_vec(&result), expected);
    }
}
