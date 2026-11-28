//! # Problem 0648: Delete Nodes From Linked List Present in Array
//!
//! - **Difficulty:** Medium
//! - **Topics:** `HashSet` lookups, filtering a list while moving its nodes, tail cursor
//! - **Source:** [LeetCode 3217. Delete Nodes From Linked List Present in Array](https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/)
//!
//! ## Problem
//!
//! You are given an array `nums` and the head of a linked list. Remove every node whose value
//! appears in `nums`, keep the other nodes in their original order, and return the new head.
//!
//! Implement
//! `Solution::modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 3], head = [1, 2, 3, 4, 5]
//! Output: [4, 5]
//!
//! Input:  nums = [1], head = [1, 2, 1, 2, 1, 2]
//! Output: [2, 2, 2]
//!
//! Input:  nums = [5], head = [1, 2, 3, 4]
//! Output: [1, 2, 3, 4]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`, `1 <= nums[i] <= 10^5`, the values of `nums` are distinct.
//! - `1 <= n <= 10^5`, `1 <= val <= 10^5`
//! - At least one node is **not** removed.
//!
//! ## Hints
//!
//! 1. Checking `nums.contains(&v)` for every node is `O(n * m)`. Put `nums` into a `HashSet` first.
//! 2. Then filter the list exactly as in problem 0633.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** build a `HashSet<i32>` from `nums`, then move every node of the input to the result
//! unless its value is in the set.
//!
//! **Complexity:** `O(n + m)` expected time, `O(m)` extra space for the set.
//!
//! **Rust concepts:**
//! - `nums.into_iter().collect::<HashSet<_>>()` consumes the vector; it is not needed afterwards.
//! - Removed nodes are dropped at the end of the loop body, so memory is freed as you go.
// @solution-end

use crate::common::ListNode;
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // @solution-start
        let banned: HashSet<i32> = nums.into_iter().collect();
        let mut result = None;
        let mut tail = &mut result;
        let mut rest = head;
        while let Some(mut node) = rest {
            rest = node.next.take();
            if !banned.contains(&node.val) {
                tail = &mut tail.insert(node).next;
            }
        }
        result
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn example_one() {
        let result = Solution::modified_list(vec![1, 2, 3], list(&[1, 2, 3, 4, 5]));
        assert_eq!(list_to_vec(&result), vec![4, 5]);
    }

    #[test]
    fn example_two() {
        let result = Solution::modified_list(vec![1], list(&[1, 2, 1, 2, 1, 2]));
        assert_eq!(list_to_vec(&result), vec![2, 2, 2]);
    }

    #[test]
    fn nothing_removed() {
        let result = Solution::modified_list(vec![5], list(&[1, 2, 3, 4]));
        assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4]);
    }

    #[test]
    fn large_inputs() {
        let nums: Vec<i32> = (1..=5_000).filter(|v| v % 3 != 0).collect();
        let values: Vec<i32> = (1..=5_000).collect();
        let result = list_to_vec(&Solution::modified_list(nums, list(&values)));
        let expected: Vec<i32> = (1..=5_000).filter(|v| v % 3 == 0).collect();
        assert_eq!(result, expected);
    }
}
