// ==================================================================
// PRACTICE 0639: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0639_maximum_twin_sum_of_a_linked_list.md
// Run the tests:          cargo test practice::p0639
// ==================================================================

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

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        todo!("Write your solution here")
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
