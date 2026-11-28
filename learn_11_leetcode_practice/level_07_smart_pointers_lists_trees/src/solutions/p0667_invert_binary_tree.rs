//! # Problem 0667: Invert Binary Tree
//!
//! - **Difficulty:** Easy
//! - **Topics:** mutating through `borrow_mut()`, `std::mem::swap` on two fields, returning the same `Rc`
//! - **Source:** [LeetCode 226. Invert Binary Tree](https://leetcode.com/problems/invert-binary-tree/)
//!
//! ## Problem
//!
//! Mirror a binary tree: for every node, swap its left and right subtrees. Return the root of the
//! inverted tree (the same root node, changed in place).
//!
//! Implement `Solution::invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [4, 2, 7, 1, 3, 6, 9]
//! Output: [4, 7, 2, 9, 6, 3, 1]
//!
//! Input:  root = [2, 1, 3]
//! Output: [2, 3, 1]
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
//! 1. Inverting a node means: swap its two children, then invert each child.
//! 2. `std::mem::swap(&mut a, &mut b)` exchanges two values. Through a `RefMut` guard, write
//!    `let mut guard = rc.borrow_mut(); let node = &mut *guard;` first: `node` is a plain
//!    `&mut TreeNode`, so `&mut node.left` and `&mut node.right` are two disjoint field borrows.
//!    (Writing `&mut guard.left, &mut guard.right` fails with `E0499`, because each field access
//!    calls `DerefMut` on the whole guard.)
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** visit every node (here with a stack), swap its children, and continue with them. The
//! tree is changed in place through the `RefCell`s, so the original root handle is returned.
//!
//! **Complexity:** `O(n)` time, `O(h)` extra space.
//!
//! **Rust concepts:**
//! - `std::mem::swap` exchanges two values behind `&mut` references without cloning them.
//! - `let node = &mut *guard;` turns the `RefMut` guard into one `&mut TreeNode`; after that, the
//!   borrow checker can see that `node.left` and `node.right` are different fields.
//! - Returning `root` hands the caller's ownership back; no node is recreated.
// @solution-end

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        // @solution-start
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = root.iter().cloned().collect();
        while let Some(rc) = stack.pop() {
            let mut guard = rc.borrow_mut();
            let node = &mut *guard;
            std::mem::swap(&mut node.left, &mut node.right);
            stack.extend(node.left.iter().chain(node.right.iter()).cloned());
        }
        root
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{parse_tree, tree_to_string};

    #[test]
    fn example_one() {
        let result = Solution::invert_tree(parse_tree("[4,2,7,1,3,6,9]"));
        assert_eq!(tree_to_string(&result), "[4,7,2,9,6,3,1]");
    }

    #[test]
    fn example_two() {
        let result = Solution::invert_tree(parse_tree("[2,1,3]"));
        assert_eq!(tree_to_string(&result), "[2,3,1]");
    }

    #[test]
    fn empty_and_lopsided() {
        assert_eq!(Solution::invert_tree(None), None);
        let result = Solution::invert_tree(parse_tree("[1,2,null,3]"));
        assert_eq!(tree_to_string(&result), "[1,null,2,null,3]");
    }

    #[test]
    fn inverting_twice_restores_the_tree() {
        let text = "[5,3,8,1,4,7,9,0,null,null,null,6]";
        let once = Solution::invert_tree(parse_tree(text));
        let original = parse_tree(text);
        assert_ne!(once, original);
        assert_eq!(Solution::invert_tree(once), original);
    }

    #[test]
    fn same_root_node_is_returned() {
        let root = parse_tree("[1,2,3]");
        let handle = root.clone();
        let result = Solution::invert_tree(root);
        assert!(Rc::ptr_eq(
            result.as_ref().unwrap(),
            handle.as_ref().unwrap()
        ));
    }
}
