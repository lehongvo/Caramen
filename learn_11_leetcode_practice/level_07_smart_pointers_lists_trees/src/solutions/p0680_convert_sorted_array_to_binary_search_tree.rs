//! # Problem 0680: Convert Sorted Array to Binary Search Tree
//!
//! - **Difficulty:** Easy
//! - **Topics:** divide and conquer on slices, `split_at`, building `Rc<RefCell<TreeNode>>` bottom-up
//! - **Source:** [LeetCode 108. Convert Sorted Array to Binary Search Tree](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/)
//!
//! ## Problem
//!
//! Turn a strictly increasing array into a **height-balanced** binary search tree: for every node,
//! the heights of its two subtrees differ by at most one, and an in-order traversal gives back the
//! array. Several trees qualify; any of them is accepted.
//!
//! Implement `Solution::sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [-10, -3, 0, 5, 9]
//! Output: [0, -3, 9, -10, null, 5]      // [0, -10, 5, null, -3, null, 9] is also accepted
//!
//! Input:  nums = [1, 3]
//! Output: [3, 1]                        // or [1, null, 3]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^4`, `-10^4 <= nums[i] <= 10^4`, strictly increasing.
//!
//! ## Hints
//!
//! 1. Make the middle element the root: half of the values go left, half go right.
//! 2. Build the left subtree from the left half and the right subtree from the right half,
//!    recursively. A helper taking a slice `&[i32]` avoids copying.
//! 3. `let (left, rest) = nums.split_at(mid);` then `rest[0]` is the root value and `&rest[1..]`
//!    the right half.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** recursively pick the middle of the current slice as the root. Both halves differ in
//! size by at most one, so the heights of the subtrees differ by at most one at every node.
//!
//! **Complexity:** `O(n)` time, `O(log n)` stack space.
//!
//! **Rust concepts:**
//! - Slices make divide and conquer cheap: `split_at` returns two borrowed views, nothing is copied.
//! - `slice.split_first()` returns `Option<(&T, &[T])>`; here `split_at` + indexing is simpler
//!   because the middle element is wanted.
// @solution-end

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    // @solution-start remove
    fn build(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let (left, rest) = nums.split_at(nums.len() / 2);
        Some(Rc::new(RefCell::new(TreeNode {
            val: rest[0],
            left: Self::build(left),
            right: Self::build(&rest[1..]),
        })))
    }
    // @solution-end

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // @solution-start
        Self::build(&nums)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            inorder(&node.left, out);
            out.push(node.val);
            inorder(&node.right, out);
        }
    }

    /// Height of a balanced subtree, or `None` if some node is unbalanced.
    fn balanced_height(node: &Option<Rc<RefCell<TreeNode>>>) -> Option<usize> {
        let Some(node) = node else {
            return Some(0);
        };
        let node = node.borrow();
        let (l, r) = (balanced_height(&node.left)?, balanced_height(&node.right)?);
        (l.abs_diff(r) <= 1).then_some(1 + l.max(r))
    }

    fn assert_valid(nums: Vec<i32>) {
        let root = Solution::sorted_array_to_bst(nums.clone());
        let mut values = Vec::new();
        inorder(&root, &mut values);
        assert_eq!(values, nums, "in-order traversal must give the array back");
        assert!(
            balanced_height(&root).is_some(),
            "tree must be height-balanced"
        );
    }

    #[test]
    fn example_one() {
        assert_valid(vec![-10, -3, 0, 5, 9]);
    }

    #[test]
    fn example_two() {
        assert_valid(vec![1, 3]);
    }

    #[test]
    fn single_element() {
        assert_valid(vec![42]);
    }

    #[test]
    fn large_array() {
        assert_valid((-5_000..5_000).collect());
        assert_valid((0..1_023).collect());
    }
}
