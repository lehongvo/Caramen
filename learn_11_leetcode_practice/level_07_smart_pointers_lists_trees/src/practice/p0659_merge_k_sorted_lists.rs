// ==================================================================
// PRACTICE 0659: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0659_merge_k_sorted_lists.md
// Run the tests:          cargo test practice::p0659
// ==================================================================

//! # Problem 0659: Merge k Sorted Lists
//!
//! - **Difficulty:** Hard
//! - **Topics:** `BinaryHeap` with `std::cmp::Reverse` as a min-heap, moving nodes out of a `Vec<Option<Box<ListNode>>>`, tail cursor
//! - **Source:** [LeetCode 23. Merge k Sorted Lists](https://leetcode.com/problems/merge-k-sorted-lists/)
//!
//! ## Problem
//!
//! You are given `k` linked lists, each sorted in ascending order. Merge them all into one sorted
//! list (reusing the nodes) and return its head.
//!
//! Implement `Solution::merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  lists = [[1, 4, 5], [1, 3, 4], [2, 6]]
//! Output: [1, 1, 2, 3, 4, 4, 5, 6]
//!
//! Input:  lists = []
//! Output: []
//!
//! Input:  lists = [[]]
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= k <= 10^4`; each list has at most `500` nodes; at most `10^4` nodes in total.
//! - `-10^4 <= val <= 10^4`; every list is sorted in ascending order.
//!
//! ## Hints
//!
//! 1. The next node of the result is the smallest **head** among the `k` lists. A min-heap finds it
//!    in `O(log k)`.
//! 2. `BinaryHeap` is a max-heap; wrap entries in `std::cmp::Reverse` to pop the smallest first.
//! 3. Store `(value, list index)` in the heap instead of the nodes: after popping index `i`, take
//!    the head of `lists[i]`, put its `next` back into `lists[i]`, and push the new head's value.

use crate::common::ListNode;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn example_one() {
        let lists = vec![list(&[1, 4, 5]), list(&[1, 3, 4]), list(&[2, 6])];
        let merged = Solution::merge_k_lists(lists);
        assert_eq!(list_to_vec(&merged), vec![1, 1, 2, 3, 4, 4, 5, 6]);
    }

    #[test]
    fn no_lists_and_empty_lists() {
        assert_eq!(Solution::merge_k_lists(vec![]), None);
        assert_eq!(Solution::merge_k_lists(vec![None]), None);
        assert_eq!(
            Solution::merge_k_lists(vec![None, list(&[1]), None]),
            list(&[1])
        );
    }

    #[test]
    fn negative_values_and_single_list() {
        let merged = Solution::merge_k_lists(vec![list(&[-3, 0, 2])]);
        assert_eq!(list_to_vec(&merged), vec![-3, 0, 2]);
        let merged = Solution::merge_k_lists(vec![list(&[-1, 5]), list(&[-10_000, 10_000])]);
        assert_eq!(list_to_vec(&merged), vec![-10_000, -1, 5, 10_000]);
    }

    #[test]
    fn many_lists() {
        // 1000 lists: list i holds i, i + 1000, i + 2000, ...
        let lists: Vec<Option<Box<ListNode>>> = (0..1_000)
            .map(|i| list(&(0..5).map(|j| i + j * 1_000).collect::<Vec<_>>()))
            .collect();
        let merged = list_to_vec(&Solution::merge_k_lists(lists));
        assert_eq!(merged, (0..5_000).collect::<Vec<_>>());
    }
}
