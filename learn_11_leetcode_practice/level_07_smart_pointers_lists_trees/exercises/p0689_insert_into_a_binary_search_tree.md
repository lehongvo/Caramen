# Problem 0689: Insert into a Binary Search Tree

- **Difficulty:** Medium
- **Topics:** walking a BST with `Rc` handles, writing into an empty child slot through `borrow_mut`
- **Source:** [LeetCode 701. Insert into a Binary Search Tree](https://leetcode.com/problems/insert-into-a-binary-search-tree/)

## Problem

Insert `val` into a binary search tree and return the root of the resulting tree. `val` is
guaranteed not to be in the tree yet. Several valid trees exist; any valid BST that contains
the old values plus `val` is accepted. (The simplest: add `val` as a new leaf.)

Implement
`Solution::insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>>`.

## Examples

```text
Input:  root = [4, 2, 7, 1, 3], val = 5
Output: [4, 2, 7, 1, 3, 5]

Input:  root = [40, 20, 60, 10, 30, 50, 70], val = 25
Output: [40, 20, 60, 10, 30, 50, 70, null, null, 25]

Input:  root = [], val = 5
Output: [5]
```

## Constraints

- `0 <= n <= 10^4`, `-10^8 <= val <= 10^8`, all values distinct, `val` is not in the tree.

## Hints

1. Walk down as in a search (problem 0678). The walk ends at a node whose child on the chosen
   side is `None`: that empty slot is where the new leaf goes.
2. Inside `let mut node = current.borrow_mut();`, pick the slot with
   `let slot = if val < node.val { &mut node.left } else { &mut node.right };`, then either
   clone the child to continue, or write `*slot = Some(new_leaf)` and stop.
3. An empty tree simply becomes the new leaf.

## Starter code

Write your code in [`src/practice/p0689_insert_into_a_binary_search_tree.rs`](../src/practice/p0689_insert_into_a_binary_search_tree.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0689
```

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
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
    use crate::common::{parse_tree, tree_to_vec};

    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>) {
        if let Some(node) = node {
            let node = node.borrow();
            inorder(&node.left, out);
            out.push(node.val);
            inorder(&node.right, out);
        }
    }

    /// Any valid answer is accepted: the in-order traversal must be the sorted values plus `val`.
    fn assert_valid_insert(text: &str, val: i32) {
        let mut expected: Vec<i32> = tree_to_vec(&parse_tree(text))
            .into_iter()
            .flatten()
            .collect();
        expected.push(val);
        expected.sort();
        let result = Solution::insert_into_bst(parse_tree(text), val);
        let mut values = Vec::new();
        inorder(&result, &mut values);
        assert_eq!(values, expected);
    }

    #[test]
    fn example_one() {
        assert_valid_insert("[4,2,7,1,3]", 5);
    }

    #[test]
    fn example_two() {
        assert_valid_insert("[40,20,60,10,30,50,70]", 25);
    }

    #[test]
    fn empty_tree() {
        let result = Solution::insert_into_bst(None, 5);
        assert_eq!(tree_to_vec(&result), vec![Some(5)]);
    }

    #[test]
    fn new_minimum_and_maximum() {
        assert_valid_insert("[4,2,7,1,3]", -100_000_000);
        assert_valid_insert("[4,2,7,1,3]", 100_000_000);
    }

    #[test]
    fn many_inserts_keep_a_valid_bst() {
        let mut root = None;
        let values: Vec<i32> = (0..1_000).map(|i| (i * 7_919) % 1_009).collect();
        for &v in &values {
            root = Solution::insert_into_bst(root, v);
        }
        let mut sorted = values.clone();
        sorted.sort();
        let mut out = Vec::new();
        inorder(&root, &mut out);
        assert_eq!(out, sorted);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** iterative descent with an `Rc` cursor. Each step borrows the current node mutably just
long enough to look at (or fill) the slot on the correct side.

**Complexity:** `O(h)` time, `O(1)` extra space.

**Rust concepts:**
- In `match slot { Some(child) => ..., None => *slot = ... }` the `None` arm holds no binding,
  so writing through `slot` is allowed.
- The `borrow_mut()` guard lives inside a block that ends before the cursor moves on, so it is
  never held across iterations.
- The original `root` handle is returned; the tree was changed in place.

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let leaf = Rc::new(RefCell::new(TreeNode::new(val)));
        let Some(mut current) = root.clone() else {
            return Some(leaf);
        };
        loop {
            let next = {
                let mut node = current.borrow_mut();
                let slot = if val < node.val {
                    &mut node.left
                } else {
                    &mut node.right
                };
                match slot {
                    Some(child) => Rc::clone(child),
                    None => {
                        *slot = Some(leaf);
                        break;
                    }
                }
            };
            current = next;
        }
        root
    }
}
```

Full tested solution: [`src/solutions/p0689_insert_into_a_binary_search_tree.rs`](../src/solutions/p0689_insert_into_a_binary_search_tree.rs)

</details>
