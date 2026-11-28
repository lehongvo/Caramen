// ==================================================================
// PRACTICE 0693: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0693_lowest_common_ancestor_of_a_binary_tree.md
// Run the tests:          cargo test practice::p0693
// ==================================================================

//! # Problem 0693: Lowest Common Ancestor of a Binary Tree
//!
//! - **Difficulty:** Medium
//! - **Topics:** post-order search returning `Option<Rc<..>>`, combining two results with `match`, `Option::or`
//! - **Source:** [LeetCode 236. Lowest Common Ancestor of a Binary Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/)
//!
//! ## Problem
//!
//! Same question as problem 0692, but the tree is an **ordinary** binary tree: the values give no
//! hint about where `p` and `q` are. Return the lowest node that has both `p` and `q` as
//! descendants (a node is a descendant of itself).
//!
//! Implement
//! `Solution::lowest_common_ancestor(root, p, q) -> Option<Rc<RefCell<TreeNode>>>`, where all three
//! parameters are `Option<Rc<RefCell<TreeNode>>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [3, 5, 1, 6, 2, 0, 8, null, null, 7, 4], p = 5, q = 1
//! Output: 3
//!
//! Input:  root = [3, 5, 1, 6, 2, 0, 8, null, null, 7, 4], p = 5, q = 4
//! Output: 5
//!
//! Input:  root = [1, 2], p = 1, q = 2
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `2 <= n <= 10^5`, `-10^9 <= val <= 10^9`, all values distinct.
//! - `p != q`, and both are nodes of the tree.
//!
//! ## Hints
//!
//! 1. Write `find(node) -> Option<Rc<..>>` that returns: `None` if the subtree contains neither
//!    target; the node itself if it **is** one of the targets; otherwise the results of its children
//!    combined.
//! 2. If both children report something, the targets are on different sides and the current node
//!    is the LCA. If only one side reports something, pass that result up.
//! 3. Values are distinct, so you can identify `p` and `q` by value.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{find_node, parse_tree};

    const TREE: &str = "[3,5,1,6,2,0,8,null,null,7,4]";

    fn lca(text: &str, p: i32, q: i32) -> i32 {
        let root = parse_tree(text);
        let (p, q) = (find_node(&root, p), find_node(&root, q));
        let answer = Solution::lowest_common_ancestor(root.clone(), p, q).unwrap();
        let val = answer.borrow().val;
        assert!(Rc::ptr_eq(&answer, &find_node(&root, val).unwrap()));
        val
    }

    #[test]
    fn targets_on_both_sides() {
        assert_eq!(lca(TREE, 5, 1), 3);
    }

    #[test]
    fn target_is_an_ancestor_of_the_other() {
        assert_eq!(lca(TREE, 5, 4), 5);
        assert_eq!(lca("[1,2]", 1, 2), 1);
    }

    #[test]
    fn deep_targets() {
        assert_eq!(lca(TREE, 7, 4), 2);
        assert_eq!(lca(TREE, 6, 4), 5);
        assert_eq!(lca(TREE, 7, 8), 3);
        assert_eq!(lca(TREE, 0, 8), 1);
    }
}
