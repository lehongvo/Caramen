// ==================================================================
// PRACTICE 0632: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0632_remove_duplicates_from_sorted_list.md
// Run the tests:          cargo test practice::p0632
// ==================================================================

//! # Problem 0632: Remove Duplicates from Sorted List
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Option::as_mut` cursor, unlinking with `take`, `Option::is_some_and`
//! - **Source:** [LeetCode 83. Remove Duplicates from Sorted List](https://leetcode.com/problems/remove-duplicates-from-sorted-list/)
//!
//! ## Problem
//!
//! A linked list is sorted in non-decreasing order. Remove nodes so that every value appears
//! **exactly once**, keeping the first node of each run of equal values, and return the head of
//! the resulting list (which is still sorted).
//!
//! Implement `Solution::delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 1, 2]
//! Output: [1, 2]
//!
//! Input:  head = [1, 1, 2, 3, 3]
//! Output: [1, 2, 3]
//!
//! Input:  head = []
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 300`, `-100 <= val <= 100`
//! - The list is sorted in non-decreasing order.
//!
//! ## Hints
//!
//! 1. Walk with `let mut current = head.as_mut();` (an `Option<&mut Box<ListNode>>`).
//! 2. While the node after `node` has the same value, unlink it:
//!    `node.next = node.next.take().and_then(|dup| dup.next);`
//! 3. Then move on: `current = node.next.as_mut();`.

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn example_one() {
        let result = Solution::delete_duplicates(list(&[1, 1, 2]));
        assert_eq!(list_to_vec(&result), vec![1, 2]);
    }

    #[test]
    fn example_two() {
        let result = Solution::delete_duplicates(list(&[1, 1, 2, 3, 3]));
        assert_eq!(list_to_vec(&result), vec![1, 2, 3]);
    }

    #[test]
    fn empty_and_single() {
        assert_eq!(Solution::delete_duplicates(None), None);
        assert_eq!(Solution::delete_duplicates(list(&[7])), list(&[7]));
    }

    #[test]
    fn all_equal_and_no_duplicates() {
        assert_eq!(Solution::delete_duplicates(list(&[4; 300])), list(&[4]));
        let distinct: Vec<i32> = (-100..100).collect();
        assert_eq!(
            list_to_vec(&Solution::delete_duplicates(list(&distinct))),
            distinct
        );
    }
}
