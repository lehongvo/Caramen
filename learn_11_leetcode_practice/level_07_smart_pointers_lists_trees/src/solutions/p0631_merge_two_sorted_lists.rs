//! # Problem 0631: Merge Two Sorted Lists
//!
//! - **Difficulty:** Easy
//! - **Topics:** `match` on a pair of `Option<Box<ListNode>>`, tail cursor with `Option::insert`, moving nodes
//! - **Source:** [LeetCode 21. Merge Two Sorted Lists](https://leetcode.com/problems/merge-two-sorted-lists/)
//!
//! ## Problem
//!
//! You are given the heads of two linked lists, each sorted in non-decreasing order. Splice their
//! nodes together into one sorted list and return its head. Reuse the given nodes; when two values
//! are equal, take the node from `list1` first.
//!
//! Implement
//! `Solution::merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  list1 = [1, 2, 4], list2 = [1, 3, 4]
//! Output: [1, 1, 2, 3, 4, 4]
//!
//! Input:  list1 = [], list2 = []
//! Output: []
//!
//! Input:  list1 = [], list2 = [0]
//! Output: [0]
//! ```
//!
//! ## Constraints
//!
//! - Each list has between `0` and `50` nodes, with `-100 <= val <= 100`.
//! - Both lists are sorted in non-decreasing order.
//!
//! ## Hints
//!
//! 1. Keep a result list and a tail cursor `tail: &mut Option<Box<ListNode>>` (see problem 0627).
//! 2. `match (a, b)` on the two remaining lists: when both are `Some`, detach the smaller head
//!    (`node.next.take()` becomes the new remaining list) and append it at the tail.
//! 3. When one list runs out, the other one can be attached in one move: `*tail = a.or(b);`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** repeatedly move the smaller of the two head nodes to the end of the result. The pair
//! `(a, b)` is moved into the `match`; each arm puts both lists back (one shortened by one node)
//! before the next iteration. When a list is empty, the rest of the other is already sorted and is
//! attached as a whole.
//!
//! **Complexity:** `O(n + m)` time, `O(1)` extra space (nodes are reused).
//!
//! **Rust concepts:**
//! - Matching on a tuple of owned options moves both lists; the compiler checks that `a` and `b`
//!   are assigned again on every path that loops.
//! - `tail = &mut tail.insert(node).next;` appends and advances the cursor in one line.
//! - `Option::or` picks whichever list is still non-empty.
// @solution-end

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // @solution-start
        let (mut a, mut b) = (list1, list2);
        let mut head = None;
        let mut tail = &mut head;
        loop {
            match (a, b) {
                (Some(mut x), Some(mut y)) => {
                    if x.val <= y.val {
                        a = x.next.take();
                        b = Some(y);
                        tail = &mut tail.insert(x).next;
                    } else {
                        b = y.next.take();
                        a = Some(x);
                        tail = &mut tail.insert(y).next;
                    }
                }
                (rest_a, rest_b) => {
                    *tail = rest_a.or(rest_b);
                    return head;
                }
            }
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn example_one() {
        let merged = Solution::merge_two_lists(list(&[1, 2, 4]), list(&[1, 3, 4]));
        assert_eq!(list_to_vec(&merged), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn empty_inputs() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
        assert_eq!(Solution::merge_two_lists(None, list(&[0])), list(&[0]));
        assert_eq!(Solution::merge_two_lists(list(&[5]), None), list(&[5]));
    }

    #[test]
    fn one_list_entirely_before_the_other() {
        let merged = Solution::merge_two_lists(list(&[4, 5, 6]), list(&[1, 2, 3]));
        assert_eq!(list_to_vec(&merged), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn negatives_and_duplicates() {
        let merged = Solution::merge_two_lists(list(&[-100, 0, 0, 100]), list(&[-5, 0, 7]));
        assert_eq!(list_to_vec(&merged), vec![-100, -5, 0, 0, 0, 7, 100]);
    }

    #[test]
    fn long_interleaved_lists() {
        let evens: Vec<i32> = (0..2_000).map(|i| i * 2).collect();
        let odds: Vec<i32> = (0..2_000).map(|i| i * 2 + 1).collect();
        let merged = Solution::merge_two_lists(list(&evens), list(&odds));
        assert_eq!(list_to_vec(&merged), (0..4_000).collect::<Vec<_>>());
    }
}
