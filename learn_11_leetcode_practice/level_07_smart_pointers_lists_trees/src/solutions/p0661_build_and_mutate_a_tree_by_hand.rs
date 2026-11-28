//! # Problem 0661: Build and Mutate a Tree by Hand
//!
//! - **Difficulty:** Easy
//! - **Topics:** LeetCode's `TreeNode`, `Rc<RefCell<TreeNode>>`, `borrow`/`borrow_mut`, shared handles into a tree
//! - **Source:** Original · [The Rust Book 15.5](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html) · [`std::rc::Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html)
//!
//! ## Problem
//!
//! The tree problems of this level use LeetCode's node type (in `crate::common`):
//!
//! ```text
//! pub struct TreeNode {
//!     pub val: i32,
//!     pub left: Option<Rc<RefCell<TreeNode>>>,
//!     pub right: Option<Rc<RefCell<TreeNode>>>,
//! }
//! ```
//!
//! A tree is an `Option<Rc<RefCell<TreeNode>>>`; `None` is the empty tree. `Rc` lets the caller
//! keep extra handles to any node, and `RefCell` lets anyone holding a handle change the node.
//! Get comfortable with this type before the LeetCode problems:
//!
//! - `leaf(val: i32) -> Option<Rc<RefCell<TreeNode>>>`: a single node;
//! - `node(val, left, right) -> Option<Rc<RefCell<TreeNode>>>`: a node with the given subtrees;
//! - `count_nodes(root: &Option<Rc<RefCell<TreeNode>>>) -> usize`;
//! - `add_to_all(root: &Option<Rc<RefCell<TreeNode>>>, delta: i32)`: add `delta` to every value,
//!   **in place**. Note the shared reference: `RefCell` makes this possible.
//!
//! ## Examples
//!
//! ```text
//! let root = node(1, node(2, None, leaf(4)), leaf(3));
//! //       1
//! //      / \
//! //     2   3
//! //      \
//! //       4
//! tree_to_string(&root)  -> "[1,2,3,null,4]"
//! count_nodes(&root)     -> 4
//! add_to_all(&root, 10);
//! tree_to_string(&root)  -> "[11,12,13,null,14]"
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` nodes; depth at most `1000`.
//!
//! ## Hints
//!
//! 1. `Some(Rc::new(RefCell::new(TreeNode { val, left, right })))` builds a node.
//! 2. To read a node: `let node = rc.borrow();` then `node.val`, `node.left`, ...; to change it:
//!    `rc.borrow_mut().val += delta;`.
//! 3. Recurse on `&node.left` and `&node.right`: they are `&Option<Rc<RefCell<TreeNode>>>` too.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** build nodes with `Rc::new(RefCell::new(..))`; walk the tree recursively through
//! `borrow()` guards. `add_to_all` updates the value with a short `borrow_mut()` and then recurses
//! with a shared `borrow()`, so no two guards on the same node overlap.
//!
//! **Complexity:** `count_nodes` and `add_to_all` are `O(n)` time and `O(h)` stack for height `h`.
//!
//! **Rust concepts:**
//! - A handle cloned from the tree (`Rc::clone`) sees every change made through the tree, because
//!   both point at the same `RefCell`.
//! - `if let Some(node) = root` on a `&Option<Rc<..>>` binds `node: &Rc<RefCell<TreeNode>>`.
//! - A `borrow_mut()` guard used in one statement is released at the semicolon.
// @solution-end

use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn leaf(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    // @solution-start
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
    // @solution-end
}

pub fn node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    // @solution-start
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    // @solution-end
}

pub fn count_nodes(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
    // @solution-start
    match root {
        None => 0,
        Some(node) => {
            let node = node.borrow();
            1 + count_nodes(&node.left) + count_nodes(&node.right)
        }
    }
    // @solution-end
}

pub fn add_to_all(root: &Option<Rc<RefCell<TreeNode>>>, delta: i32) {
    // @solution-start
    if let Some(node) = root {
        node.borrow_mut().val += delta;
        let node = node.borrow();
        add_to_all(&node.left, delta);
        add_to_all(&node.right, delta);
    }
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{find_node, parse_tree, tree_to_string};

    #[test]
    fn build_the_example_tree() {
        let root = node(1, node(2, None, leaf(4)), leaf(3));
        assert_eq!(tree_to_string(&root), "[1,2,3,null,4]");
        assert_eq!(root, parse_tree("[1,2,3,null,4]"));
        assert_eq!(count_nodes(&root), 4);
    }

    #[test]
    fn add_to_all_changes_values_in_place() {
        let root = node(1, node(2, None, leaf(4)), leaf(3));
        add_to_all(&root, 10);
        assert_eq!(tree_to_string(&root), "[11,12,13,null,14]");
        add_to_all(&root, -11);
        assert_eq!(tree_to_string(&root), "[0,1,2,null,3]");
    }

    #[test]
    fn a_kept_handle_sees_the_changes() {
        let root = parse_tree("[5,3,8]");
        let three = find_node(&root, 3).unwrap();
        assert_eq!(Rc::strong_count(&three), 2);
        add_to_all(&root, 1);
        assert_eq!(three.borrow().val, 4);
    }

    #[test]
    fn empty_tree_and_single_leaf() {
        assert_eq!(count_nodes(&None), 0);
        add_to_all(&None, 3);
        let one = leaf(7);
        add_to_all(&one, 3);
        assert_eq!(tree_to_string(&one), "[10]");
        assert_eq!(count_nodes(&one), 1);
    }

    #[test]
    fn deep_left_chain() {
        let mut root = None;
        for val in (0..1_000).rev() {
            root = node(val, root, None);
        }
        assert_eq!(count_nodes(&root), 1_000);
        add_to_all(&root, 1);
        assert_eq!(root.as_ref().unwrap().borrow().val, 1);
    }
}
