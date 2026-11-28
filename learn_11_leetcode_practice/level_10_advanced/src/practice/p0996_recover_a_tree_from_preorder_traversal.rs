// ==================================================================
// PRACTICE 0996: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0996_recover_a_tree_from_preorder_traversal.md
// Run the tests:          cargo test practice::p0996
// ==================================================================

//! # Problem 0996: Recover a Tree From Preorder Traversal
//!
//! - **Difficulty:** Hard
//! - **Topics:** parsing with a byte cursor, a stack of `Rc<RefCell<TreeNode>>` indexed by depth, `Rc::clone`, `borrow_mut`
//! - **Source:** [LeetCode 1028. Recover a Tree From Preorder Traversal](https://leetcode.com/problems/recover-a-tree-from-preorder-traversal/)
//!
//! ## Problem
//!
//! A binary tree was printed by a preorder DFS: for each node, output `D` dashes (`D` = its depth,
//! the root has depth 0) followed by its value. If a node has exactly one child, it is the **left**
//! child. Given the printed string, rebuild the tree and return its root.
//!
//! Implement `Solution::recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>>`
//! using LeetCode's `TreeNode` (defined in this level's `common` module).
//!
//! ## Examples
//!
//! ```text
//! traversal = "1-2--3--4-5--6--7"     => [1,2,5,3,4,6,7]
//! traversal = "1-2--3---4-5--6---7"   => [1,2,5,3,null,6,null,4,null,7]
//! traversal = "1-401--349---90--88"   => [1,401,null,349,88,90]
//! ```
//!
//! ## Constraints
//!
//! - The tree has `1..=1000` nodes with values in `1..=10^9`.
//!
//! ## Hints
//!
//! 1. Read the string as a sequence of `(depth, value)` pairs.
//! 2. Keep the path from the root to the last node in a stack: `path[d]` is the most recent node at
//!    depth `d`.
//! 3. For a node at depth `d`, truncate the stack to length `d`; its parent is then the top. Attach it
//!    as the left child if that slot is free, otherwise as the right child.

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tree_to_string;

    fn run(text: &str) -> String {
        tree_to_string(&Solution::recover_from_preorder(text.to_string()))
    }

    #[test]
    fn examples() {
        assert_eq!(run("1-2--3--4-5--6--7"), "[1,2,5,3,4,6,7]");
        assert_eq!(run("1-2--3---4-5--6---7"), "[1,2,5,3,null,6,null,4,null,7]");
        assert_eq!(run("1-401--349---90--88"), "[1,401,null,349,88,90]");
    }

    #[test]
    fn single_node_and_chains() {
        assert_eq!(run("1000000000"), "[1000000000]");
        assert_eq!(run("1-2--3"), "[1,2,null,3]");
        assert_eq!(run("1-2-3"), "[1,2,3]");
    }

    #[test]
    fn round_trip_with_a_printer() {
        fn print(node: &Option<Rc<RefCell<TreeNode>>>, depth: usize, out: &mut String) {
            if let Some(n) = node {
                let n = n.borrow();
                out.push_str(&"-".repeat(depth));
                out.push_str(&n.val.to_string());
                print(&n.left, depth + 1, out);
                print(&n.right, depth + 1, out);
            }
        }
        let tree = crate::common::parse_tree("[5,3,8,1,4,7,9,2,null,null,null,6]");
        let mut text = String::new();
        print(&tree, 0, &mut text);
        assert_eq!(text, "5-3--1---2--4-8--7---6--9");
        assert_eq!(run(&text), tree_to_string(&tree));
    }

    #[test]
    fn deep_left_chain() {
        let text: String = (0..1_000)
            .map(|d| format!("{}{}", "-".repeat(d), d + 1))
            .collect();
        let root = Solution::recover_from_preorder(text);
        let mut depth = 0;
        let mut current = root;
        while let Some(node) = current {
            depth += 1;
            current = node.borrow().left.clone();
        }
        assert_eq!(depth, 1_000);
    }
}
