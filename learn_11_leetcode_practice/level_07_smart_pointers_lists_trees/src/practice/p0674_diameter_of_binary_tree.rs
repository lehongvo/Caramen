// ==================================================================
// PRACTICE 0674: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0674_diameter_of_binary_tree.md
// Run the tests:          cargo test practice::p0674
// ==================================================================

//! # Problem 0674: Diameter of Binary Tree
//!
//! - **Difficulty:** Easy
//! - **Topics:** post-order recursion that returns one value and updates another through `&mut`
//! - **Source:** [LeetCode 543. Diameter of Binary Tree](https://leetcode.com/problems/diameter-of-binary-tree/)
//!
//! ## Problem
//!
//! The *diameter* of a binary tree is the number of **edges** on the longest path between any two
//! nodes. The path does not have to pass through the root. Return the diameter.
//!
//! Implement `Solution::diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [1, 2, 3, 4, 5]
//! Output: 3              // 4 -> 2 -> 1 -> 3 (or 5 -> 2 -> 1 -> 3)
//!
//! Input:  root = [1, 2]
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 10^4`, `-100 <= val <= 100`
//!
//! ## Hints
//!
//! 1. The longest path that turns around at a node `x` has `height(x.left) + height(x.right)`
//!    edges, where `height` counts nodes on the longest downward path.
//! 2. Compute heights bottom-up; while doing so, update a `best: &mut i32` with that sum at every
//!    node.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
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
            Solution::diameter_of_binary_tree(parse_tree("[1,2,3,4,5]")),
            3
        );
    }

    #[test]
    fn two_nodes_and_one_node() {
        assert_eq!(Solution::diameter_of_binary_tree(parse_tree("[1,2]")), 1);
        assert_eq!(Solution::diameter_of_binary_tree(parse_tree("[1]")), 0);
    }

    #[test]
    fn longest_path_avoids_the_root() {
        // Node 2 has two long arms; the root only adds a short branch.
        let root = parse_tree("[1,2,3,4,5,null,null,6,null,null,7,8,null,null,9]");
        assert_eq!(Solution::diameter_of_binary_tree(root), 6);
    }

    #[test]
    fn long_chain() {
        let chain = format!(
            "[{}]",
            (0..1_000)
                .map(|v| (v % 100).to_string())
                .collect::<Vec<_>>()
                .join(",null,")
        );
        assert_eq!(Solution::diameter_of_binary_tree(parse_tree(&chain)), 999);
    }
}
