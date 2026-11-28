//! # Problem 0639: Maximum Twin Sum of a Linked List
//!
//! - **Difficulty:** Medium
//! - **Topics:** list to `Vec`, pairing the front and back halves with `zip` and `rev`, `Iterator::max`
//! - **Source:** [LeetCode 2130. Maximum Twin Sum of a Linked List](https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/)
//!
//! ## Problem
//!
//! A linked list has an **even** number of nodes `n`. Node `i` (0-based) and node `n - 1 - i` are
//! *twins*: the first and the last, the second and the second to last, and so on. The twin sum of
//! a pair is the sum of their values. Return the largest twin sum.
//!
//! Implement `Solution::pair_sum(head: Option<Box<ListNode>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [5, 4, 2, 1]
//! Output: 6          // twins (5, 1) and (4, 2) both sum to 6
//!
//! Input:  head = [4, 2, 2, 3]
//! Output: 7          // (4, 3) = 7, (2, 2) = 4
//!
//! Input:  head = [1, 100000]
//! Output: 100001
//! ```
//!
//! ## Constraints
//!
//! - `n` is even and `2 <= n <= 10^5`; `1 <= val <= 10^5`
//!
//! ## Hints
//!
//! 1. Copy the values into a `Vec`; then twins are `values[i]` and `values[n - 1 - i]`.
//! 2. `values.iter().zip(values.iter().rev()).take(n / 2)` produces every twin pair once.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** collect the values, pair the front half with the reversed back half and take the
//! maximum sum. (An `O(1)` space alternative reverses the second half of the list in place.)
//!
//! **Complexity:** `O(n)` time, `O(n)` extra space.
//!
//! **Rust concepts:**
//! - `Iterator::max` returns an `Option`; the list is guaranteed non-empty, so `unwrap_or(0)` is
//!   only a formality.
//! - The list is only borrowed (`as_deref`) while its values are copied out.
// @solution-end

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        // @solution-start
        let values: Vec<i32> = std::iter::successors(head.as_deref(), |node| node.next.as_deref())
            .map(|node| node.val)
            .collect();
        values
            .iter()
            .zip(values.iter().rev())
            .take(values.len() / 2)
            .map(|(a, b)| a + b)
            .max()
            .unwrap_or(0)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::list;

    #[test]
    fn example_one() {
        assert_eq!(Solution::pair_sum(list(&[5, 4, 2, 1])), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::pair_sum(list(&[4, 2, 2, 3])), 7);
    }

    #[test]
    fn two_nodes() {
        assert_eq!(Solution::pair_sum(list(&[1, 100_000])), 100_001);
    }

    #[test]
    fn best_pair_in_the_middle() {
        let mut values = vec![1; 4_000];
        values[1_999] = 100_000;
        values[2_000] = 99_999;
        assert_eq!(Solution::pair_sum(list(&values)), 199_999);
    }
}
