//! # Problem 0650: Next Greater Node In Linked List
//!
//! - **Difficulty:** Medium
//! - **Topics:** list to `Vec`, monotonic stack of indices, `while let Some(&top) = waiting.last()`
//! - **Source:** [LeetCode 1019. Next Greater Node In Linked List](https://leetcode.com/problems/next-greater-node-in-linked-list/)
//!
//! ## Problem
//!
//! For every node of a linked list, find the value of the **first** node after it whose value is
//! strictly larger. Return these answers as a `Vec<i32>` in node order, using `0` when no larger
//! node follows.
//!
//! Implement `Solution::next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [2, 1, 5]
//! Output: [5, 5, 0]
//!
//! Input:  head = [2, 7, 4, 3, 5]
//! Output: [7, 0, 5, 5, 0]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 10^4`, `1 <= val <= 10^9`
//!
//! ## Hints
//!
//! 1. Copy the values into a `Vec` so you can index them.
//! 2. Keep a stack of indices that are still waiting for their answer; their values are
//!    non-increasing from bottom to top.
//! 3. When a new value is larger than the value at the top index, it is the answer for that index:
//!    pop and record, and repeat. Then push the new index.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the classic monotonic stack. Each index is pushed once and popped at most once, at the
//! moment its next greater value appears. Indices never popped keep the answer `0`.
//!
//! **Complexity:** `O(n)` time, `O(n)` extra space.
//!
//! **Rust concepts:**
//! - `while let Some(&top) = waiting.last()` peeks without borrowing the stack beyond the condition,
//!   so the loop body can `pop`.
//! - `vec![0; n]` pre-fills the default answer.
// @solution-end

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        // @solution-start
        let values: Vec<i32> = std::iter::successors(head.as_deref(), |node| node.next.as_deref())
            .map(|node| node.val)
            .collect();
        let mut answer = vec![0; values.len()];
        let mut waiting: Vec<usize> = Vec::new();
        for (i, &value) in values.iter().enumerate() {
            while let Some(&top) = waiting.last() {
                if values[top] >= value {
                    break;
                }
                answer[top] = value;
                waiting.pop();
            }
            waiting.push(i);
        }
        answer
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::list;

    #[test]
    fn example_one() {
        assert_eq!(Solution::next_larger_nodes(list(&[2, 1, 5])), vec![5, 5, 0]);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::next_larger_nodes(list(&[2, 7, 4, 3, 5])),
            vec![7, 0, 5, 5, 0]
        );
    }

    #[test]
    fn equal_values_are_not_greater() {
        assert_eq!(Solution::next_larger_nodes(list(&[3, 3, 3])), vec![0, 0, 0]);
        assert_eq!(Solution::next_larger_nodes(list(&[1])), vec![0]);
    }

    #[test]
    fn decreasing_then_peak() {
        let mut values: Vec<i32> = (1..=5_000).rev().collect();
        values.push(1_000_000_000);
        let answer = Solution::next_larger_nodes(list(&values));
        assert!(answer[..5_000].iter().all(|&v| v == 1_000_000_000));
        assert_eq!(answer[5_000], 0);
    }
}
