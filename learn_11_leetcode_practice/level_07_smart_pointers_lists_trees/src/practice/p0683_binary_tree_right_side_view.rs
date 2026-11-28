// ==================================================================
// PRACTICE 0683: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0683_binary_tree_right_side_view.md
// Run the tests:          cargo test practice::p0683
// ==================================================================

//! # Problem 0683: Binary Tree Right Side View
//!
//! - **Difficulty:** Medium
//! - **Topics:** level-by-level BFS, `VecDeque` with a per-level count, `let ... else`, `Extend` from `Option`
//! - **Source:** [LeetCode 199. Binary Tree Right Side View](https://leetcode.com/problems/binary-tree-right-side-view/)
//!
//! ## Problem
//!
//! Imagine standing on the right side of a binary tree. Return the values you can see, from top to
//! bottom: for each depth, the **rightmost** node at that depth (it may be in the left subtree if
//! the right subtree is shorter).
//!
//! Implement `Solution::right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [1, 2, 3, null, 5, null, 4]
//! Output: [1, 3, 4]
//!
//! Input:  root = [1, 2, 3, 4, null, null, null, 5]
//! Output: [1, 3, 4, 5]   // 4 and 5 are seen through the gap on the right
//!
//! Input:  root = []
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 100`, `-100 <= val <= 100`
//!
//! ## Hints
//!
//! 1. Traverse level by level (problem 0682); the last node of each level is the visible one.
//! 2. With a `VecDeque`, record `queue.len()` at the start of a level: exactly that many nodes
//!    belong to it.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_tree;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::right_side_view(parse_tree("[1,2,3,null,5,null,4]")),
            vec![1, 3, 4]
        );
    }

    #[test]
    fn deeper_left_subtree_is_visible() {
        assert_eq!(
            Solution::right_side_view(parse_tree("[1,2,3,4,null,null,null,5]")),
            vec![1, 3, 4, 5]
        );
    }

    #[test]
    fn empty_and_chains() {
        assert!(Solution::right_side_view(None).is_empty());
        assert_eq!(
            Solution::right_side_view(parse_tree("[1,null,3]")),
            vec![1, 3]
        );
        assert_eq!(
            Solution::right_side_view(parse_tree("[1,2,null,3]")),
            vec![1, 2, 3]
        );
    }
}
