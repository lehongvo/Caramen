// ==================================================================
// PRACTICE 0662: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0662_root_equals_sum_of_children.md
// Run the tests:          cargo test practice::p0662
// ==================================================================

//! # Problem 0662: Root Equals Sum of Children
//!
//! - **Difficulty:** Easy
//! - **Topics:** `let ... else` on `Option<Rc<RefCell<TreeNode>>>`, `borrow()`, matching a pair of children
//! - **Source:** [LeetCode 2236. Root Equals Sum of Children](https://leetcode.com/problems/root-equals-sum-of-children/)
//!
//! ## Problem
//!
//! A binary tree has exactly three nodes: a root, its left child and its right child. Return
//! `true` if the root's value equals the sum of its two children's values, and `false` otherwise.
//!
//! Implement `Solution::check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [10, 4, 6]
//! Output: true           // 4 + 6 = 10
//!
//! Input:  root = [5, 3, 1]
//! Output: false          // 3 + 1 = 4
//! ```
//!
//! ## Constraints
//!
//! - The tree has exactly `3` nodes; `-100 <= val <= 100`.
//!
//! ## Hints
//!
//! 1. `let Some(root) = root else { return false };` gives the `Rc`; `root.borrow()` gives a
//!    readable `TreeNode`.
//! 2. Match on `(&node.left, &node.right)` and borrow each child to read its `val`.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_tree;

    #[test]
    fn sum_matches() {
        assert!(Solution::check_tree(parse_tree("[10,4,6]")));
    }

    #[test]
    fn sum_does_not_match() {
        assert!(!Solution::check_tree(parse_tree("[5,3,1]")));
    }

    #[test]
    fn negative_and_zero_values() {
        assert!(Solution::check_tree(parse_tree("[-100,-50,-50]")));
        assert!(Solution::check_tree(parse_tree("[0,7,-7]")));
        assert!(!Solution::check_tree(parse_tree("[1,0,0]")));
    }
}
