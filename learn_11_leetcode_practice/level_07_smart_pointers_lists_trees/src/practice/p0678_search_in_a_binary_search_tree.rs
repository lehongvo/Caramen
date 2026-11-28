// ==================================================================
// PRACTICE 0678: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0678_search_in_a_binary_search_tree.md
// Run the tests:          cargo test practice::p0678
// ==================================================================

//! # Problem 0678: Search in a Binary Search Tree
//!
//! - **Difficulty:** Easy
//! - **Topics:** BST walk with an owned `Option<Rc<..>>` cursor, `Ordering`, returning a shared subtree handle
//! - **Source:** [LeetCode 700. Search in a Binary Search Tree](https://leetcode.com/problems/search-in-a-binary-search-tree/)
//!
//! ## Problem
//!
//! In a binary search tree (BST), every value in a node's left subtree is smaller than the node's
//! value, and every value in its right subtree is larger. Find the node whose value is `val` and
//! return the subtree rooted at it (the node itself, shared with the input tree), or `None` if no
//! node has that value.
//!
//! Implement
//! `Solution::search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [4, 2, 7, 1, 3], val = 2
//! Output: [2, 1, 3]
//!
//! Input:  root = [4, 2, 7, 1, 3], val = 5
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 5000`, `1 <= node.val <= 10^7`, all values distinct, `1 <= val <= 10^7`
//! - `root` is a valid BST.
//!
//! ## Hints
//!
//! 1. Compare `val` with the current node: equal means found; smaller means go left; larger means
//!    go right. Only one path is explored.
//! 2. Keep the cursor as an owned `Option<Rc<RefCell<TreeNode>>>`; moving to a child is
//!    `node.borrow().left.clone()`.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{find_node, parse_tree, tree_to_string};

    #[test]
    fn found_inner_node() {
        let result = Solution::search_bst(parse_tree("[4,2,7,1,3]"), 2);
        assert_eq!(tree_to_string(&result), "[2,1,3]");
    }

    #[test]
    fn missing_value() {
        assert_eq!(Solution::search_bst(parse_tree("[4,2,7,1,3]"), 5), None);
    }

    #[test]
    fn root_and_leaves() {
        assert_eq!(
            tree_to_string(&Solution::search_bst(parse_tree("[4,2,7,1,3]"), 4)),
            "[4,2,7,1,3]"
        );
        assert_eq!(
            tree_to_string(&Solution::search_bst(parse_tree("[4,2,7,1,3]"), 3)),
            "[3]"
        );
        assert_eq!(
            tree_to_string(&Solution::search_bst(parse_tree("[4,2,7,1,3]"), 7)),
            "[7]"
        );
    }

    #[test]
    fn result_is_shared_with_the_tree() {
        let root = parse_tree("[8,4,12,2,6,10,14]");
        let found = Solution::search_bst(root.clone(), 10).unwrap();
        assert!(Rc::ptr_eq(&found, &find_node(&root, 10).unwrap()));
        found.borrow_mut().val = 11;
        assert_eq!(tree_to_string(&root), "[8,4,12,2,6,11,14]");
    }
}
