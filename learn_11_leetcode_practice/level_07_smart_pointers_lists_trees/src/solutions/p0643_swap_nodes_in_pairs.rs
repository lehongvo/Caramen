//! # Problem 0643: Swap Nodes in Pairs
//!
//! - **Difficulty:** Medium
//! - **Topics:** detaching two nodes at a time, re-linking owned boxes, tail cursor
//! - **Source:** [LeetCode 24. Swap Nodes in Pairs](https://leetcode.com/problems/swap-nodes-in-pairs/)
//!
//! ## Problem
//!
//! Swap every two adjacent nodes of a linked list and return the new head: the 1st and 2nd nodes
//! change places, then the 3rd and 4th, and so on. A final node without a partner stays where it
//! is. Move the **nodes** themselves; do not just swap the values.
//!
//! Implement `Solution::swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 2, 3, 4]
//! Output: [2, 1, 4, 3]
//!
//! Input:  head = [1, 2, 3]
//! Output: [2, 1, 3]
//!
//! Input:  head = []
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 100`, `0 <= val <= 100`
//!
//! ## Hints
//!
//! 1. Take the nodes off the input two at a time: `first`, then `second = first.next.take()`.
//!    The rest of the list is `second.next.take()`.
//! 2. Append `second`, then `first`, to the result with a tail cursor.
//! 3. If `second` does not exist, append `first` alone and stop.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** treat the input as a queue of nodes. Each round detaches a pair, completely unlinks
//! both nodes, and appends them in swapped order. Every node is moved exactly once.
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - After `take()`, each node's `next` is `None`, so appending it never drags along nodes you did
//!   not intend to move.
//! - The `while let Some(mut first) = rest` loop moves `rest`; every path that continues assigns it
//!   again, which the compiler verifies.
// @solution-end

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // @solution-start
        let mut result = None;
        let mut tail = &mut result;
        let mut rest = head;
        while let Some(mut first) = rest {
            match first.next.take() {
                Some(mut second) => {
                    rest = second.next.take();
                    tail = &mut tail.insert(second).next;
                    tail = &mut tail.insert(first).next;
                }
                None => {
                    *tail = Some(first);
                    break;
                }
            }
        }
        result
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn even_length() {
        let result = Solution::swap_pairs(list(&[1, 2, 3, 4]));
        assert_eq!(list_to_vec(&result), vec![2, 1, 4, 3]);
    }

    #[test]
    fn odd_length_keeps_last_node() {
        let result = Solution::swap_pairs(list(&[1, 2, 3]));
        assert_eq!(list_to_vec(&result), vec![2, 1, 3]);
    }

    #[test]
    fn empty_and_single() {
        assert_eq!(Solution::swap_pairs(None), None);
        assert_eq!(Solution::swap_pairs(list(&[1])), list(&[1]));
    }

    #[test]
    fn hundred_nodes() {
        let values: Vec<i32> = (0..100).collect();
        let result = list_to_vec(&Solution::swap_pairs(list(&values)));
        let expected: Vec<i32> = (0..100).map(|i| i ^ 1).collect();
        assert_eq!(result, expected);
    }
}
