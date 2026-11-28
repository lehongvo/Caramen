# Problem 0669: Minimum Depth of Binary Tree

- **Difficulty:** Easy
- **Topics:** breadth-first search with `VecDeque<(Rc<RefCell<TreeNode>>, i32)>`, leaf detection
- **Source:** [LeetCode 111. Minimum Depth of Binary Tree](https://leetcode.com/problems/minimum-depth-of-binary-tree/)

## Problem

Return the number of nodes on the **shortest** path from the root down to a **leaf** (a node
with no children). The empty tree has minimum depth `0`.

Careful: a node with only one child is not a leaf, so the path must continue through that
child.

Implement `Solution::min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32`.

## Examples

```text
Input:  root = [3, 9, 20, null, null, 15, 7]
Output: 2              // 3 -> 9

Input:  root = [2, null, 3, null, 4, null, 5, null, 6]
Output: 5              // the only leaf is 6
```

## Constraints

- `0 <= n <= 10^5`, `-1000 <= val <= 1000`

## Hints

1. Breadth-first search visits nodes level by level, so the **first** leaf it meets is on the
   shortest path. No need to explore deeper levels.
2. Store `(node, depth)` pairs in a `VecDeque`; `pop_front` and `push_back` make it a queue.

## Starter code

Write your code in [`src/practice/p0669_minimum_depth_of_binary_tree.rs`](../src/practice/p0669_minimum_depth_of_binary_tree.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0669
```

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
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
    use crate::common::{parse_tree, tree};

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::min_depth(parse_tree("[3,9,20,null,null,15,7]")),
            2
        );
    }

    #[test]
    fn one_sided_chain() {
        assert_eq!(
            Solution::min_depth(parse_tree("[2,null,3,null,4,null,5,null,6]")),
            5
        );
    }

    #[test]
    fn empty_and_single() {
        assert_eq!(Solution::min_depth(None), 0);
        assert_eq!(Solution::min_depth(parse_tree("[1]")), 1);
        assert_eq!(Solution::min_depth(parse_tree("[1,2]")), 2);
    }

    #[test]
    fn big_tree_with_a_shallow_leaf() {
        let mut values: Vec<Option<i32>> = (1..=4095).map(Some).collect();
        // Remove both children of node 3 (indices 5 and 6), making it a leaf at depth 2.
        values[5] = None;
        values[6] = None;
        assert_eq!(Solution::min_depth(tree(&values)), 2);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** BFS from the root, carrying each node's depth. Return as soon as a node without
children is dequeued.

**Complexity:** `O(n)` time in the worst case, `O(w)` extra space for the widest level `w`; it
often stops early.

**Rust concepts:**
- The queue owns `Rc` clones of the nodes; cloning is a counter increment.
- `node.left.is_none() && node.right.is_none()` tests for a leaf without moving anything.

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, i32)> =
            root.into_iter().map(|node| (node, 1)).collect();
        while let Some((node, depth)) = queue.pop_front() {
            let node = node.borrow();
            if node.left.is_none() && node.right.is_none() {
                return depth;
            }
            for child in [&node.left, &node.right].into_iter().flatten() {
                queue.push_back((Rc::clone(child), depth + 1));
            }
        }
        0
    }
}
```

Full tested solution: [`src/solutions/p0669_minimum_depth_of_binary_tree.rs`](../src/solutions/p0669_minimum_depth_of_binary_tree.rs)

</details>
