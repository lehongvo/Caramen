// ==================================================================
// PRACTICE 0660: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0660_reverse_nodes_in_k_group.md
// Run the tests:          cargo test practice::p0660
// ==================================================================

//! # Problem 0660: Reverse Nodes in k-Group
//!
//! - **Difficulty:** Hard
//! - **Topics:** detaching groups of nodes, reversing by prepending, splicing groups with a tail cursor
//! - **Source:** [LeetCode 25. Reverse Nodes in k-Group](https://leetcode.com/problems/reverse-nodes-in-k-group/)
//!
//! ## Problem
//!
//! Reverse the nodes of a linked list `k` at a time: the first `k` nodes are reversed, then the
//! next `k`, and so on. If fewer than `k` nodes remain at the end, they keep their order. Move the
//! nodes; do not change their values. Return the new head.
//!
//! Implement `Solution::reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 2, 3, 4, 5], k = 2
//! Output: [2, 1, 4, 3, 5]
//!
//! Input:  head = [1, 2, 3, 4, 5], k = 3
//! Output: [3, 2, 1, 4, 5]
//!
//! Input:  head = [1, 2, 3], k = 1
//! Output: [1, 2, 3]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= k <= n <= 5000`, `0 <= val <= 1000`
//!
//! ## Hints
//!
//! 1. Count the nodes first: you then know how many full groups there are (`n / k`).
//! 2. For each full group, move `k` nodes from the input onto the front of a `group` list; that
//!    reverses them (problem 0630).
//! 3. Attach the group at the result's tail cursor, then walk the cursor to the end of the group.
//!    After the last full group, attach whatever is left of the input unchanged.

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn groups_of_two() {
        let result = Solution::reverse_k_group(list(&[1, 2, 3, 4, 5]), 2);
        assert_eq!(list_to_vec(&result), vec![2, 1, 4, 3, 5]);
    }

    #[test]
    fn groups_of_three() {
        let result = Solution::reverse_k_group(list(&[1, 2, 3, 4, 5]), 3);
        assert_eq!(list_to_vec(&result), vec![3, 2, 1, 4, 5]);
    }

    #[test]
    fn k_of_one_and_k_of_n() {
        assert_eq!(
            Solution::reverse_k_group(list(&[1, 2, 3]), 1),
            list(&[1, 2, 3])
        );
        assert_eq!(
            Solution::reverse_k_group(list(&[1, 2, 3]), 3),
            list(&[3, 2, 1])
        );
        assert_eq!(Solution::reverse_k_group(list(&[7]), 1), list(&[7]));
    }

    #[test]
    fn long_list_with_a_partial_group() {
        let values: Vec<i32> = (0..5_000).collect();
        let result = list_to_vec(&Solution::reverse_k_group(list(&values), 7));
        let mut expected = Vec::new();
        for chunk in values.chunks(7) {
            if chunk.len() == 7 {
                expected.extend(chunk.iter().rev());
            } else {
                expected.extend(chunk);
            }
        }
        assert_eq!(result, expected);
    }
}
