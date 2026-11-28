//! # Problem 0666: Same Tree
//!
//! - **Difficulty:** Easy
//! - **Topics:** `match` on a pair of optional nodes, recursion with borrowed trees, derived `PartialEq`
//! - **Source:** [LeetCode 100. Same Tree](https://leetcode.com/problems/same-tree/)
//!
//! ## Problem
//!
//! Two binary trees are the same if they have the same shape and every pair of corresponding
//! nodes holds the same value. Return whether the trees `p` and `q` are the same.
//!
//! Implement
//! `Solution::is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  p = [1, 2, 3], q = [1, 2, 3]
//! Output: true
//!
//! Input:  p = [1, 2], q = [1, null, 2]
//! Output: false          // same values, different shape
//!
//! Input:  p = [1, 2, 1], q = [1, 1, 2]
//! Output: false
//! ```
//!
//! ## Constraints
//!
//! - Each tree has `0` to `100` nodes; `-10^4 <= val <= 10^4`.
//!
//! ## Hints
//!
//! 1. Match on `(p, q)`: both empty means equal; exactly one empty means different; both present
//!    means "equal values, and equal left subtrees, and equal right subtrees".
//! 2. A helper taking two `&Option<Rc<RefCell<TreeNode>>>` avoids cloning while recursing.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** structural recursion on both trees at once. `&&` short-circuits, so the comparison
//! stops at the first difference.
//!
//! **Complexity:** `O(min(n, m))` time, `O(h)` stack space.
//!
//! **Rust concepts:**
//! - `TreeNode` derives `PartialEq`, and `Rc<T>`, `RefCell<T>` and `Option<T>` all forward
//!   `PartialEq` to their contents, so `p == q` gives the same answer in one expression. Writing
//!   the recursion yourself is the point of this exercise.
//! - Matching on `(a, b)` with `a, b: &Option<..>` binds `&Rc<..>` references; nothing is moved.
// @solution-end

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    // @solution-start remove
    fn same(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                let (a, b) = (a.borrow(), b.borrow());
                a.val == b.val && Self::same(&a.left, &b.left) && Self::same(&a.right, &b.right)
            }
            _ => false,
        }
    }
    // @solution-end

    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // @solution-start
        Self::same(&p, &q)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::parse_tree;

    #[test]
    fn identical_trees() {
        assert!(Solution::is_same_tree(
            parse_tree("[1,2,3]"),
            parse_tree("[1,2,3]")
        ));
    }

    #[test]
    fn different_shape() {
        assert!(!Solution::is_same_tree(
            parse_tree("[1,2]"),
            parse_tree("[1,null,2]")
        ));
    }

    #[test]
    fn different_values() {
        assert!(!Solution::is_same_tree(
            parse_tree("[1,2,1]"),
            parse_tree("[1,1,2]")
        ));
        assert!(!Solution::is_same_tree(
            parse_tree("[5,4,7,3,null,2,null,-1,null,9]"),
            parse_tree("[5,4,7,3,null,2,null,-1,null,8]")
        ));
    }

    #[test]
    fn empty_trees() {
        assert!(Solution::is_same_tree(None, None));
        assert!(!Solution::is_same_tree(parse_tree("[0]"), None));
        assert!(!Solution::is_same_tree(None, parse_tree("[0]")));
    }
}
