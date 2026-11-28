// ==================================================================
// PRACTICE 0673: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0673_balanced_binary_tree.md
// Run the tests:          cargo test practice::p0673
// ==================================================================

//! # Problem 0673: Balanced Binary Tree
//!
//! - **Difficulty:** Easy
//! - **Topics:** post-order recursion returning `Option<i32>`, early exit with `?`
//! - **Source:** [LeetCode 110. Balanced Binary Tree](https://leetcode.com/problems/balanced-binary-tree/)
//!
//! ## Problem
//!
//! A binary tree is *height-balanced* if, for **every** node, the heights of its left and right
//! subtrees differ by at most one. Return whether the given tree is height-balanced. The empty tree
//! is balanced.
//!
//! Implement `Solution::is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [3, 9, 20, null, null, 15, 7]
//! Output: true
//!
//! Input:  root = [1, 2, 2, 3, 3, null, null, 4, 4]
//! Output: false          // the root's subtrees have heights 3 and 1
//!
//! Input:  root = []
//! Output: true
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 5000`, `-10^4 <= val <= 10^4`
//!
//! ## Hints
//!
//! 1. Computing the height separately for every node costs `O(n^2)` on a long chain. Compute
//!    heights bottom-up once instead.
//! 2. Let a helper return `Some(height)` for a balanced subtree and `None` as soon as an unbalanced
//!    node is found; `?` then stops the whole recursion early.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{parse_tree, tree};

    #[test]
    fn balanced_example() {
        assert!(Solution::is_balanced(parse_tree("[3,9,20,null,null,15,7]")));
    }

    #[test]
    fn unbalanced_example() {
        assert!(!Solution::is_balanced(parse_tree(
            "[1,2,2,3,3,null,null,4,4]"
        )));
    }

    #[test]
    fn empty_and_small_trees() {
        assert!(Solution::is_balanced(None));
        assert!(Solution::is_balanced(parse_tree("[1,2]")));
        assert!(!Solution::is_balanced(parse_tree("[1,2,null,3]")));
    }

    #[test]
    fn imbalance_deep_inside() {
        // The root's subtrees both have height 3, but node 2's subtrees have heights 2 and 0.
        assert!(!Solution::is_balanced(parse_tree(
            "[1,2,3,4,null,5,6,7,null,null,null,8]"
        )));
        let perfect: Vec<Option<i32>> = (1..=2047).map(Some).collect();
        assert!(Solution::is_balanced(tree(&perfect)));
    }
}
