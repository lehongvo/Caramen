//! # Problem 0647: Remove Duplicates from Sorted List II
//!
//! - **Difficulty:** Medium
//! - **Topics:** skipping runs of nodes, `Option::is_some_and`, rebuilding with a tail cursor
//! - **Source:** [LeetCode 82. Remove Duplicates from Sorted List II](https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/)
//!
//! ## Problem
//!
//! A linked list is sorted in non-decreasing order. Delete **all** nodes whose value appears more
//! than once, so only values that were unique in the original list remain. Return the sorted
//! result. (Compare with problem 0632, which keeps one copy of each value.)
//!
//! Implement `Solution::delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 2, 3, 3, 4, 4, 5]
//! Output: [1, 2, 5]
//!
//! Input:  head = [1, 1, 1, 2, 3]
//! Output: [2, 3]
//!
//! Input:  head = [1, 1]
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 300`, `-100 <= val <= 100`
//! - The list is sorted in non-decreasing order.
//!
//! ## Hints
//!
//! 1. Detach the next node. If the following node has the same value, you are at the start of a
//!    run: drop nodes until the value changes, and keep none of them.
//! 2. Otherwise the node is unique: append it to the result.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** read the input as a queue of nodes. A node is kept only if the next node has a
//! different value; when a run starts, the whole run is discarded with
//! `rest = rest.and_then(|dup| dup.next)` until the value changes.
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - `rest.and_then(|dup| dup.next)` consumes `rest`, drops the duplicate node and puts the
//!   remaining list back, all in one expression.
//! - Because the result is a separate list with its own tail cursor, no node ever needs to be
//!   unlinked from the middle of a list, which keeps the borrow checker happy.
// @solution-end

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // @solution-start
        let mut result = None;
        let mut tail = &mut result;
        let mut rest = head;
        while let Some(mut node) = rest {
            rest = node.next.take();
            let value = node.val;
            if rest.as_ref().is_some_and(|next| next.val == value) {
                while rest.as_ref().is_some_and(|next| next.val == value) {
                    rest = rest.and_then(|duplicate| duplicate.next);
                }
            } else {
                tail = &mut tail.insert(node).next;
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
    fn example_one() {
        let result = Solution::delete_duplicates(list(&[1, 2, 3, 3, 4, 4, 5]));
        assert_eq!(list_to_vec(&result), vec![1, 2, 5]);
    }

    #[test]
    fn duplicated_head() {
        let result = Solution::delete_duplicates(list(&[1, 1, 1, 2, 3]));
        assert_eq!(list_to_vec(&result), vec![2, 3]);
    }

    #[test]
    fn everything_removed_or_nothing_removed() {
        assert_eq!(Solution::delete_duplicates(list(&[1, 1])), None);
        assert_eq!(Solution::delete_duplicates(None), None);
        assert_eq!(
            Solution::delete_duplicates(list(&[-1, 0, 1])),
            list(&[-1, 0, 1])
        );
    }

    #[test]
    fn duplicated_tail_and_long_runs() {
        let mut values = vec![-100];
        values.extend([0; 150]);
        values.extend([7, 8, 8]);
        let result = Solution::delete_duplicates(list(&values));
        assert_eq!(list_to_vec(&result), vec![-100, 7]);
    }
}
