// ==================================================================
// PRACTICE 0671: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0671_sum_of_left_leaves.md
// Run the tests:          cargo test practice::p0671
// ==================================================================

//! # Problem 0671: Sum of Left Leaves
//!
//! - **Difficulty:** Easy
//! - **Topics:** passing context (`is_left`) down a recursion, leaf tests on borrowed nodes
//! - **Source:** [LeetCode 404. Sum of Left Leaves](https://leetcode.com/problems/sum-of-left-leaves/)
//!
//! ## Problem
//!
//! A *left leaf* is a leaf (a node without children) that is the **left** child of its parent.
//! Return the sum of the values of all left leaves. A tree made of a single node has no left
//! leaves (the root is nobody's child).
//!
//! Implement `Solution::sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [3, 9, 20, null, null, 15, 7]
//! Output: 24             // 9 + 15
//!
//! Input:  root = [1]
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 1000`, `-1000 <= val <= 1000`
//!
//! ## Hints
//!
//! 1. A node cannot know by itself whether it is a left child. Pass that information down:
//!    `fn sum(node, is_left: bool) -> i32`.
//! 2. At a leaf, return its value if `is_left`, otherwise `0`.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
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
            Solution::sum_of_left_leaves(parse_tree("[3,9,20,null,null,15,7]")),
            24
        );
    }

    #[test]
    fn single_root_is_not_a_left_leaf() {
        assert_eq!(Solution::sum_of_left_leaves(parse_tree("[1]")), 0);
    }

    #[test]
    fn right_leaves_do_not_count() {
        assert_eq!(
            Solution::sum_of_left_leaves(parse_tree("[1,null,2,null,3]")),
            0
        );
        assert_eq!(Solution::sum_of_left_leaves(parse_tree("[1,2,3,4,5]")), 4);
    }

    #[test]
    fn negative_values() {
        assert_eq!(
            Solution::sum_of_left_leaves(parse_tree("[0,-5,7,-3,null,-1,2]")),
            -4
        );
    }
}
