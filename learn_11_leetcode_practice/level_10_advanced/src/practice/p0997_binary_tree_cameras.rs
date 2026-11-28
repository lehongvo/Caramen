// ==================================================================
// PRACTICE 0997: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0997_binary_tree_cameras.md
// Run the tests:          cargo test practice::p0997
// ==================================================================

//! # Problem 0997: Binary Tree Cameras
//!
//! - **Difficulty:** Hard
//! - **Topics:** greedy post-order tree DP with three states, an `enum` for node states, recursion over `Option<Rc<RefCell<TreeNode>>>`
//! - **Source:** [LeetCode 968. Binary Tree Cameras](https://leetcode.com/problems/binary-tree-cameras/)
//!
//! ## Problem
//!
//! Cameras can be installed on nodes of a binary tree. A camera monitors its own node, its parent and
//! its immediate children. Return the minimum number of cameras needed to monitor **every** node.
//!
//! Implement `Solution::min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! root = [0,0,null,0,0]              => 1     (one camera on the left child)
//! root = [0,0,null,0,null,0,null,null,0] => 2
//! root = [0]                         => 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= number of nodes <= 1000`; all values are `0`.
//!
//! ## Hints
//!
//! 1. Leaves should never hold a camera: putting it on their parent covers at least as much.
//! 2. Process children before parents and classify each node as `HasCamera`, `Covered` (by a child)
//!    or `NeedsCover`. A missing child counts as `Covered`.
//! 3. If any child needs cover, the node must take a camera; else if any child has a camera, the node
//!    is covered; else it needs cover from its parent. At the root, "needs cover" costs one more
//!    camera.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_tree;

    fn run(tree: &str) -> i32 {
        Solution::min_camera_cover(parse_tree(tree))
    }

    #[test]
    fn examples() {
        assert_eq!(run("[0,0,null,0,0]"), 1);
        assert_eq!(run("[0,0,null,0,null,0,null,null,0]"), 2);
        assert_eq!(run("[0]"), 1);
    }

    #[test]
    fn small_shapes() {
        assert_eq!(run("[0,0]"), 1);
        assert_eq!(run("[0,0,0]"), 1);
        assert_eq!(run("[0,0,0,0,0,0,0]"), 2);
        assert_eq!(run("[0,null,0,null,0,null,0]"), 2);
    }

    #[test]
    fn chain_needs_one_camera_per_three_nodes() {
        for n in 1..=30_usize {
            let mut items = vec!["0".to_string()];
            for _ in 1..n {
                items.push("0".to_string());
                items.push("null".to_string());
            }
            let text = format!("[{}]", items.join(","));
            assert_eq!(run(&text), n.div_ceil(3) as i32, "chain of {n}");
        }
    }

    #[test]
    fn complete_tree_of_1023_nodes() {
        let text = format!("[{}]", vec!["0"; 1_023].join(","));
        // Depths 0..=9: cameras on all nodes at depths 8 (256), 5 (32) and 2 (4), plus the root.
        let cameras = run(&text);
        assert_eq!(cameras, 256 + 32 + 4 + 1);
    }
}
