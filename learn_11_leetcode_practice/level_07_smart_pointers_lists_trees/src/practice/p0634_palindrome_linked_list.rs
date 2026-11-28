// ==================================================================
// PRACTICE 0634: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0634_palindrome_linked_list.md
// Run the tests:          cargo test practice::p0634
// ==================================================================

//! # Problem 0634: Palindrome Linked List
//!
//! - **Difficulty:** Easy
//! - **Topics:** collecting a list into a `Vec`, `std::iter::successors`, comparing with a reversed iterator
//! - **Source:** [LeetCode 234. Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/)
//!
//! ## Problem
//!
//! Return `true` if the values of a singly linked list read the same forwards and backwards, and
//! `false` otherwise.
//!
//! Implement `Solution::is_palindrome(head: Option<Box<ListNode>>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 2, 2, 1]
//! Output: true
//!
//! Input:  head = [1, 2]
//! Output: false
//!
//! Input:  head = [3]
//! Output: true
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 10^5`, `0 <= val <= 9`
//!
//! ## Hints
//!
//! 1. A singly linked list cannot be walked backwards, but a `Vec` can. Copy the values out first.
//! 2. Compare the first half of the `Vec` with the second half read in reverse.
//! 3. Follow-up (`O(1)` extra space): find the middle, reverse the second half in place
//!    (problem 0630), and compare the two halves node by node.

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::list;

    #[test]
    fn even_palindrome() {
        assert!(Solution::is_palindrome(list(&[1, 2, 2, 1])));
    }

    #[test]
    fn not_a_palindrome() {
        assert!(!Solution::is_palindrome(list(&[1, 2])));
        assert!(!Solution::is_palindrome(list(&[1, 2, 3, 1])));
    }

    #[test]
    fn odd_lengths() {
        assert!(Solution::is_palindrome(list(&[3])));
        assert!(Solution::is_palindrome(list(&[1, 2, 3, 2, 1])));
        assert!(!Solution::is_palindrome(list(&[1, 2, 3, 3, 1])));
    }

    #[test]
    fn long_palindrome_with_one_mismatch() {
        let mut values: Vec<i32> = (0..2_500).map(|i| i % 10).collect();
        let mirror: Vec<i32> = values.iter().rev().copied().collect();
        values.extend(mirror);
        assert!(Solution::is_palindrome(list(&values)));
        values[1_234] = (values[1_234] + 1) % 10;
        assert!(!Solution::is_palindrome(list(&values)));
    }
}
