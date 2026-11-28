//! # Problem 0640: Remove Nth Node From End of List
//!
//! - **Difficulty:** Medium
//! - **Topics:** dummy head node, length pass, `&mut Box<ListNode>` cursor, unlinking
//! - **Source:** [LeetCode 19. Remove Nth Node From End of List](https://leetcode.com/problems/remove-nth-node-from-end-of-list/)
//!
//! ## Problem
//!
//! Remove the `n`-th node **counted from the end** of a linked list (`n = 1` is the last node) and
//! return the head of the list.
//!
//! Implement `Solution::remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [1, 2, 3, 4, 5], n = 2
//! Output: [1, 2, 3, 5]
//!
//! Input:  head = [1], n = 1
//! Output: []
//!
//! Input:  head = [1, 2], n = 2
//! Output: [2]            // the head itself is removed
//! ```
//!
//! ## Constraints
//!
//! - `1 <= size <= 30`, `0 <= val <= 100`, `1 <= n <= size`
//!
//! ## Hints
//!
//! 1. The `n`-th node from the end is the node at index `size - n` from the front.
//! 2. Removing the head is a special case... unless you put a **dummy** node in front of the list.
//!    Then every real node has a predecessor, including the head.
//! 3. Walk `size - n` steps from the dummy, unlink the next node, and return `dummy.next`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** prepend a dummy node owning the list, so "the node before the target" always exists.
//! Count the nodes, walk to the predecessor of index `size - n`, and splice the target out.
//!
//! **Complexity:** `O(size)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - A dummy `Box<ListNode>` turns the "remove the head" special case into the general case.
//! - `cursor = cursor.next.as_mut()?;` advances a `&mut Box<ListNode>`; the `?` can never fire
//!   here because `size - n < size`, but it keeps the code free of `unwrap`.
//! - The one-pass "two pointers `n` apart" trick from other languages needs two cursors into the
//!   same list, one of them mutable, which the borrow checker does not allow without extra work.
// @solution-end

use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // @solution-start
        let size = std::iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        let steps = size - n as usize;
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut cursor = &mut dummy;
        for _ in 0..steps {
            cursor = cursor.next.as_mut()?;
        }
        cursor.next = cursor.next.take().and_then(|target| target.next);
        dummy.next
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{list, list_to_vec};

    #[test]
    fn example_one() {
        let result = Solution::remove_nth_from_end(list(&[1, 2, 3, 4, 5]), 2);
        assert_eq!(list_to_vec(&result), vec![1, 2, 3, 5]);
    }

    #[test]
    fn single_node() {
        assert_eq!(Solution::remove_nth_from_end(list(&[1]), 1), None);
    }

    #[test]
    fn remove_head_and_tail() {
        assert_eq!(Solution::remove_nth_from_end(list(&[1, 2]), 2), list(&[2]));
        assert_eq!(Solution::remove_nth_from_end(list(&[1, 2]), 1), list(&[1]));
    }

    #[test]
    fn every_position_of_a_thirty_node_list() {
        let values: Vec<i32> = (0..30).collect();
        for n in 1..=30 {
            let result = list_to_vec(&Solution::remove_nth_from_end(list(&values), n));
            let mut expected = values.clone();
            expected.remove(30 - n as usize);
            assert_eq!(result, expected, "n = {n}");
        }
    }
}
