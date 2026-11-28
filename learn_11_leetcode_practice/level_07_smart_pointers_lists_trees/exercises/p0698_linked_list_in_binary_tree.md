# Problem 0698: Linked List in Binary Tree

- **Difficulty:** Medium
- **Topics:** matching a borrowed `Box` list (`Option<&ListNode>`) against an `Rc<RefCell<TreeNode>>` tree, two nested recursions
- **Source:** [LeetCode 1367. Linked List in Binary Tree](https://leetcode.com/problems/linked-list-in-binary-tree/)

## Problem

Given a linked list `head` and a binary tree `root`, return `true` if the list's values appear,
in order, along some **downward path** of the tree: the path starts at any node and repeatedly
moves to a left or right child.

Implement
`Solution::is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool`.

## Examples

```text
tree = [1, 4, 4, null, 2, 2, null, 1, null, 6, 8, null, null, null, null, 1, 3]

Input:  head = [4, 2, 8], root = tree
Output: true

Input:  head = [1, 4, 2, 6], root = tree
Output: true

Input:  head = [1, 4, 2, 6, 8], root = tree
Output: false
```

## Constraints

- The tree has `1` to `2500` nodes, the list `1` to `100` nodes; `1 <= val <= 100`.

## Hints

1. Write `matches(list, node) -> bool`: does the list match a downward path **starting exactly
   at `node`**? An exhausted list matches; a missing node does not; otherwise compare values and
   continue with `list.next` on either child.
2. The answer is whether `matches` holds at **some** node: try the root, then search both
   subtrees.
3. Walk the list with `Option<&ListNode>` (`head.as_deref()`, `node.next.as_deref()`), so it is
   never moved or copied.

## Starter code

Write your code in [`src/practice/p0698_linked_list_in_binary_tree.rs`](../src/practice/p0698_linked_list_in_binary_tree.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0698
```

```rust
use crate::common::{ListNode, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
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
    use crate::common::{list, parse_tree};

    const TREE: &str = "[1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]";

    #[test]
    fn example_one() {
        assert!(Solution::is_sub_path(list(&[4, 2, 8]), parse_tree(TREE)));
    }

    #[test]
    fn example_two() {
        assert!(Solution::is_sub_path(list(&[1, 4, 2, 6]), parse_tree(TREE)));
    }

    #[test]
    fn example_three() {
        assert!(!Solution::is_sub_path(
            list(&[1, 4, 2, 6, 8]),
            parse_tree(TREE)
        ));
    }

    #[test]
    fn path_must_go_downward_and_be_contiguous() {
        let root = parse_tree("[1,2,3]");
        assert!(!Solution::is_sub_path(list(&[2, 1, 3]), root.clone()));
        assert!(!Solution::is_sub_path(list(&[1, 3, 2]), root.clone()));
        assert!(Solution::is_sub_path(list(&[3]), root.clone()));
        assert!(Solution::is_sub_path(list(&[1, 3]), root));
    }

    #[test]
    fn restart_after_a_partial_match() {
        // The path 1 -> 1 -> 2 exists, but the first 1 -> 1 -> 1 branch only matches partially.
        let root = parse_tree("[1,1,null,1,2]");
        assert!(Solution::is_sub_path(list(&[1, 1, 2]), root.clone()));
        assert!(!Solution::is_sub_path(list(&[1, 1, 1, 1]), root));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** an outer depth-first search chooses the starting node; an inner recursion checks the
list from there. Both walk borrowed data only.

**Complexity:** `O(n * m)` time in the worst case for `n` tree nodes and a list of length `m`;
`O(h)` stack space.

**Rust concepts:**
- `Option<&ListNode>` is `Copy`, so the same list position can be passed to both children.
- `Box` (exclusive ownership) and `Rc<RefCell<..>>` (shared ownership) are read in the same way
  once you hold a reference: `list.val` and `node.borrow().val`.

```rust
use crate::common::{ListNode, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    fn matches(list: Option<&ListNode>, node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(list) = list else {
            return true;
        };
        let Some(node) = node else {
            return false;
        };
        let node = node.borrow();
        let rest = list.next.as_deref();
        node.val == list.val
            && (Self::matches(rest, &node.left) || Self::matches(rest, &node.right))
    }

    fn search(list: &ListNode, node: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(rc) = node else {
            return false;
        };
        if Self::matches(Some(list), node) {
            return true;
        }
        let current = rc.borrow();
        Self::search(list, &current.left) || Self::search(list, &current.right)
    }

    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match head.as_deref() {
            None => true,
            Some(list) => Self::search(list, &root),
        }
    }
}
```

Full tested solution: [`src/solutions/p0698_linked_list_in_binary_tree.rs`](../src/solutions/p0698_linked_list_in_binary_tree.rs)

</details>
