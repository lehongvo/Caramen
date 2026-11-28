# Problem 0688: Validate Binary Search Tree

- **Difficulty:** Medium
- **Topics:** recursion with `Option<i32>` bounds, avoiding sentinel values, `is_none_or`-style checks
- **Source:** [LeetCode 98. Validate Binary Search Tree](https://leetcode.com/problems/validate-binary-search-tree/)

## Problem

Decide whether a binary tree is a valid binary search tree: for **every** node, all values in
its left subtree are strictly smaller than the node's value, and all values in its right subtree
are strictly larger. Both subtrees must be BSTs too.

Implement `Solution::is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool`.

## Examples

```text
Input:  root = [2, 1, 3]
Output: true

Input:  root = [5, 1, 4, null, null, 3, 6]
Output: false          // 4 is in the right subtree of 5 but smaller than 5

Input:  root = [5, 4, 6, null, null, 3, 7]
Output: false          // 3 is below 6 (fine locally) but also right of 5
```

## Constraints

- `1 <= n <= 10^4`, `-2^31 <= val <= 2^31 - 1`

## Hints

1. Comparing a node only with its direct children is not enough (third example).
2. Pass down the open interval `(low, high)` the node's value must fall into. Going left, the
   upper bound becomes the node's value; going right, the lower bound does.
3. Values can be `i32::MIN` or `i32::MAX`, so do not use them as "no bound". Use
   `Option<i32>`: `None` means unbounded.

## Starter code

Write your code in [`src/practice/p0688_validate_binary_search_tree.rs`](../src/practice/p0688_validate_binary_search_tree.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0688
```

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
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
    fn valid_small_tree() {
        assert!(Solution::is_valid_bst(parse_tree("[2,1,3]")));
    }

    #[test]
    fn invalid_direct_child() {
        assert!(!Solution::is_valid_bst(parse_tree("[5,1,4,null,null,3,6]")));
    }

    #[test]
    fn invalid_deep_descendant() {
        assert!(!Solution::is_valid_bst(parse_tree("[5,4,6,null,null,3,7]")));
    }

    #[test]
    fn duplicates_are_not_allowed() {
        assert!(!Solution::is_valid_bst(parse_tree("[2,2,2]")));
        assert!(!Solution::is_valid_bst(parse_tree("[1,null,1]")));
    }

    #[test]
    fn extreme_values() {
        assert!(Solution::is_valid_bst(parse_tree(
            "[-2147483648,null,2147483647]"
        )));
        assert!(Solution::is_valid_bst(parse_tree("[2147483647]")));
        assert!(!Solution::is_valid_bst(parse_tree(
            "[-2147483648,-2147483648]"
        )));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** recursive check with optional bounds. Each node must satisfy `low < val < high` (where
a missing bound is always satisfied), and its children are checked with the tightened bounds.

**Complexity:** `O(n)` time, `O(h)` stack space.

**Rust concepts:**
- `Option<i32>` bounds remove the classic sentinel bug (a node holding `i32::MIN`).
- `low.is_none_or(|low| val > low)` (stable since Rust 1.82) reads "no bound, or the bound
  holds". On older compilers, `low.map_or(true, |low| val > low)` says the same thing.

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    fn valid(node: &Option<Rc<RefCell<TreeNode>>>, low: Option<i32>, high: Option<i32>) -> bool {
        let Some(node) = node else {
            return true;
        };
        let node = node.borrow();
        let val = node.val;
        low.is_none_or(|low| val > low)
            && high.is_none_or(|high| val < high)
            && Self::valid(&node.left, low, Some(val))
            && Self::valid(&node.right, Some(val), high)
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::valid(&root, None, None)
    }
}
```

Full tested solution: [`src/solutions/p0688_validate_binary_search_tree.rs`](../src/solutions/p0688_validate_binary_search_tree.rs)

</details>
