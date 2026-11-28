// ==================================================================
// PRACTICE 0672: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0672_binary_tree_paths.md
// Run the tests:          cargo test practice::p0672
// ==================================================================

//! # Problem 0672: Binary Tree Paths
//!
//! - **Difficulty:** Easy
//! - **Topics:** backtracking with a shared path `Vec`, `join`, collecting `String`s
//! - **Source:** [LeetCode 257. Binary Tree Paths](https://leetcode.com/problems/binary-tree-paths/)
//!
//! ## Problem
//!
//! Return every root-to-leaf path of a binary tree as a string of values joined by `"->"`, for
//! example `"1->2->5"`. The paths may be returned in **any order**.
//!
//! Implement `Solution::binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [1, 2, 3, null, 5]
//! Output: ["1->2->5", "1->3"]
//!
//! Input:  root = [1]
//! Output: ["1"]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 100`, `-100 <= val <= 100`
//!
//! ## Hints
//!
//! 1. Walk the tree depth-first while keeping the values of the current path in a `Vec<String>`
//!    (or `Vec<i32>`).
//! 2. Push the node's value when entering it and `pop` it when leaving: this is *backtracking*, and
//!    it lets one vector serve every path.
//! 3. At a leaf, `path.join("->")` builds the answer string.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_tree;

    fn sorted(mut paths: Vec<String>) -> Vec<String> {
        paths.sort();
        paths
    }

    #[test]
    fn example_one() {
        let paths = Solution::binary_tree_paths(parse_tree("[1,2,3,null,5]"));
        assert_eq!(sorted(paths), vec!["1->2->5", "1->3"]);
    }

    #[test]
    fn single_node() {
        assert_eq!(Solution::binary_tree_paths(parse_tree("[1]")), vec!["1"]);
    }

    #[test]
    fn negative_values_and_chain() {
        let paths = Solution::binary_tree_paths(parse_tree("[-1,-2,null,-3]"));
        assert_eq!(paths, vec!["-1->-2->-3"]);
    }

    #[test]
    fn complete_tree() {
        let paths = Solution::binary_tree_paths(parse_tree("[1,2,3,4,5,6,7]"));
        assert_eq!(
            sorted(paths),
            vec!["1->2->4", "1->2->5", "1->3->6", "1->3->7"]
        );
    }
}
