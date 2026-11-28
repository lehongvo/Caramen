# Problem 0674: Diameter of Binary Tree

- **Difficulty:** Easy
- **Topics:** post-order recursion that returns one value and updates another through `&mut`
- **Source:** [LeetCode 543. Diameter of Binary Tree](https://leetcode.com/problems/diameter-of-binary-tree/)

## Problem

The *diameter* of a binary tree is the number of **edges** on the longest path between any two
nodes. The path does not have to pass through the root. Return the diameter.

Implement `Solution::diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32`.

## Examples

```text
Input:  root = [1, 2, 3, 4, 5]
Output: 3              // 4 -> 2 -> 1 -> 3 (or 5 -> 2 -> 1 -> 3)

Input:  root = [1, 2]
Output: 1
```

## Constraints

- `1 <= n <= 10^4`, `-100 <= val <= 100`

## Hints

1. The longest path that turns around at a node `x` has `height(x.left) + height(x.right)`
   edges, where `height` counts nodes on the longest downward path.
2. Compute heights bottom-up; while doing so, update a `best: &mut i32` with that sum at every
   node.

## Starter code

Write your code in [`src/practice/p0674_diameter_of_binary_tree.rs`](../src/practice/p0674_diameter_of_binary_tree.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0674
```

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
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
    fn example_one() {
        assert_eq!(
            Solution::diameter_of_binary_tree(parse_tree("[1,2,3,4,5]")),
            3
        );
    }

    #[test]
    fn two_nodes_and_one_node() {
        assert_eq!(Solution::diameter_of_binary_tree(parse_tree("[1,2]")), 1);
        assert_eq!(Solution::diameter_of_binary_tree(parse_tree("[1]")), 0);
    }

    #[test]
    fn longest_path_avoids_the_root() {
        // Node 2 has two long arms; the root only adds a short branch.
        let root = parse_tree("[1,2,3,4,5,null,null,6,null,null,7,8,null,null,9]");
        assert_eq!(Solution::diameter_of_binary_tree(root), 6);
    }

    #[test]
    fn long_chain() {
        let chain = format!(
            "[{}]",
            (0..1_000)
                .map(|v| (v % 100).to_string())
                .collect::<Vec<_>>()
                .join(",null,")
        );
        assert_eq!(Solution::diameter_of_binary_tree(parse_tree(&chain)), 999);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** one post-order pass. The helper returns the height of a subtree and, as a side effect,
records the best "left height + right height" seen at any node.

**Complexity:** `O(n)` time, `O(h)` stack space.

**Rust concepts:**
- Returning one value and accumulating another through `&mut i32` keeps the helper simple; a
  tuple return `(height, best)` would work too.
- The `borrow()` guard for a node stays alive while its children are visited; only shared
  borrows are involved.

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    fn height(node: &Option<Rc<RefCell<TreeNode>>>, best: &mut i32) -> i32 {
        let Some(node) = node else {
            return 0;
        };
        let node = node.borrow();
        let left = Self::height(&node.left, best);
        let right = Self::height(&node.right, best);
        *best = (*best).max(left + right);
        1 + left.max(right)
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut best = 0;
        Self::height(&root, &mut best);
        best
    }
}
```

Full tested solution: [`src/solutions/p0674_diameter_of_binary_tree.rs`](../src/solutions/p0674_diameter_of_binary_tree.rs)

</details>
