// ==================================================================
// PRACTICE 0687: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0687_path_sum_ii.md
// Run the tests:          cargo test practice::p0687
// ==================================================================

//! # Problem 0687: Path Sum II
//!
//! - **Difficulty:** Medium
//! - **Topics:** backtracking with one path `Vec`, cloning the path at a leaf, `Vec<Vec<i32>>` results
//! - **Source:** [LeetCode 113. Path Sum II](https://leetcode.com/problems/path-sum-ii/)
//!
//! ## Problem
//!
//! Return **all** root-to-leaf paths whose values add up to `target_sum`. Each path is the list of
//! node values from the root to the leaf. The paths may be returned in any order.
//!
//! Implement
//! `Solution::path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1], target_sum = 22
//! Output: [[5, 4, 11, 2], [5, 8, 4, 5]]
//!
//! Input:  root = [1, 2, 3], target_sum = 5
//! Output: []
//!
//! Input:  root = [1, 2], target_sum = 0
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 5000`, `-1000 <= val <= 1000`, `-1000 <= target_sum <= 1000`
//!
//! ## Hints
//!
//! 1. This is problem 0670 (Path Sum) plus problem 0672's backtracking: keep the current path in a
//!    `Vec<i32>`, push on entry and pop on exit.
//! 2. When you reach a leaf and the remaining target is exactly its value, push `path.clone()` into
//!    the answer.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_tree;

    fn sorted(mut paths: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        paths.sort();
        paths
    }

    #[test]
    fn example_one() {
        let root = parse_tree("[5,4,8,11,null,13,4,7,2,null,null,5,1]");
        assert_eq!(
            sorted(Solution::path_sum(root, 22)),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        );
    }

    #[test]
    fn no_matching_paths() {
        assert!(Solution::path_sum(parse_tree("[1,2,3]"), 5).is_empty());
        assert!(Solution::path_sum(parse_tree("[1,2]"), 0).is_empty());
        assert!(Solution::path_sum(None, 0).is_empty());
    }

    #[test]
    fn negative_values_and_many_paths() {
        assert_eq!(
            Solution::path_sum(parse_tree("[-2,null,-3]"), -5),
            vec![vec![-2, -3]]
        );
        let root = parse_tree("[0,1,1,-1,-1,-1,-1]");
        assert_eq!(sorted(Solution::path_sum(root, 0)).len(), 4);
    }
}
