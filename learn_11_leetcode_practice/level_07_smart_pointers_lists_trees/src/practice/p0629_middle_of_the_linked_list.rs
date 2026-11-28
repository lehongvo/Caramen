// ==================================================================
// PRACTICE 0629: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0629_middle_of_the_linked_list.md
// Run the tests:          cargo test practice::p0629
// ==================================================================

//! # Problem 0629: Middle of the Linked List
//!
//! - **Difficulty:** Easy
//! - **Topics:** counting pass with `std::iter::successors`, owning cursor, moving out of `Box` fields, why slow/fast pointers do not fit owned lists
//! - **Source:** [LeetCode 876. Middle of the Linked List](https://leetcode.com/problems/middle-of-the-linked-list/)
//!
//! ## Problem
//!
//! Given a non-empty linked list, return the list that starts at its **middle node**. With `n`
//! nodes, the middle is the node at index `n / 2` (0-based), so when there are two middle nodes,
//! return the second one.
//!
//! Implement `Solution::middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.
//! The returned value is the original nodes from the middle to the end (the nodes before it are
//! simply dropped).
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 2, 3, 4, 5]
//! Output: [3, 4, 5]
//!
//! Input:  head = [1, 2, 3, 4, 5, 6]
//! Output: [4, 5, 6]        // two middles (3 and 4): return the second
//!
//! Input:  head = [7]
//! Output: [7]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 100`, `1 <= val <= 100`
//!
//! ## Hints
//!
//! 1. Count the nodes first with a shared cursor (`as_deref`).
//! 2. Then take ownership: `let mut current = head;` and repeat `n / 2` times
//!    `current = current?.next;`. Moving `next` out of the box drops the node you leave behind.
//! 3. The classic slow/fast pointer trick works with shared references, but returning an owned
//!    list from it requires a second pass anyway.

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn odd_length() {
        let result = Solution::middle_node(list(&[1, 2, 3, 4, 5]));
        assert_eq!(list_to_vec(&result), vec![3, 4, 5]);
    }

    #[test]
    fn even_length_returns_second_middle() {
        let result = Solution::middle_node(list(&[1, 2, 3, 4, 5, 6]));
        assert_eq!(list_to_vec(&result), vec![4, 5, 6]);
    }

    #[test]
    fn one_and_two_nodes() {
        assert_eq!(list_to_vec(&Solution::middle_node(list(&[7]))), vec![7]);
        assert_eq!(list_to_vec(&Solution::middle_node(list(&[1, 2]))), vec![2]);
    }

    #[test]
    fn hundred_nodes() {
        let values: Vec<i32> = (1..=100).collect();
        let result = Solution::middle_node(list(&values));
        assert_eq!(list_to_vec(&result), (51..=100).collect::<Vec<_>>());
    }
}
