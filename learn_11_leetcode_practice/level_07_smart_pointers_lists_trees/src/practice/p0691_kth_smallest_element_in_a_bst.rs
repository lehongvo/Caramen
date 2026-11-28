// ==================================================================
// PRACTICE 0691: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0691_kth_smallest_element_in_a_bst.md
// Run the tests:          cargo test practice::p0691
// ==================================================================

//! # Problem 0691: Kth Smallest Element in a BST
//!
//! - **Difficulty:** Medium
//! - **Topics:** iterative in-order traversal with an explicit stack of `Rc`s, stopping early
//! - **Source:** [LeetCode 230. Kth Smallest Element in a BST](https://leetcode.com/problems/kth-smallest-element-in-a-bst/)
//!
//! ## Problem
//!
//! Return the `k`-th smallest value (1-based) among the values of a binary search tree.
//!
//! Implement `Solution::kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [3, 1, 4, null, 2], k = 1
//! Output: 1
//!
//! Input:  root = [5, 3, 6, 2, 4, null, null, 1], k = 3
//! Output: 3
//! ```
//!
//! ## Constraints
//!
//! - `1 <= k <= n <= 10^4`, `0 <= val <= 10^4`; `root` is a valid BST.
//!
//! ## Hints
//!
//! 1. In-order traversal of a BST yields the values in increasing order; the answer is the `k`-th
//!    value it produces.
//! 2. Do it iteratively so you can stop as soon as you have counted `k` values: push the left
//!    spine onto a stack, pop a node (the next smallest), then push the left spine of its right
//!    child.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_tree;

    #[test]
    fn example_one() {
        assert_eq!(Solution::kth_smallest(parse_tree("[3,1,4,null,2]"), 1), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::kth_smallest(parse_tree("[5,3,6,2,4,null,null,1]"), 3),
            3
        );
    }

    #[test]
    fn every_k_of_a_balanced_tree() {
        let text = "[8,4,12,2,6,10,14,1,3,5,7,9,11,13,15]";
        for k in 1..=15 {
            assert_eq!(Solution::kth_smallest(parse_tree(text), k), k);
        }
    }

    #[test]
    fn single_node() {
        assert_eq!(Solution::kth_smallest(parse_tree("[0]"), 1), 0);
    }
}
