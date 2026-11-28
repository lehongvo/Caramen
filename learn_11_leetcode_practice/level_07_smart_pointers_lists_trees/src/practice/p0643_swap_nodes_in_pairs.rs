// ==================================================================
// PRACTICE 0643: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0643_swap_nodes_in_pairs.md
// Run the tests:          cargo test practice::p0643
// ==================================================================

//! # Problem 0643: Swap Nodes in Pairs
//!
//! - **Difficulty:** Medium
//! - **Topics:** detaching two nodes at a time, re-linking owned boxes, tail cursor
//! - **Source:** [LeetCode 24. Swap Nodes in Pairs](https://leetcode.com/problems/swap-nodes-in-pairs/)
//!
//! ## Problem
//!
//! Swap every two adjacent nodes of a linked list and return the new head: the 1st and 2nd nodes
//! change places, then the 3rd and 4th, and so on. A final node without a partner stays where it
//! is. Move the **nodes** themselves; do not just swap the values.
//!
//! Implement `Solution::swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 2, 3, 4]
//! Output: [2, 1, 4, 3]
//!
//! Input:  head = [1, 2, 3]
//! Output: [2, 1, 3]
//!
//! Input:  head = []
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 100`, `0 <= val <= 100`
//!
//! ## Hints
//!
//! 1. Take the nodes off the input two at a time: `first`, then `second = first.next.take()`.
//!    The rest of the list is `second.next.take()`.
//! 2. Append `second`, then `first`, to the result with a tail cursor.
//! 3. If `second` does not exist, append `first` alone and stop.

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn even_length() {
        let result = Solution::swap_pairs(list(&[1, 2, 3, 4]));
        assert_eq!(list_to_vec(&result), vec![2, 1, 4, 3]);
    }

    #[test]
    fn odd_length_keeps_last_node() {
        let result = Solution::swap_pairs(list(&[1, 2, 3]));
        assert_eq!(list_to_vec(&result), vec![2, 1, 3]);
    }

    #[test]
    fn empty_and_single() {
        assert_eq!(Solution::swap_pairs(None), None);
        assert_eq!(Solution::swap_pairs(list(&[1])), list(&[1]));
    }

    #[test]
    fn hundred_nodes() {
        let values: Vec<i32> = (0..100).collect();
        let result = list_to_vec(&Solution::swap_pairs(list(&values)));
        let expected: Vec<i32> = (0..100).map(|i| i ^ 1).collect();
        assert_eq!(result, expected);
    }
}
