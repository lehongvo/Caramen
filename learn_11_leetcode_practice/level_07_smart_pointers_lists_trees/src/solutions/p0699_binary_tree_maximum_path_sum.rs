//! # Problem 0699: Binary Tree Maximum Path Sum
//!
//! - **Difficulty:** Hard
//! - **Topics:** post-order "gain" recursion, clamping negative branches with `max(0)`, `&mut i32` accumulator
//! - **Source:** [LeetCode 124. Binary Tree Maximum Path Sum](https://leetcode.com/problems/binary-tree-maximum-path-sum/)
//!
//! ## Problem
//!
//! A *path* in a binary tree is a sequence of nodes where each consecutive pair is connected by an
//! edge, and no node appears twice. A path has at least one node and does **not** need to pass
//! through the root. Its *sum* is the sum of its values. Return the largest path sum.
//!
//! Implement `Solution::max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  root = [1, 2, 3]
//! Output: 6              // 2 -> 1 -> 3
//!
//! Input:  root = [-10, 9, 20, null, null, 15, 7]
//! Output: 42             // 15 -> 20 -> 7
//!
//! Input:  root = [-3]
//! Output: -3             // a path has at least one node
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 3 * 10^4`, `-1000 <= val <= 1000`
//!
//! ## Hints
//!
//! 1. Every path has a single highest node where it "turns". At that node the path is
//!    `left branch + node + right branch`, where each branch goes straight down.
//! 2. Define `gain(node)`: the best sum of a downward path **starting** at `node`. It is
//!    `node.val + max(0, gain(left), gain(right))`; a negative branch is better left out.
//! 3. While computing gains bottom-up, update the answer with
//!    `node.val + max(0, gain(left)) + max(0, gain(right))`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** one post-order pass. Each call returns the node's downward gain to its parent and
//! records the best path that turns at the node. Clamping child gains at `0` drops harmful
//! branches; the answer starts at `i32::MIN` so all-negative trees work.
//!
//! **Complexity:** `O(n)` time, `O(h)` stack space.
//!
//! **Rust concepts:**
//! - The function returns one quantity (gain) and accumulates another (`best: &mut i32`): two
//!   different meanings, kept apart.
//! - Values are small (`|val| <= 1000`, `n <= 3 * 10^4`), so `i32` sums cannot overflow.
// @solution-end

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    // @solution-start remove
    fn gain(node: &Option<Rc<RefCell<TreeNode>>>, best: &mut i32) -> i32 {
        let Some(node) = node else {
            return 0;
        };
        let node = node.borrow();
        let left = Self::gain(&node.left, best).max(0);
        let right = Self::gain(&node.right, best).max(0);
        *best = (*best).max(node.val + left + right);
        node.val + left.max(right)
    }
    // @solution-end

    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // @solution-start
        let mut best = i32::MIN;
        Self::gain(&root, &mut best);
        best
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_tree;

    #[test]
    fn path_through_the_root() {
        assert_eq!(Solution::max_path_sum(parse_tree("[1,2,3]")), 6);
    }

    #[test]
    fn path_avoiding_the_root() {
        assert_eq!(
            Solution::max_path_sum(parse_tree("[-10,9,20,null,null,15,7]")),
            42
        );
    }

    #[test]
    fn all_negative_values() {
        assert_eq!(Solution::max_path_sum(parse_tree("[-3]")), -3);
        assert_eq!(Solution::max_path_sum(parse_tree("[-2,-1]")), -1);
        assert_eq!(Solution::max_path_sum(parse_tree("[-5,-4,-3]")), -3);
    }

    #[test]
    fn negative_branch_is_skipped() {
        // 4 -> 2 -> 5 and 5 -> 2 -> 1 -> 3 both sum to 11; the -100 leaves are never worth taking.
        assert_eq!(
            Solution::max_path_sum(parse_tree("[1,2,3,4,5,-100,-100]")),
            11
        );
        // 7 -> 11 -> 4 -> 5 -> 8 -> 13 = 48.
        assert_eq!(
            Solution::max_path_sum(parse_tree("[5,4,8,11,null,13,4,7,2,null,null,null,1]")),
            48
        );
    }
}
