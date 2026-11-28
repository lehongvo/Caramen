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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** depth-first search with one mutable path. Each leaf turns the current path into a
//! string; after visiting a node's children, its value is popped again.
//!
//! **Complexity:** `O(n * h)` time and output size (each of up to `n / 2` leaves copies a path of
//! length `h`); `O(h)` extra space for the path.
//!
//! **Rust concepts:**
//! - The helper takes `&mut Vec<String>` twice (path and output): two different vectors, so two
//!   mutable borrows are fine.
//! - `[String]::join` concatenates with a separator.
// @solution-end

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    // @solution-start remove
    fn walk(node: &Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<String>, out: &mut Vec<String>) {
        let Some(node) = node else {
            return;
        };
        let node = node.borrow();
        path.push(node.val.to_string());
        if node.left.is_none() && node.right.is_none() {
            out.push(path.join("->"));
        } else {
            Self::walk(&node.left, path, out);
            Self::walk(&node.right, path, out);
        }
        path.pop();
    }
    // @solution-end

    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        // @solution-start
        let mut out = Vec::new();
        Self::walk(&root, &mut Vec::new(), &mut out);
        out
        // @solution-end
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
