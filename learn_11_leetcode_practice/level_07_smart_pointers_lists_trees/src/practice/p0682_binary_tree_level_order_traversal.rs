// ==================================================================
// PRACTICE 0682: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0682_binary_tree_level_order_traversal.md
// Run the tests:          cargo test practice::p0682
// ==================================================================

//! # Problem 0682: Binary Tree Level Order Traversal
//!
//! - **Difficulty:** Medium
//! - **Topics:** breadth-first search level by level, building the next level as a `Vec`, `flat_map`
//! - **Source:** [LeetCode 102. Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/)
//!
//! ## Problem
//!
//! Return the values of a binary tree level by level: one inner vector per depth, from the root's
//! level downward, each listing its values from left to right.
//!
//! Implement `Solution::level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [3, 9, 20, null, null, 15, 7]
//! Output: [[3], [9, 20], [15, 7]]
//!
//! Input:  root = [1]
//! Output: [[1]]
//!
//! Input:  root = []
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 2000`, `-1000 <= val <= 1000`
//!
//! ## Hints
//!
//! 1. Keep the nodes of the current level in a `Vec`. Their values form one output row.
//! 2. The next level is the list of all their children, left to right. Stop when a level is empty.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{parse_tree, tree};

    #[test]
    fn example_one() {
        let root = parse_tree("[3,9,20,null,null,15,7]");
        assert_eq!(
            Solution::level_order(root),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }

    #[test]
    fn single_and_empty() {
        assert_eq!(Solution::level_order(parse_tree("[1]")), vec![vec![1]]);
        assert!(Solution::level_order(None).is_empty());
    }

    #[test]
    fn gaps_inside_levels() {
        let root = parse_tree("[1,2,3,null,4,5,null,6]");
        assert_eq!(
            Solution::level_order(root),
            vec![vec![1], vec![2, 3], vec![4, 5], vec![6]]
        );
    }

    #[test]
    fn complete_tree_levels() {
        let values: Vec<Option<i32>> = (1..=15).map(Some).collect();
        let levels = Solution::level_order(tree(&values));
        assert_eq!(levels.len(), 4);
        assert_eq!(levels[3], (8..=15).collect::<Vec<_>>());
    }
}
