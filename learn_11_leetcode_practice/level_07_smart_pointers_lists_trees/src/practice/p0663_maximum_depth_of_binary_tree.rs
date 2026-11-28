// ==================================================================
// PRACTICE 0663: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0663_maximum_depth_of_binary_tree.md
// Run the tests:          cargo test practice::p0663
// ==================================================================

//! # Problem 0663: Maximum Depth of Binary Tree
//!
//! - **Difficulty:** Easy
//! - **Topics:** recursion over `&Option<Rc<RefCell<TreeNode>>>`, `borrow()`, `i32::max`
//! - **Source:** [LeetCode 104. Maximum Depth of Binary Tree](https://leetcode.com/problems/maximum-depth-of-binary-tree/)
//!
//! ## Problem
//!
//! The depth of a binary tree is the number of nodes on its longest path from the root down to a
//! leaf. Return it (`0` for the empty tree).
//!
//! Implement `Solution::max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [3, 9, 20, null, null, 15, 7]
//! Output: 3              // 3 -> 20 -> 15 (or 7)
//!
//! Input:  root = [1, null, 2]
//! Output: 2
//!
//! Input:  root = []
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 10^4`, `-100 <= val <= 100`
//!
//! ## Hints
//!
//! 1. The depth of a node is `1 + max(depth(left), depth(right))`; the depth of `None` is `0`.
//! 2. A helper that takes `&Option<Rc<RefCell<TreeNode>>>` lets you recurse without cloning any
//!    `Rc`.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{parse_tree, tree};

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::max_depth(parse_tree("[3,9,20,null,null,15,7]")),
            3
        );
    }

    #[test]
    fn right_leaning() {
        assert_eq!(Solution::max_depth(parse_tree("[1,null,2]")), 2);
    }

    #[test]
    fn empty_and_single() {
        assert_eq!(Solution::max_depth(None), 0);
        assert_eq!(Solution::max_depth(parse_tree("[0]")), 1);
    }

    #[test]
    fn perfect_tree_and_long_chain() {
        let perfect: Vec<Option<i32>> = (1..=1023).map(Some).collect();
        assert_eq!(Solution::max_depth(tree(&perfect)), 10);
        let chain = format!(
            "[{}]",
            (0..1_000)
                .map(|v| (v % 100).to_string())
                .collect::<Vec<_>>()
                .join(",null,")
        );
        assert_eq!(Solution::max_depth(parse_tree(&chain)), 1_000);
    }
}
