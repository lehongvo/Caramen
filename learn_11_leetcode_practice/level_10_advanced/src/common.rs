//! Shared types and test helpers for level 10 (the same definitions as level 7).
//!
//! [`ListNode`] and [`TreeNode`] are exactly the definitions LeetCode puts at the top of its Rust
//! templates (only `new` is made `pub` so other modules can call it). Because the types match,
//! any `impl Solution { ... }` block in this level can be pasted into LeetCode unchanged.
//!
//! The helper functions below build and inspect lists and trees; the tests use them so that
//! examples can be written in LeetCode's own notation:
//!
//! ```text
//! list(&[1, 2, 3])                      // 1 -> 2 -> 3
//! list_to_vec(&head)                    // [1, 2, 3]
//! tree(&[Some(1), None, Some(2)])       // 1 with a right child 2
//! parse_tree("[3,9,20,null,null,15,7]") // same format as LeetCode examples
//! tree_to_string(&root)                 // "[3,9,20,null,null,15,7]"
//! ```

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// LeetCode's singly-linked list node.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// LeetCode's binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Builds a linked list holding `values` in order. An empty slice gives `None`.
pub fn list(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in values.iter().rev() {
        head = Some(Box::new(ListNode { val, next: head }));
    }
    head
}

/// Collects the values of a list from front to back.
pub fn list_to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut values = Vec::new();
    let mut current = head.as_deref();
    while let Some(node) = current {
        values.push(node.val);
        current = node.next.as_deref();
    }
    values
}

/// Builds a binary tree from LeetCode's level-order format, where `None` marks a missing child.
///
/// Children are assigned left to right, level by level, and only real (non-`None`) nodes get
/// children. An empty slice, or a slice that starts with `None`, gives an empty tree.
pub fn tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    let mut items = values.iter();
    let root = match items.next() {
        Some(Some(val)) => Rc::new(RefCell::new(TreeNode::new(*val))),
        _ => return None,
    };
    let mut queue = VecDeque::from([Rc::clone(&root)]);
    while let Some(parent) = queue.pop_front() {
        for is_left in [true, false] {
            match items.next() {
                None => return Some(root),
                Some(None) => {}
                Some(Some(val)) => {
                    let child = Rc::new(RefCell::new(TreeNode::new(*val)));
                    queue.push_back(Rc::clone(&child));
                    let mut parent = parent.borrow_mut();
                    if is_left {
                        parent.left = Some(child);
                    } else {
                        parent.right = Some(child);
                    }
                }
            }
        }
    }
    Some(root)
}

/// Parses a tree written exactly like a LeetCode example, e.g. `"[3,9,20,null,null,15,7]"`.
///
/// Panics if a value is neither an integer nor `null` (it is only meant for tests).
pub fn parse_tree(text: &str) -> Option<Rc<RefCell<TreeNode>>> {
    let inner = text.trim().trim_start_matches('[').trim_end_matches(']');
    let values: Vec<Option<i32>> = inner
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(|item| match item {
            "null" => None,
            number => Some(
                number
                    .parse()
                    .expect("tree values must be integers or null"),
            ),
        })
        .collect();
    tree(&values)
}

/// Serializes a tree to LeetCode's level-order format, without trailing `None`s.
pub fn tree_to_vec(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut values = Vec::new();
    let mut queue = VecDeque::from([root.clone()]);
    while let Some(slot) = queue.pop_front() {
        match slot {
            None => values.push(None),
            Some(node) => {
                let node = node.borrow();
                values.push(Some(node.val));
                queue.push_back(node.left.clone());
                queue.push_back(node.right.clone());
            }
        }
    }
    while values.last() == Some(&None) {
        values.pop();
    }
    values
}

/// Formats a tree like a LeetCode example, e.g. `"[3,9,20,null,null,15,7]"`.
pub fn tree_to_string(root: &Option<Rc<RefCell<TreeNode>>>) -> String {
    let items: Vec<String> = tree_to_vec(root)
        .into_iter()
        .map(|value| value.map_or_else(|| "null".to_string(), |v| v.to_string()))
        .collect();
    format!("[{}]", items.join(","))
}

/// Returns a handle (a clone of the `Rc`) to the first node with value `val`, in pre-order.
pub fn find_node(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let node = root.as_ref()?;
    if node.borrow().val == val {
        return Some(Rc::clone(node));
    }
    let node = node.borrow();
    find_node(&node.left, val).or_else(|| find_node(&node.right, val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_round_trip() {
        assert_eq!(list_to_vec(&list(&[1, 2, 3])), vec![1, 2, 3]);
        assert_eq!(list(&[]), None);
    }

    #[test]
    fn tree_round_trip() {
        let text = "[3,9,20,null,null,15,7]";
        assert_eq!(tree_to_string(&parse_tree(text)), text);
        assert_eq!(tree_to_string(&parse_tree("[]")), "[]");
        let root = tree(&[Some(1), None, Some(2), Some(3)]);
        assert_eq!(tree_to_vec(&root), vec![Some(1), None, Some(2), Some(3)]);
    }

    #[test]
    fn find_node_returns_a_shared_handle() {
        let root = parse_tree("[1,2,3,4]");
        let four = find_node(&root, 4).unwrap();
        four.borrow_mut().val = 40;
        assert_eq!(tree_to_string(&root), "[1,2,3,40]");
        assert!(find_node(&root, 99).is_none());
    }
}
