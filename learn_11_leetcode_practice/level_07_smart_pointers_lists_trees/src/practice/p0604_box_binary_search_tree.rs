// ==================================================================
// PRACTICE 0604: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0604_box_binary_search_tree.md
// Run the tests:          cargo test practice::p0604
// ==================================================================

//! # Problem 0604: Binary Search Tree with Box
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Option<Box<Node>>`, `&mut Option<Box<T>>` cursor, `Ordering`, recursion vs loops
//! - **Source:** Original · [The Rust Book 15.1](https://doc.rust-lang.org/book/ch15-01-box.html) · [`Option::as_deref`](https://doc.rust-lang.org/std/option/enum.Option.html#method.as_deref)
//!
//! ## Problem
//!
//! When a tree has a single owner for every node, `Option<Box<Node>>` is the simplest way to link
//! the nodes: `None` is an empty subtree, `Some(boxed)` owns the child. (LeetCode's `TreeNode` uses
//! `Rc<RefCell<..>>` instead; later problems in this level use it.)
//!
//! Implement a set of distinct `i32` values stored as a binary search tree (BST): every value in a
//! node's left subtree is smaller than the node, every value in its right subtree is larger.
//!
//! - `BoxBst::new() -> BoxBst`: an empty tree;
//! - `insert(&mut self, value: i32) -> bool`: add `value`; return `false` (and change nothing) if
//!   it is already present, `true` otherwise;
//! - `contains(&self, value: i32) -> bool`;
//! - `len(&self) -> usize` and `is_empty(&self) -> bool`;
//! - `min(&self) -> Option<i32>`: the smallest value, `None` for an empty tree;
//! - `in_order(&self) -> Vec<i32>`: all values in increasing order;
//! - `height(&self) -> usize`: the number of nodes on the longest root-to-leaf path (`0` when empty).
//!
//! The internal node type is up to you.
//!
//! ## Examples
//!
//! ```text
//! let mut bst = BoxBst::new();
//! for v in [5, 3, 8, 1, 4] { bst.insert(v); }     // all return true
//! bst.insert(3)        -> false                     // already there
//! bst.len()            -> 5
//! bst.contains(4)      -> true,  bst.contains(7) -> false
//! bst.min()            -> Some(1)
//! bst.in_order()       -> [1, 3, 4, 5, 8]
//! bst.height()         -> 3                         // 5 -> 3 -> 1
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` calls in total.
//! - Values may arrive in sorted order, so the tree can become a long chain: `insert`, `contains`
//!   and `min` should use loops, not recursion.
//!
//! ## Hints
//!
//! 1. A private `struct Node { value: i32, left: Option<Box<Node>>, right: Option<Box<Node>> }`
//!    and a field `root: Option<Box<Node>>` are enough (plus a counter for `len`).
//! 2. For `insert`, walk a cursor `let mut slot = &mut self.root;`. While `slot` holds a node, move
//!    it to `&mut node.left` or `&mut node.right`. When the loop ends, `slot` is the empty place
//!    where the new node belongs: `*slot = Some(Box::new(...))`.
//! 3. For read-only walks, `let mut current = self.root.as_deref();` gives an `Option<&Node>`.

#[derive(Default)]
pub struct BoxBst {}

impl BoxBst {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn insert(&mut self, value: i32) -> bool {
        todo!("Write your solution here")
    }

    pub fn contains(&self, value: i32) -> bool {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn min(&self) -> Option<i32> {
        todo!("Write your solution here")
    }

    pub fn in_order(&self) -> Vec<i32> {
        todo!("Write your solution here")
    }

    pub fn height(&self) -> usize {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build(values: &[i32]) -> BoxBst {
        let mut bst = BoxBst::new();
        for &v in values {
            bst.insert(v);
        }
        bst
    }

    #[test]
    fn example_tree() {
        let mut bst = build(&[5, 3, 8, 1, 4]);
        assert!(!bst.insert(3));
        assert_eq!(bst.len(), 5);
        assert!(bst.contains(4));
        assert!(!bst.contains(7));
        assert_eq!(bst.min(), Some(1));
        assert_eq!(bst.in_order(), vec![1, 3, 4, 5, 8]);
        assert_eq!(bst.height(), 3);
    }

    #[test]
    fn empty_tree() {
        let bst = BoxBst::new();
        assert!(bst.is_empty());
        assert_eq!(bst.len(), 0);
        assert_eq!(bst.min(), None);
        assert!(bst.in_order().is_empty());
        assert_eq!(bst.height(), 0);
        assert!(!bst.contains(0));
    }

    #[test]
    fn insert_reports_new_values() {
        let mut bst = BoxBst::new();
        assert!(bst.insert(10));
        assert!(bst.insert(-10));
        assert!(!bst.insert(10));
        assert!(!bst.is_empty());
        assert_eq!(bst.len(), 2);
        assert_eq!(bst.in_order(), vec![-10, 10]);
    }

    #[test]
    fn sorted_input_makes_a_chain() {
        let values: Vec<i32> = (0..2_000).collect();
        let bst = build(&values);
        assert_eq!(bst.height(), 2_000);
        assert_eq!(bst.in_order(), values);
        assert!(bst.contains(1_999));
        assert_eq!(bst.min(), Some(0));
    }
}
