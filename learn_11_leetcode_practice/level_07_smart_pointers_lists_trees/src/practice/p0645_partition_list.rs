// ==================================================================
// PRACTICE 0645: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0645_partition_list.md
// Run the tests:          cargo test practice::p0645
// ==================================================================

//! # Problem 0645: Partition List
//!
//! - **Difficulty:** Medium
//! - **Topics:** two output lists with two tail cursors, stable partition, joining lists
//! - **Source:** [LeetCode 86. Partition List](https://leetcode.com/problems/partition-list/)
//!
//! ## Problem
//!
//! Rearrange a linked list so that every node with a value **less than** `x` comes before every
//! node with a value **greater than or equal to** `x`. Inside each of the two groups the nodes keep
//! their original relative order. Return the new head.
//!
//! Implement `Solution::partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 4, 3, 2, 5, 2], x = 3
//! Output: [1, 2, 2, 4, 3, 5]
//!
//! Input:  head = [2, 1], x = 2
//! Output: [1, 2]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 200`, `-100 <= val <= 100`, `-200 <= x <= 200`
//!
//! ## Hints
//!
//! 1. Build two lists while you walk the input: `small` and `large`, each with its own tail cursor.
//! 2. At the end, attach `large` to the end of `small`: `*small_tail = large;`.

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn example_one() {
        let result = Solution::partition(list(&[1, 4, 3, 2, 5, 2]), 3);
        assert_eq!(list_to_vec(&result), vec![1, 2, 2, 4, 3, 5]);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::partition(list(&[2, 1]), 2), list(&[1, 2]));
    }

    #[test]
    fn all_on_one_side() {
        assert_eq!(Solution::partition(list(&[3, 1, 2]), 10), list(&[3, 1, 2]));
        assert_eq!(Solution::partition(list(&[3, 1, 2]), -10), list(&[3, 1, 2]));
        assert_eq!(Solution::partition(None, 0), None);
    }

    #[test]
    fn stable_for_many_values() {
        let values: Vec<i32> = (0..200).map(|i| (i * 37) % 201 - 100).collect();
        let result = list_to_vec(&Solution::partition(list(&values), 0));
        let mut expected: Vec<i32> = values.iter().copied().filter(|&v| v < 0).collect();
        expected.extend(values.iter().copied().filter(|&v| v >= 0));
        assert_eq!(result, expected);
    }
}
