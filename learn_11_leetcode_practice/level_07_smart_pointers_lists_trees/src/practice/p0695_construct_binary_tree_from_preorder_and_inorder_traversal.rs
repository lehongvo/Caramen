// ==================================================================
// PRACTICE 0695: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0695_construct_binary_tree_from_preorder_and_inorder_traversal.md
// Run the tests:          cargo test practice::p0695
// ==================================================================

//! # Problem 0695: Construct Binary Tree from Preorder and Inorder Traversal
//!
//! - **Difficulty:** Medium
//! - **Topics:** `HashMap` of positions, recursion over index ranges, a shared `&mut usize` cursor
//! - **Source:** [LeetCode 105. Construct Binary Tree from Preorder and Inorder Traversal](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/)
//!
//! ## Problem
//!
//! You are given the pre-order and the in-order traversal of the same binary tree, whose values
//! are all distinct. Rebuild the tree and return its root.
//!
//! Implement
//! `Solution::build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  preorder = [3, 9, 20, 15, 7], inorder = [9, 3, 15, 20, 7]
//! Output: [3, 9, 20, null, null, 15, 7]
//!
//! Input:  preorder = [-1], inorder = [-1]
//! Output: [-1]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 3000`, `-3000 <= val <= 3000`, all values distinct.
//! - Both arrays are traversals of the same tree.
//!
//! ## Hints
//!
//! 1. The first value of the pre-order is the root. In the in-order, everything left of the root's
//!    position belongs to the left subtree, everything right of it to the right subtree.
//! 2. Pre-order lists the root, then the whole left subtree, then the whole right subtree. So if
//!    you build the left subtree first, a single index walking through `preorder` always points at
//!    the next root.
//! 3. A `HashMap<i32, usize>` from value to in-order position avoids searching for the root.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{parse_tree, tree_to_string};

    fn traversals(node: &Option<Rc<RefCell<TreeNode>>>, pre: &mut Vec<i32>, ino: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            pre.push(node.val);
            traversals(&node.left, pre, ino);
            ino.push(node.val);
            traversals(&node.right, pre, ino);
        }
    }

    fn round_trip(text: &str) {
        let original = parse_tree(text);
        let (mut pre, mut ino) = (Vec::new(), Vec::new());
        traversals(&original, &mut pre, &mut ino);
        assert_eq!(Solution::build_tree(pre, ino), original, "tree {text}");
    }

    #[test]
    fn example_one() {
        let root = Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
        assert_eq!(tree_to_string(&root), "[3,9,20,null,null,15,7]");
    }

    #[test]
    fn single_node() {
        assert_eq!(
            tree_to_string(&Solution::build_tree(vec![-1], vec![-1])),
            "[-1]"
        );
    }

    #[test]
    fn various_shapes_round_trip() {
        round_trip("[1,2,3,4,5,6,7]");
        round_trip("[1,null,2,null,3]");
        round_trip("[1,2,null,3,null,4]");
        round_trip("[5,3,8,1,4,7,9,0,null,null,null,6]");
    }

    #[test]
    fn long_left_chain() {
        let pre: Vec<i32> = (0..1_000).collect();
        let ino: Vec<i32> = (0..1_000).rev().collect();
        let root = Solution::build_tree(pre, ino);
        let mut depth = 0;
        let mut current = root;
        while let Some(node) = current {
            depth += 1;
            current = node.borrow().left.clone();
        }
        assert_eq!(depth, 1_000);
    }
}
