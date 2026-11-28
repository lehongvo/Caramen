// ==================================================================
// PRACTICE 0675: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0675_merge_two_binary_trees.md
// Run the tests:          cargo test practice::p0675
// ==================================================================

//! # Problem 0675: Merge Two Binary Trees
//!
//! - **Difficulty:** Easy
//! - **Topics:** or-patterns with bindings (`(None, tree) | (tree, None)`), `borrow_mut` on two nodes, `take()` of children
//! - **Source:** [LeetCode 617. Merge Two Binary Trees](https://leetcode.com/problems/merge-two-binary-trees/)
//!
//! ## Problem
//!
//! Overlay two binary trees. Where both trees have a node at the same position, the merged tree has
//! one node holding the **sum** of the two values. Where only one tree has a node, the merged tree
//! uses that node (with its whole subtree). Return the merged tree. You may reuse and modify the
//! nodes of both inputs.
//!
//! Implement
//! `Solution::merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root1 = [1, 3, 2, 5], root2 = [2, 1, 3, null, 4, null, 7]
//! Output: [3, 4, 5, 5, 4, null, 7]
//!
//! Input:  root1 = [1], root2 = [1, 2]
//! Output: [2, 2]
//! ```
//!
//! ## Constraints
//!
//! - Each tree has `0` to `2000` nodes; `-10^4 <= val <= 10^4`.
//!
//! ## Hints
//!
//! 1. Match on the pair of roots. If one side is `None`, the answer is the other side, unchanged.
//! 2. If both exist, keep the first node: add the second node's value to it, and set each child to
//!    the merge of the corresponding children (taken out of both nodes with `take()`).

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{parse_tree, tree_to_string};

    #[test]
    fn example_one() {
        let merged =
            Solution::merge_trees(parse_tree("[1,3,2,5]"), parse_tree("[2,1,3,null,4,null,7]"));
        assert_eq!(tree_to_string(&merged), "[3,4,5,5,4,null,7]");
    }

    #[test]
    fn example_two() {
        let merged = Solution::merge_trees(parse_tree("[1]"), parse_tree("[1,2]"));
        assert_eq!(tree_to_string(&merged), "[2,2]");
    }

    #[test]
    fn one_or_both_empty() {
        assert_eq!(Solution::merge_trees(None, None), None);
        let merged = Solution::merge_trees(None, parse_tree("[4,5]"));
        assert_eq!(tree_to_string(&merged), "[4,5]");
        let merged = Solution::merge_trees(parse_tree("[4,null,5]"), None);
        assert_eq!(tree_to_string(&merged), "[4,null,5]");
    }

    #[test]
    fn disjoint_shapes() {
        let merged =
            Solution::merge_trees(parse_tree("[1,2,null,3]"), parse_tree("[1,null,2,null,3]"));
        assert_eq!(tree_to_string(&merged), "[2,2,2,3,null,null,3]");
    }
}
