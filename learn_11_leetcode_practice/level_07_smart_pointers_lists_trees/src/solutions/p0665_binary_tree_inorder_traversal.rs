//! # Problem 0665: Binary Tree Inorder Traversal
//!
//! - **Difficulty:** Easy
//! - **Topics:** recursive helper with `&Option<Rc<RefCell<TreeNode>>>` and `&mut Vec<i32>`
//! - **Source:** [LeetCode 94. Binary Tree Inorder Traversal](https://leetcode.com/problems/binary-tree-inorder-traversal/)
//!
//! ## Problem
//!
//! Return the values of a binary tree in **in-order**: the whole left subtree, then the node,
//! then the whole right subtree. (For a binary search tree this lists the values in increasing
//! order.)
//!
//! Implement `Solution::inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [1, null, 2, 3]
//! Output: [1, 3, 2]
//!
//! Input:  root = [1, 2, 3, 4, 5, null, 8, null, null, 6, 7, 9]
//! Output: [4, 2, 6, 5, 7, 1, 3, 9, 8]
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
//! 1. Write `fn visit(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>)`: visit the left
//!    child, push the value, visit the right child.
//! 2. Passing the output vector as `&mut Vec<i32>` avoids building and concatenating many small
//!    vectors.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** a recursive helper that appends to one shared output vector.
//!
//! **Complexity:** `O(n)` time, `O(h)` stack space.
//!
//! **Rust concepts:**
//! - The helper takes the tree by shared reference and the result by mutable reference: two
//!   different values, so both borrows coexist.
//! - The iterative version (a stack of nodes plus a "current" cursor) is problem 0691's technique.
// @solution-end

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    // @solution-start remove
    fn visit(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            Self::visit(&node.left, out);
            out.push(node.val);
            Self::visit(&node.right, out);
        }
    }
    // @solution-end

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // @solution-start
        let mut out = Vec::new();
        Self::visit(&root, &mut out);
        out
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{parse_tree, tree};

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::inorder_traversal(parse_tree("[1,null,2,3]")),
            vec![1, 3, 2]
        );
    }

    #[test]
    fn example_two() {
        let root = parse_tree("[1,2,3,4,5,null,8,null,null,6,7,9]");
        assert_eq!(
            Solution::inorder_traversal(root),
            vec![4, 2, 6, 5, 7, 1, 3, 9, 8]
        );
    }

    #[test]
    fn empty_and_single() {
        assert!(Solution::inorder_traversal(None).is_empty());
        assert_eq!(
            Solution::inorder_traversal(parse_tree("[-100]")),
            vec![-100]
        );
    }

    #[test]
    fn binary_search_tree_comes_out_sorted() {
        let root = parse_tree("[8,4,12,2,6,10,14,1,3,5,7,9,11,13,15]");
        assert_eq!(
            Solution::inorder_traversal(root),
            (1..=15).collect::<Vec<_>>()
        );
        let values: Vec<Option<i32>> = (1..=3).map(Some).collect();
        assert_eq!(Solution::inorder_traversal(tree(&values)), vec![2, 1, 3]);
    }
}
