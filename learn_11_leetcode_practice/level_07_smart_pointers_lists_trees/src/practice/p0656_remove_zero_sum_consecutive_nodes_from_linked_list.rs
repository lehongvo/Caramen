// ==================================================================
// PRACTICE 0656: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0656_remove_zero_sum_consecutive_nodes_from_linked_list.md
// Run the tests:          cargo test practice::p0656
// ==================================================================

//! # Problem 0656: Remove Zero Sum Consecutive Nodes from Linked List
//!
//! - **Difficulty:** Medium
//! - **Topics:** prefix sums, `HashMap` of last positions, rebuilding a list from kept values
//! - **Source:** [LeetCode 1171. Remove Zero Sum Consecutive Nodes from Linked List](https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/)
//!
//! ## Problem
//!
//! Repeatedly delete any run of **consecutive** nodes whose values sum to `0`, until no such run
//! is left. Return the head of the final list. Several final lists may be possible; any of them is
//! accepted.
//!
//! Implement
//! `Solution::remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 2, -3, 3, 1]
//! Output: [3, 1]          // [1, 2, 1] is also accepted
//!
//! Input:  head = [1, 2, 3, -3, 4]
//! Output: [1, 2, 4]
//!
//! Input:  head = [1, 2, 3, -3, -2]
//! Output: [1]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 1000`, `-1000 <= val <= 1000`
//!
//! ## Hints
//!
//! 1. Let `prefix[i]` be the sum of the first `i` values (`prefix[0] = 0`). A run from position
//!    `i` to `j - 1` sums to zero exactly when `prefix[i] == prefix[j]`.
//! 2. Record, for every prefix sum, the **last** position where it occurs.
//! 3. Walk from position `0`: jump to the last position with the same prefix sum (skipping a
//!    zero-sum run), keep the value there, and continue after it.

use crate::common::ListNode;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    /// Checks that `output` is a valid final list for `input`.
    fn assert_valid(input: &[i32], output: &[i32]) {
        // Nothing left to remove: no consecutive run sums to zero.
        for start in 0..output.len() {
            let mut sum = 0;
            for &value in &output[start..] {
                sum += value;
                assert_ne!(sum, 0, "{output:?} still has a zero-sum run");
            }
        }
        // Only zero-sum runs were removed: the total is unchanged, and the order is kept.
        assert_eq!(input.iter().sum::<i32>(), output.iter().sum::<i32>());
        let mut remaining = input.iter();
        assert!(
            output.iter().all(|v| remaining.any(|x| x == v)),
            "{output:?} is not a subsequence of {input:?}"
        );
    }

    fn run(input: &[i32]) -> Vec<i32> {
        let output = list_to_vec(&Solution::remove_zero_sum_sublists(list(input)));
        assert_valid(input, &output);
        output
    }

    #[test]
    fn example_one() {
        let output = run(&[1, 2, -3, 3, 1]);
        assert!(
            output == vec![3, 1] || output == vec![1, 2, 1],
            "{output:?}"
        );
    }

    #[test]
    fn example_two_and_three() {
        assert_eq!(run(&[1, 2, 3, -3, 4]), vec![1, 2, 4]);
        assert_eq!(run(&[1, 2, 3, -3, -2]), vec![1]);
    }

    #[test]
    fn everything_cancels() {
        assert_eq!(run(&[0]), Vec::<i32>::new());
        assert_eq!(run(&[5, -5, 3, -3]), Vec::<i32>::new());
    }

    #[test]
    fn long_mixed_input() {
        let input: Vec<i32> = (0..1_000).map(|i| (i * 37) % 21 - 10).collect();
        run(&input);
    }
}
