// ==================================================================
// PRACTICE 0681: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0681_minimum_absolute_difference_in_bst.md
// Run the tests:          cargo test practice::p0681
// ==================================================================

//! # Problem 0681: Minimum Absolute Difference in BST
//!
//! - **Difficulty:** Easy
//! - **Topics:** iterative in-order traversal with a stack, carrying the previous value in an `Option`
//! - **Source:** [LeetCode 530. Minimum Absolute Difference in BST](https://leetcode.com/problems/minimum-absolute-difference-in-bst/)
//!
//! ## Problem
//!
//! Given a binary search tree with at least two nodes, return the smallest absolute difference
//! between the values of any two **different** nodes.
//!
//! Implement `Solution::get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [4, 2, 6, 1, 3]
//! Output: 1
//!
//! Input:  root = [1, 0, 48, null, null, 12, 49]
//! Output: 1              // 48 and 49
//! ```
//!
//! ## Constraints
//!
//! - `2 <= n <= 10^4`, `0 <= val <= 10^5`; `root` is a valid BST.
//!
//! ## Hints
//!
//! 1. An in-order traversal of a BST visits the values in increasing order.
//! 2. In a sorted sequence, the closest pair is always two **neighbours**, so compare each value
//!    with the previous one only.
//! 3. Iterative in-order: push left children while you can, pop a node, visit it, then continue
//!    with its right child.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_tree;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::get_minimum_difference(parse_tree("[4,2,6,1,3]")),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::get_minimum_difference(parse_tree("[1,0,48,null,null,12,49]")),
            1
        );
    }

    #[test]
    fn two_nodes() {
        assert_eq!(
            Solution::get_minimum_difference(parse_tree("[0,null,100000]")),
            100_000
        );
    }

    #[test]
    fn closest_pair_across_levels() {
        // In-order: 10, 20, 29, 30, 50, 70; the closest pair is 29 and 30 (root and a deep node).
        assert_eq!(
            Solution::get_minimum_difference(parse_tree("[30,20,50,10,29,null,70]")),
            1
        );
    }
}
