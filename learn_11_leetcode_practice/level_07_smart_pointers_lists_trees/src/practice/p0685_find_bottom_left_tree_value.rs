// ==================================================================
// PRACTICE 0685: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0685_find_bottom_left_tree_value.md
// Run the tests:          cargo test practice::p0685
// ==================================================================

//! # Problem 0685: Find Bottom Left Tree Value
//!
//! - **Difficulty:** Medium
//! - **Topics:** BFS that visits right children before left ones, "last dequeued" as the answer
//! - **Source:** [LeetCode 513. Find Bottom Left Tree Value](https://leetcode.com/problems/find-bottom-left-tree-value/)
//!
//! ## Problem
//!
//! Return the value of the **leftmost** node on the **deepest** level of a non-empty binary tree.
//!
//! Implement `Solution::find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [2, 1, 3]
//! Output: 1
//!
//! Input:  root = [1, 2, 3, 4, null, 5, 6, null, null, 7]
//! Output: 7              // the deepest level contains only 7
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 10^4`, `-2^31 <= val <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. A normal BFS (left child first) visits the leftmost node of the deepest level first among the
//!    nodes of that level, but you only know it is the deepest level at the very end.
//! 2. Reverse the order: enqueue the **right** child before the left one. Then the very **last**
//!    node the BFS visits is the bottom-left node.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_tree;

    #[test]
    fn small_tree() {
        assert_eq!(Solution::find_bottom_left_value(parse_tree("[2,1,3]")), 1);
    }

    #[test]
    fn deepest_node_in_the_right_subtree() {
        let root = parse_tree("[1,2,3,4,null,5,6,null,null,7]");
        assert_eq!(Solution::find_bottom_left_value(root), 7);
    }

    #[test]
    fn single_node_and_extremes() {
        assert_eq!(
            Solution::find_bottom_left_value(parse_tree("[-2147483648]")),
            i32::MIN
        );
        let root = parse_tree("[0,null,2147483647]");
        assert_eq!(Solution::find_bottom_left_value(root), i32::MAX);
    }

    #[test]
    fn leftmost_of_a_full_level() {
        let root = parse_tree("[1,2,3,4,5,6,7]");
        assert_eq!(Solution::find_bottom_left_value(root), 4);
    }
}
