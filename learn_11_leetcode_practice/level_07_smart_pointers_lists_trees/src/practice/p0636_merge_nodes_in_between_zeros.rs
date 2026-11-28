// ==================================================================
// PRACTICE 0636: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0636_merge_nodes_in_between_zeros.md
// Run the tests:          cargo test practice::p0636
// ==================================================================

//! # Problem 0636: Merge Nodes in Between Zeros
//!
//! - **Difficulty:** Medium
//! - **Topics:** reading with `as_deref`, building with a tail cursor, running sums
//! - **Source:** [LeetCode 2181. Merge Nodes in Between Zeros](https://leetcode.com/problems/merge-nodes-in-between-zeros/)
//!
//! ## Problem
//!
//! A linked list starts with a `0`, ends with a `0`, and never has two `0`s next to each other.
//! The zeros cut the list into groups of positive numbers. Return a new list with one node per
//! group, holding the **sum** of that group, in order. The result contains no zeros.
//!
//! Implement `Solution::merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [0, 3, 1, 0, 4, 5, 2, 0]
//! Output: [4, 11]          // 3 + 1, then 4 + 5 + 2
//!
//! Input:  head = [0, 1, 0, 3, 0, 2, 2, 0]
//! Output: [1, 3, 4]
//! ```
//!
//! ## Constraints
//!
//! - `3 <= n <= 2 * 10^5`, `0 <= val <= 1000`
//! - No two consecutive nodes are `0`; the first and last nodes are `0`.
//!
//! ## Hints
//!
//! 1. Skip the first zero, then keep a running `sum`.
//! 2. On every zero, append a node holding `sum` to the result (tail cursor from problem 0627) and
//!    reset `sum` to `0`.

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn example_one() {
        let result = Solution::merge_nodes(list(&[0, 3, 1, 0, 4, 5, 2, 0]));
        assert_eq!(list_to_vec(&result), vec![4, 11]);
    }

    #[test]
    fn example_two() {
        let result = Solution::merge_nodes(list(&[0, 1, 0, 3, 0, 2, 2, 0]));
        assert_eq!(list_to_vec(&result), vec![1, 3, 4]);
    }

    #[test]
    fn single_group() {
        assert_eq!(Solution::merge_nodes(list(&[0, 1000, 0])), list(&[1000]));
    }

    #[test]
    fn many_groups() {
        let mut values = vec![0];
        for i in 1..=1_000 {
            values.extend([i % 7 + 1, 1, 0]);
        }
        let result = list_to_vec(&Solution::merge_nodes(list(&values)));
        assert_eq!(result.len(), 1_000);
        // Group i holds (i % 7 + 1) and 1.
        assert_eq!(result[0], 3);
        assert_eq!(result[5], 8);
        assert_eq!(result[6], 2);
    }
}
