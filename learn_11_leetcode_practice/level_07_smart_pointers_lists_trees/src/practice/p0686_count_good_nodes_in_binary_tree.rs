// ==================================================================
// PRACTICE 0686: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0686_count_good_nodes_in_binary_tree.md
// Run the tests:          cargo test practice::p0686
// ==================================================================

//! # Problem 0686: Count Good Nodes in Binary Tree
//!
//! - **Difficulty:** Medium
//! - **Topics:** DFS carrying the path maximum, explicit stack of `(node, max_so_far)` pairs
//! - **Source:** [LeetCode 1448. Count Good Nodes in Binary Tree](https://leetcode.com/problems/count-good-nodes-in-binary-tree/)
//!
//! ## Problem
//!
//! A node `X` is *good* if no node on the path from the root down to `X` (the root and `X`
//! included) has a value **greater** than `X`'s value. Return the number of good nodes. The root is
//! always good.
//!
//! Implement `Solution::good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [3, 1, 4, 3, null, 1, 5]
//! Output: 4              // 3 (root), 4, 5 and the lower 3
//!
//! Input:  root = [3, 3, null, 4, 2]
//! Output: 3              // 3, 3, 4  (2 is beaten by 3)
//!
//! Input:  root = [1]
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 10^5`, `-10^4 <= val <= 10^4`
//!
//! ## Hints
//!
//! 1. Whether a node is good depends only on the **maximum** value above it, so carry that one
//!    number down the tree.
//! 2. A node is good when `val >= max_so_far`; its children then see `max(max_so_far, val)`.
//! 3. With up to `10^5` nodes in a chain, an explicit stack of `(node, max)` pairs is safer than
//!    recursion.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_tree;

    #[test]
    fn example_one() {
        assert_eq!(Solution::good_nodes(parse_tree("[3,1,4,3,null,1,5]")), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::good_nodes(parse_tree("[3,3,null,4,2]")), 3);
    }

    #[test]
    fn single_node() {
        assert_eq!(Solution::good_nodes(parse_tree("[1]")), 1);
    }

    #[test]
    fn increasing_and_decreasing_chains() {
        let up = format!(
            "[{}]",
            (1..=500)
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",null,")
        );
        assert_eq!(Solution::good_nodes(parse_tree(&up)), 500);
        let down = format!(
            "[{}]",
            (1..=500)
                .rev()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",null,")
        );
        assert_eq!(Solution::good_nodes(parse_tree(&down)), 1);
    }
}
