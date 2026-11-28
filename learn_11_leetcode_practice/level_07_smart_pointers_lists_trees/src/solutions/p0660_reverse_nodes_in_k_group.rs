//! # Problem 0660: Reverse Nodes in k-Group
//!
//! - **Difficulty:** Hard
//! - **Topics:** detaching groups of nodes, reversing by prepending, splicing groups with a tail cursor
//! - **Source:** [LeetCode 25. Reverse Nodes in k-Group](https://leetcode.com/problems/reverse-nodes-in-k-group/)
//!
//! ## Problem
//!
//! Reverse the nodes of a linked list `k` at a time: the first `k` nodes are reversed, then the
//! next `k`, and so on. If fewer than `k` nodes remain at the end, they keep their order. Move the
//! nodes; do not change their values. Return the new head.
//!
//! Implement `Solution::reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 2, 3, 4, 5], k = 2
//! Output: [2, 1, 4, 3, 5]
//!
//! Input:  head = [1, 2, 3, 4, 5], k = 3
//! Output: [3, 2, 1, 4, 5]
//!
//! Input:  head = [1, 2, 3], k = 1
//! Output: [1, 2, 3]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= k <= n <= 5000`, `0 <= val <= 1000`
//!
//! ## Hints
//!
//! 1. Count the nodes first: you then know how many full groups there are (`n / k`).
//! 2. For each full group, move `k` nodes from the input onto the front of a `group` list; that
//!    reverses them (problem 0630).
//! 3. Attach the group at the result's tail cursor, then walk the cursor to the end of the group.
//!    After the last full group, attach whatever is left of the input unchanged.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** knowing `n` up front removes the need to "look ahead" `k` nodes before reversing. Each
//! full group is detached and reversed in one pass, appended to the result, and the tail cursor is
//! moved over it. The remaining `n % k` nodes are attached as a whole.
//!
//! **Complexity:** `O(n)` time (every node is moved once and walked over once), `O(1)` extra space.
//!
//! **Rust concepts:**
//! - The tail cursor is a `&mut Option<Box<ListNode>>`; after `*tail = group;`, the loop
//!   `while let Some(node) = tail { tail = &mut node.next; }` leaves it on the empty slot after the
//!   group.
//! - `rest.take()` inside `if let` keeps `rest` initialized on every path.
// @solution-end

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // @solution-start
        let mut remaining =
            std::iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        let k = k as usize;
        let mut result = None;
        let mut tail = &mut result;
        let mut rest = head;
        while remaining >= k {
            let mut group = None;
            for _ in 0..k {
                if let Some(mut node) = rest.take() {
                    rest = node.next.take();
                    node.next = group;
                    group = Some(node);
                }
            }
            *tail = group;
            while let Some(node) = tail {
                tail = &mut node.next;
            }
            remaining -= k;
        }
        *tail = rest;
        result
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn groups_of_two() {
        let result = Solution::reverse_k_group(list(&[1, 2, 3, 4, 5]), 2);
        assert_eq!(list_to_vec(&result), vec![2, 1, 4, 3, 5]);
    }

    #[test]
    fn groups_of_three() {
        let result = Solution::reverse_k_group(list(&[1, 2, 3, 4, 5]), 3);
        assert_eq!(list_to_vec(&result), vec![3, 2, 1, 4, 5]);
    }

    #[test]
    fn k_of_one_and_k_of_n() {
        assert_eq!(
            Solution::reverse_k_group(list(&[1, 2, 3]), 1),
            list(&[1, 2, 3])
        );
        assert_eq!(
            Solution::reverse_k_group(list(&[1, 2, 3]), 3),
            list(&[3, 2, 1])
        );
        assert_eq!(Solution::reverse_k_group(list(&[7]), 1), list(&[7]));
    }

    #[test]
    fn long_list_with_a_partial_group() {
        let values: Vec<i32> = (0..5_000).collect();
        let result = list_to_vec(&Solution::reverse_k_group(list(&values), 7));
        let mut expected = Vec::new();
        for chunk in values.chunks(7) {
            if chunk.len() == 7 {
                expected.extend(chunk.iter().rev());
            } else {
                expected.extend(chunk);
            }
        }
        assert_eq!(result, expected);
    }
}
