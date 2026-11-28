//! # Problem 0670: Path Sum
//!
//! - **Difficulty:** Easy
//! - **Topics:** root-to-leaf recursion, passing the remaining target down, `let ... else`, short-circuiting `||`
//! - **Source:** [LeetCode 112. Path Sum](https://leetcode.com/problems/path-sum/)
//!
//! ## Problem
//!
//! Return `true` if the tree has a **root-to-leaf** path whose values add up to `target_sum`, and
//! `false` otherwise. A leaf is a node without children; the empty tree has no paths at all.
//!
//! Implement `Solution::has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, 1], target_sum = 22
//! Output: true           // 5 -> 4 -> 11 -> 2
//!
//! Input:  root = [1, 2, 3], target_sum = 5
//! Output: false          // the paths sum to 3 and 4
//!
//! Input:  root = [], target_sum = 0
//! Output: false
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 5000`, `-1000 <= val <= 1000`, `-1000 <= target_sum <= 1000`
//!
//! ## Hints
//!
//! 1. Subtract the node's value from the target and ask the same question for its children.
//! 2. At a leaf, the answer is whether the remaining target equals the leaf's value.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** depth-first search that carries "how much is still needed". A leaf succeeds when its
//! value is exactly what remains; an inner node succeeds if either child succeeds with the reduced
//! target.
//!
//! **Complexity:** `O(n)` time, `O(h)` stack space.
//!
//! **Rust concepts:**
//! - `let Some(node) = root else { return false; };` handles the empty tree up front and leaves
//!   the rest of the function working with a real node.
//! - Recursing on `node.left.clone()` passes an owned `Option<Rc<..>>` to the LeetCode signature;
//!   cloning an `Rc` is cheap.
//! - `||` short-circuits, so the right subtree is skipped once the left one has a path.
// @solution-end

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        // @solution-start
        let Some(node) = root else {
            return false;
        };
        let node = node.borrow();
        let rest = target_sum - node.val;
        if node.left.is_none() && node.right.is_none() {
            return rest == 0;
        }
        Self::has_path_sum(node.left.clone(), rest) || Self::has_path_sum(node.right.clone(), rest)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_tree;

    #[test]
    fn example_one() {
        let root = parse_tree("[5,4,8,11,null,13,4,7,2,null,null,null,1]");
        assert!(Solution::has_path_sum(root, 22));
    }

    #[test]
    fn no_matching_path() {
        assert!(!Solution::has_path_sum(parse_tree("[1,2,3]"), 5));
        assert!(Solution::has_path_sum(parse_tree("[1,2,3]"), 4));
    }

    #[test]
    fn empty_tree_has_no_path() {
        assert!(!Solution::has_path_sum(None, 0));
    }

    #[test]
    fn inner_node_is_not_a_leaf() {
        // Path 1 alone sums to 1, but node 1 has a child, so it is not a leaf.
        assert!(!Solution::has_path_sum(parse_tree("[1,2]"), 1));
        assert!(Solution::has_path_sum(parse_tree("[1,2]"), 3));
        assert!(Solution::has_path_sum(parse_tree("[-2,null,-3]"), -5));
    }
}
