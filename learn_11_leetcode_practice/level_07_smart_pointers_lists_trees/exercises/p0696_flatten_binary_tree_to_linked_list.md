# Problem 0696: Flatten Binary Tree to Linked List

- **Difficulty:** Medium
- **Topics:** `&mut Option<Rc<RefCell<TreeNode>>>` parameter, collecting `Rc` handles in pre-order, re-linking with `borrow_mut`, `windows(2)`
- **Source:** [LeetCode 114. Flatten Binary Tree to Linked List](https://leetcode.com/problems/flatten-binary-tree-to-linked-list/)

## Problem

Flatten a binary tree **in place** into a "linked list" made of the same `TreeNode`s: every
node's `left` becomes `None`, and its `right` points to the next node in **pre-order**
(node, left subtree, right subtree). The root stays the first node.

Implement `Solution::flatten(root: &mut Option<Rc<RefCell<TreeNode>>>)`. It returns nothing;
the tree behind `root` is rearranged.

## Examples

```text
Input:  root = [1, 2, 5, 3, 4, null, 6]
After:  root = [1, null, 2, null, 3, null, 4, null, 5, null, 6]

Input:  root = []
After:  root = []

Input:  root = [0]
After:  root = [0]
```

## Constraints

- `0 <= n <= 2000`, `-100 <= val <= 100`

## Hints

1. First collect handles (`Rc::clone`) to all nodes in pre-order, using a stack: push the right
   child before the left one.
2. Then walk the collected handles in pairs (`nodes.windows(2)`): set `left = None` and
   `right = Some(next)` on the first of each pair. Do not forget the last node.
3. Follow-up: there is an `O(1)` extra space version (for each node, splice its left subtree in
   front of its right subtree).

## Starter code

Write your code in [`src/practice/p0696_flatten_binary_tree_to_linked_list.rs`](../src/practice/p0696_flatten_binary_tree_to_linked_list.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0696
```

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        todo!("Write your solution here")
    }
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{parse_tree, tree_to_string};

    fn flattened(text: &str) -> String {
        let mut root = parse_tree(text);
        Solution::flatten(&mut root);
        tree_to_string(&root)
    }

    #[test]
    fn example_one() {
        assert_eq!(
            flattened("[1,2,5,3,4,null,6]"),
            "[1,null,2,null,3,null,4,null,5,null,6]"
        );
    }

    #[test]
    fn empty_and_single() {
        assert_eq!(flattened("[]"), "[]");
        assert_eq!(flattened("[0]"), "[0]");
    }

    #[test]
    fn left_chain_becomes_right_chain() {
        assert_eq!(flattened("[1,2,null,3]"), "[1,null,2,null,3]");
        assert_eq!(flattened("[1,null,2,null,3]"), "[1,null,2,null,3]");
    }

    #[test]
    fn same_nodes_are_reused() {
        let mut root = parse_tree("[1,2,3]");
        let three = root.as_ref().unwrap().borrow().right.clone().unwrap();
        Solution::flatten(&mut root);
        let two = root.as_ref().unwrap().borrow().right.clone().unwrap();
        let after = two.borrow().right.clone().unwrap();
        assert!(Rc::ptr_eq(&after, &three));
        assert_eq!(tree_to_string(&root), "[1,null,2,null,3]");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** separate "find the order" from "change the links". While collecting, nothing is
modified, so the traversal is simple. While re-linking, every node is kept alive by the `nodes`
vector, so overwriting `right` never frees a node that is still needed.

**Complexity:** `O(n)` time, `O(n)` extra space for the handles.

**Rust concepts:**
- Holding `Rc` handles in a `Vec` is what makes the two-phase approach safe: dropping a link
  only decrements a count.
- `windows(2)` yields overlapping pairs `&[a, b]` without index arithmetic.
- The parameter is `&mut Option<..>`, but the root node itself does not change, so only the
  nodes are modified (through their `RefCell`s).

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut nodes: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = root.iter().cloned().collect();
        while let Some(node) = stack.pop() {
            {
                let current = node.borrow();
                stack.extend(current.right.clone());
                stack.extend(current.left.clone());
            }
            nodes.push(node);
        }
        for pair in nodes.windows(2) {
            let mut first = pair[0].borrow_mut();
            first.left = None;
            first.right = Some(Rc::clone(&pair[1]));
        }
        if let Some(last) = nodes.last() {
            let mut last = last.borrow_mut();
            last.left = None;
            last.right = None;
        }
    }
}
```

Full tested solution: [`src/solutions/p0696_flatten_binary_tree_to_linked_list.rs`](../src/solutions/p0696_flatten_binary_tree_to_linked_list.rs)

</details>
