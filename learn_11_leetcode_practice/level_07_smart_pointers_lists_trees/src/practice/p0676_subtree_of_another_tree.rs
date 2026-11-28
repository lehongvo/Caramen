// ==================================================================
// PRACTICE 0676: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0676_subtree_of_another_tree.md
// Run the tests:          cargo test practice::p0676
// ==================================================================

//! # Problem 0676: Subtree of Another Tree
//!
//! - **Difficulty:** Easy
//! - **Topics:** tree equality through derived `PartialEq` on `Rc<RefCell<TreeNode>>`, DFS with a stack
//! - **Source:** [LeetCode 572. Subtree of Another Tree](https://leetcode.com/problems/subtree-of-another-tree/)
//!
//! ## Problem
//!
//! Return `true` if `sub_root` appears inside `root` as a **complete subtree**: some node of
//! `root`, together with **all** of its descendants, has exactly the same shape and values as
//! `sub_root`. A tree counts as a subtree of itself.
//!
//! Implement
//! `Solution::is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [3, 4, 5, 1, 2], sub_root = [4, 1, 2]
//! Output: true
//!
//! Input:  root = [3, 4, 5, 1, 2, null, null, null, null, 0], sub_root = [4, 1, 2]
//! Output: false          // the node 4 has an extra descendant 0
//! ```
//!
//! ## Constraints
//!
//! - `root` has `1` to `2000` nodes, `sub_root` has `1` to `1000` nodes; `-10^4 <= val <= 10^4`.
//!
//! ## Hints
//!
//! 1. Visit every node of `root` and ask "is the tree starting here the same as `sub_root`?"
//!    (problem 0666).
//! 2. `TreeNode` derives `PartialEq`, and so do `Rc` and `RefCell` (by comparing their contents),
//!    so `*node == *sub` compares two whole subtrees.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_tree;

    #[test]
    fn example_one() {
        assert!(Solution::is_subtree(
            parse_tree("[3,4,5,1,2]"),
            parse_tree("[4,1,2]")
        ));
    }

    #[test]
    fn extra_descendant_breaks_the_match() {
        assert!(!Solution::is_subtree(
            parse_tree("[3,4,5,1,2,null,null,null,null,0]"),
            parse_tree("[4,1,2]")
        ));
    }

    #[test]
    fn whole_tree_and_leaves() {
        assert!(Solution::is_subtree(
            parse_tree("[1,2,3]"),
            parse_tree("[1,2,3]")
        ));
        assert!(Solution::is_subtree(
            parse_tree("[1,2,3]"),
            parse_tree("[3]")
        ));
        assert!(!Solution::is_subtree(
            parse_tree("[1,2,3]"),
            parse_tree("[1,2]")
        ));
    }

    #[test]
    fn repeated_values_need_a_full_match() {
        let root = parse_tree("[1,1,1,1,1,1,1,null,null,null,null,null,null,1,2]");
        assert!(Solution::is_subtree(root.clone(), parse_tree("[1,1,2]")));
        assert!(!Solution::is_subtree(root, parse_tree("[1,2,1]")));
    }
}
