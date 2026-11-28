// ==================================================================
// PRACTICE 0644: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0644_rotate_list.md
// Run the tests:          cargo test practice::p0644
// ==================================================================

//! # Problem 0644: Rotate List
//!
//! - **Difficulty:** Medium
//! - **Topics:** splitting a list with `take`, reattaching at the end, `k % len`
//! - **Source:** [LeetCode 61. Rotate List](https://leetcode.com/problems/rotate-list/)
//!
//! ## Problem
//!
//! Rotate a linked list to the right by `k` places: each rotation moves the last node to the
//! front. Return the new head.
//!
//! Implement `Solution::rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 2, 3, 4, 5], k = 2
//! Output: [4, 5, 1, 2, 3]
//!
//! Input:  head = [0, 1, 2], k = 4
//! Output: [2, 0, 1]          // rotating by 3 changes nothing, so 4 is the same as 1
//!
//! Input:  head = [], k = 7
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 500`, `-100 <= val <= 100`, `0 <= k <= 2 * 10^9`
//!
//! ## Hints
//!
//! 1. Rotating by the length gives the same list, so only `k % len` matters (and `k` can be huge).
//! 2. Rotating by `r` means: the last `r` nodes move, in order, to the front. Cut the list after
//!    node number `len - r` (`take()` its `next`), then attach the old head at the end of the cut
//!    piece.

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn rotate_by_two() {
        let result = Solution::rotate_right(list(&[1, 2, 3, 4, 5]), 2);
        assert_eq!(list_to_vec(&result), vec![4, 5, 1, 2, 3]);
    }

    #[test]
    fn k_larger_than_length() {
        let result = Solution::rotate_right(list(&[0, 1, 2]), 4);
        assert_eq!(list_to_vec(&result), vec![2, 0, 1]);
    }

    #[test]
    fn empty_and_full_rotations() {
        assert_eq!(Solution::rotate_right(None, 7), None);
        assert_eq!(
            Solution::rotate_right(list(&[1, 2, 3]), 3),
            list(&[1, 2, 3])
        );
        assert_eq!(
            Solution::rotate_right(list(&[1, 2, 3]), 0),
            list(&[1, 2, 3])
        );
        assert_eq!(Solution::rotate_right(list(&[5]), 99), list(&[5]));
    }

    #[test]
    fn huge_k() {
        let values: Vec<i32> = (0..500).map(|i| i % 201 - 100).collect();
        let k = 2_000_000_000;
        let r = k as usize % 500;
        let result = list_to_vec(&Solution::rotate_right(list(&values), k));
        let mut expected = values[500 - r..].to_vec();
        expected.extend_from_slice(&values[..500 - r]);
        assert_eq!(result, expected);
    }
}
