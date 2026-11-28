//! # Problem 0697: Convert Sorted List to Binary Search Tree
//!
//! - **Difficulty:** Medium
//! - **Topics:** from `Option<Box<ListNode>>` to `Option<Rc<RefCell<TreeNode>>>`, slices, divide and conquer
//! - **Source:** [LeetCode 109. Convert Sorted List to Binary Search Tree](https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/)
//!
//! ## Problem
//!
//! A singly linked list is sorted in ascending order. Build a **height-balanced** binary search
//! tree with the same values (for every node, the heights of the two subtrees differ by at most
//! one). Any such tree is accepted.
//!
//! Implement
//! `Solution::sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  head = [-10, -3, 0, 5, 9]
//! Output: [0, -3, 9, -10, null, 5]
//!
//! Input:  head = []
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 2 * 10^4`, `-10^5 <= val <= 10^5`
//!
//! ## Hints
//!
//! 1. Lists have no random access, but you need the middle element again and again. Copy the
//!    values into a `Vec` first.
//! 2. Then solve problem 0680: the middle value becomes the root, the halves become the subtrees.
//! 3. Follow-up: an `O(1)` extra space version builds the tree in in-order while consuming the list
//!    from the front.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** convert the list into a `Vec<i32>` (one pass), then build the tree recursively from the
//! middle of each slice. Both halves differ in size by at most one, so the result is balanced.
//!
//! **Complexity:** `O(n)` time, `O(n)` extra space for the values (`O(log n)` stack).
//!
//! **Rust concepts:**
//! - The input is consumed: `while let Some(node) = current { current = node.next; }` moves each box
//!   and frees it after its value is copied.
//! - Two different smart pointers meet here: the list owns its nodes with `Box`, the tree shares
//!   its nodes with `Rc<RefCell<..>>`.
// @solution-end

use crate::common::{ListNode, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    // @solution-start remove
    fn build(values: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if values.is_empty() {
            return None;
        }
        let (left, rest) = values.split_at(values.len() / 2);
        Some(Rc::new(RefCell::new(TreeNode {
            val: rest[0],
            left: Self::build(left),
            right: Self::build(&rest[1..]),
        })))
    }
    // @solution-end

    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        // @solution-start
        let mut values = Vec::new();
        let mut current = head;
        while let Some(node) = current {
            values.push(node.val);
            current = node.next;
        }
        Self::build(&values)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::list;

    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            inorder(&node.left, out);
            out.push(node.val);
            inorder(&node.right, out);
        }
    }

    fn balanced_height(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<usize> {
        let Some(node) = node else {
            return Some(0);
        };
        let node = node.borrow();
        let (l, r) = (balanced_height(&node.left)?, balanced_height(&node.right)?);
        (l.abs_diff(r) <= 1).then_some(1 + l.max(r))
    }

    fn assert_valid(values: &[i32]) {
        let root = Solution::sorted_list_to_bst(list(values));
        let mut out = Vec::new();
        inorder(&root, &mut out);
        assert_eq!(out, values, "in-order traversal must give the list back");
        assert!(
            balanced_height(&root).is_some(),
            "tree must be height-balanced"
        );
    }

    #[test]
    fn example_one() {
        assert_valid(&[-10, -3, 0, 5, 9]);
    }

    #[test]
    fn empty_list() {
        assert_eq!(Solution::sorted_list_to_bst(None), None);
    }

    #[test]
    fn small_lists() {
        assert_valid(&[1]);
        assert_valid(&[1, 3]);
        assert_valid(&[1, 2, 3, 4]);
    }

    #[test]
    fn long_list() {
        let values: Vec<i32> = (-2_500..2_500).collect();
        assert_valid(&values);
    }
}
