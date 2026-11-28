# Problem 0662: Root Equals Sum of Children

- **Difficulty:** Easy
- **Topics:** `let ... else` on `Option<Rc<RefCell<TreeNode>>>`, `borrow()`, matching a pair of children
- **Source:** [LeetCode 2236. Root Equals Sum of Children](https://leetcode.com/problems/root-equals-sum-of-children/)

## Problem

A binary tree has exactly three nodes: a root, its left child and its right child. Return
`true` if the root's value equals the sum of its two children's values, and `false` otherwise.

Implement `Solution::check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool`.

## Examples

```text
Input:  root = [10, 4, 6]
Output: true           // 4 + 6 = 10

Input:  root = [5, 3, 1]
Output: false          // 3 + 1 = 4
```

## Constraints

- The tree has exactly `3` nodes; `-100 <= val <= 100`.

## Hints

1. `let Some(root) = root else { return false };` gives the `Rc`; `root.borrow()` gives a
   readable `TreeNode`.
2. Match on `(&node.left, &node.right)` and borrow each child to read its `val`.

## Starter code

Write your code in [`src/practice/p0662_root_equals_sum_of_children.rs`](../src/practice/p0662_root_equals_sum_of_children.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0662
```

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
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
    use crate::common::parse_tree;

    #[test]
    fn sum_matches() {
        assert!(Solution::check_tree(parse_tree("[10,4,6]")));
    }

    #[test]
    fn sum_does_not_match() {
        assert!(!Solution::check_tree(parse_tree("[5,3,1]")));
    }

    #[test]
    fn negative_and_zero_values() {
        assert!(Solution::check_tree(parse_tree("[-100,-50,-50]")));
        assert!(Solution::check_tree(parse_tree("[0,7,-7]")));
        assert!(!Solution::check_tree(parse_tree("[1,0,0]")));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** borrow the root, then both children, and compare the values. Any missing node means
the input is not the promised shape, so the answer is `false`.

**Complexity:** `O(1)` time and space.

**Rust concepts:**
- `Rc<RefCell<TreeNode>>` needs two steps to read a field: the `Rc` derefs to the `RefCell`, and
  `borrow()` returns a guard that derefs to the `TreeNode`.
- Matching on a tuple of references avoids moving the children out of the borrowed node.

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let Some(root) = root else {
            return false;
        };
        let node = root.borrow();
        match (&node.left, &node.right) {
            (Some(left), Some(right)) => node.val == left.borrow().val + right.borrow().val,
            _ => false,
        }
    }
}
```

Full tested solution: [`src/solutions/p0662_root_equals_sum_of_children.rs`](../src/solutions/p0662_root_equals_sum_of_children.rs)

</details>
