// ==================================================================
// PRACTICE 0624: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0624_tree_with_parent_pointers.md
// Run the tests:          cargo test practice::p0624
// ==================================================================

//! # Problem 0624: Tree with Parent Pointers
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Weak<T>` parent links, `Rc<T>` child links, `RefCell<Weak<T>>`, `strong_count`/`weak_count`
//! - **Source:** Original · [The Rust Book 15.6](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)
//!
//! ## Problem
//!
//! In a tree where children also know their parent, the links go both ways. If both directions
//! used `Rc`, every parent/child pair would be a reference cycle and nothing would ever be freed.
//! The rule from the Book: **parents own their children** (`Rc`), **children only refer to their
//! parent** (`Weak`). Dropping a parent then frees it, and its children's parent links simply stop
//! upgrading.
//!
//! The node type is given (it is the Book's):
//!
//! ```text
//! pub struct Node {
//!     pub value: i32,
//!     pub parent: RefCell<Weak<Node>>,
//!     pub children: RefCell<Vec<Rc<Node>>>,
//! }
//! ```
//!
//! Implement:
//!
//! - `new_node(value: i32) -> Rc<Node>`: no parent, no children;
//! - `add_child(parent: &Rc<Node>, child: &Rc<Node>)`: the parent stores a strong handle to the
//!   child and the child's parent becomes a weak handle to `parent`;
//! - `parent_value(node: &Node) -> Option<i32>`: the parent's value, if the parent is still alive;
//! - `path_to_root(node: &Rc<Node>) -> Vec<i32>`: the values from `node` up to the root
//!   (both included);
//! - `depth(node: &Rc<Node>) -> usize`: `0` for a root, `1` for its children, ...;
//! - `subtree_sum(node: &Node) -> i64`: the sum of the values in `node`'s subtree.
//!
//! ## Examples
//!
//! ```text
//! let leaf = new_node(3);
//! let branch = new_node(5);
//! add_child(&branch, &leaf);
//! parent_value(&leaf)  -> Some(5)
//! Rc::strong_count(&leaf) == 2, Rc::weak_count(&branch) == 1
//! path_to_root(&leaf)  -> [3, 5], depth(&leaf) -> 1
//! subtree_sum(&branch) -> 8
//! drop(branch);
//! parent_value(&leaf)  -> None     // the parent was freed
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` nodes; depth at most `1000`.
//!
//! ## Hints
//!
//! 1. `*child.parent.borrow_mut() = Rc::downgrade(parent);`
//! 2. Walking up: `node.parent.borrow().upgrade()` gives `Option<Rc<Node>>`. Loop until it is
//!    `None`, so a deep tree does not recurse.
//! 3. For `subtree_sum`, an explicit stack of `Rc<Node>` avoids deep recursion as well.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<Vec<Rc<Node>>>,
}

pub fn new_node(value: i32) -> Rc<Node> {
    todo!("Write your solution here")
}

pub fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
    todo!("Write your solution here")
}

pub fn parent_value(node: &Node) -> Option<i32> {
    todo!("Write your solution here")
}

pub fn path_to_root(node: &Rc<Node>) -> Vec<i32> {
    todo!("Write your solution here")
}

pub fn depth(node: &Rc<Node>) -> usize {
    todo!("Write your solution here")
}

pub fn subtree_sum(node: &Node) -> i64 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book_example() {
        let leaf = new_node(3);
        assert_eq!(parent_value(&leaf), None);
        let branch = new_node(5);
        add_child(&branch, &leaf);
        assert_eq!(parent_value(&leaf), Some(5));
        assert_eq!(Rc::strong_count(&leaf), 2);
        assert_eq!(Rc::weak_count(&branch), 1);
        assert_eq!(Rc::strong_count(&branch), 1);
        drop(branch);
        assert_eq!(parent_value(&leaf), None);
        assert_eq!(Rc::strong_count(&leaf), 1);
    }

    #[test]
    fn paths_and_depths() {
        let root = new_node(1);
        let mid = new_node(2);
        let low = new_node(3);
        add_child(&root, &mid);
        add_child(&mid, &low);
        assert_eq!(path_to_root(&low), vec![3, 2, 1]);
        assert_eq!(depth(&low), 2);
        assert_eq!(depth(&root), 0);
        assert_eq!(path_to_root(&root), vec![1]);
    }

    #[test]
    fn subtree_sums() {
        let root = new_node(10);
        let a = new_node(-4);
        let b = new_node(6);
        let c = new_node(1);
        add_child(&root, &a);
        add_child(&root, &b);
        add_child(&b, &c);
        assert_eq!(subtree_sum(&root), 13);
        assert_eq!(subtree_sum(&b), 7);
        assert_eq!(subtree_sum(&c), 1);
        assert_eq!(root.children.borrow().len(), 2);
    }

    #[test]
    fn deep_chain_without_recursion() {
        let root = new_node(0);
        let mut last = Rc::clone(&root);
        for value in 1..1_000 {
            let node = new_node(value);
            add_child(&last, &node);
            last = node;
        }
        assert_eq!(depth(&last), 999);
        assert_eq!(path_to_root(&last)[999], 0);
        assert_eq!(subtree_sum(&root), (0..1_000).sum::<i64>());
    }
}
