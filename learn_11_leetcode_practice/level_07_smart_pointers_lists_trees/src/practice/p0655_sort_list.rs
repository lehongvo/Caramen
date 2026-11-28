// ==================================================================
// PRACTICE 0655: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0655_sort_list.md
// Run the tests:          cargo test practice::p0655
// ==================================================================

//! # Problem 0655: Sort List
//!
//! - **Difficulty:** Medium
//! - **Topics:** merge sort on owned lists, splitting by length, recursive helpers inside `impl Solution`
//! - **Source:** [LeetCode 148. Sort List](https://leetcode.com/problems/sort-list/)
//!
//! ## Problem
//!
//! Sort a linked list in ascending order in `O(n log n)` time and return its head. Do it by
//! re-linking the nodes (do not copy the values into a `Vec` and sort that).
//!
//! Implement `Solution::sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [4, 2, 1, 3]
//! Output: [1, 2, 3, 4]
//!
//! Input:  head = [-1, 5, 3, 4, 0]
//! Output: [-1, 0, 3, 4, 5]
//!
//! Input:  head = []
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 5 * 10^4`, `-10^5 <= val <= 10^5`
//!
//! ## Hints
//!
//! 1. Merge sort fits linked lists well: merging two sorted lists needs no extra memory (problem
//!    0631), and splitting needs no random access.
//! 2. Write `sort(head, len)`: if `len <= 1` the list is sorted; otherwise cut it after `len / 2`
//!    nodes, sort both halves, merge them.
//! 3. Passing the length down avoids counting each half again.

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn example_one() {
        let result = Solution::sort_list(list(&[4, 2, 1, 3]));
        assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4]);
    }

    #[test]
    fn example_two() {
        let result = Solution::sort_list(list(&[-1, 5, 3, 4, 0]));
        assert_eq!(list_to_vec(&result), vec![-1, 0, 3, 4, 5]);
    }

    #[test]
    fn empty_single_and_duplicates() {
        assert_eq!(Solution::sort_list(None), None);
        assert_eq!(Solution::sort_list(list(&[9])), list(&[9]));
        assert_eq!(
            Solution::sort_list(list(&[2, 1, 2, 1])),
            list(&[1, 1, 2, 2])
        );
    }

    #[test]
    fn large_pseudo_random_input() {
        let values: Vec<i32> = (0..5_000)
            .map(|i| (i * 48_271) % 200_001 - 100_000)
            .collect();
        let mut expected = values.clone();
        expected.sort();
        let result = Solution::sort_list(list(&values));
        assert_eq!(list_to_vec(&result), expected);
    }
}
