// ==================================================================
// PRACTICE 0668: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0668_symmetric_tree.md
// Run the tests:          cargo test practice::p0668
// ==================================================================

//! # Problem 0668: Symmetric Tree
//!
//! - **Difficulty:** Easy
//! - **Topics:** mirror recursion over two subtrees, borrowing two nodes at once
//! - **Source:** [LeetCode 101. Symmetric Tree](https://leetcode.com/problems/symmetric-tree/)
//!
//! ## Problem
//!
//! A binary tree is symmetric if it is a mirror image of itself around its center: the left
//! subtree is the mirror of the right subtree. Return whether the tree is symmetric.
//!
//! Implement `Solution::is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [1, 2, 2, 3, 4, 4, 3]
//! Output: true
//!
//! Input:  root = [1, 2, 2, null, 3, null, 3]
//! Output: false
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 1000`, `-100 <= val <= 100`
//!
//! ## Hints
//!
//! 1. Two trees `a` and `b` are mirrors when their roots have equal values, `a.left` mirrors
//!    `b.right`, and `a.right` mirrors `b.left`.
//! 2. The whole tree is symmetric when `root.left` and `root.right` are mirrors.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_tree;

    #[test]
    fn symmetric_example() {
        assert!(Solution::is_symmetric(parse_tree("[1,2,2,3,4,4,3]")));
    }

    #[test]
    fn asymmetric_shape() {
        assert!(!Solution::is_symmetric(parse_tree("[1,2,2,null,3,null,3]")));
    }

    #[test]
    fn asymmetric_values() {
        assert!(!Solution::is_symmetric(parse_tree("[1,2,3]")));
        assert!(!Solution::is_symmetric(parse_tree("[1,2,2,2,null,2]")));
    }

    #[test]
    fn single_node_and_deeper_symmetry() {
        assert!(Solution::is_symmetric(parse_tree("[7]")));
        assert!(Solution::is_symmetric(parse_tree(
            "[1,2,2,3,null,null,3,4,null,null,4]"
        )));
    }
}
