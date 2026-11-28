# Problem 0684: Binary Tree Zigzag Level Order Traversal

- **Difficulty:** Medium
- **Topics:** level-by-level BFS, reversing every other row, a flipping `bool`
- **Source:** [LeetCode 103. Binary Tree Zigzag Level Order Traversal](https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/)

## Problem

Return the values of a binary tree level by level, alternating the direction: the first level
left to right, the second right to left, the third left to right again, and so on.

Implement `Solution::zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>`.

## Examples

```text
Input:  root = [3, 9, 20, null, null, 15, 7]
Output: [[3], [20, 9], [15, 7]]

Input:  root = [1]
Output: [[1]]

Input:  root = []
Output: []
```

## Constraints

- `0 <= n <= 2000`, `-100 <= val <= 100`

## Hints

1. Collect the levels exactly as in problem 0682.
2. Before storing a row, reverse it when the level index is odd. Changing the order in which
   children are visited is not needed.

## Starter code

Write your code in [`src/practice/p0684_binary_tree_zigzag_level_order_traversal.rs`](../src/practice/p0684_binary_tree_zigzag_level_order_traversal.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0684
```

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
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
        let root = parse_tree("[3,9,20,null,null,15,7]");
        assert_eq!(
            Solution::zigzag_level_order(root),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }

    #[test]
    fn single_and_empty() {
        assert_eq!(
            Solution::zigzag_level_order(parse_tree("[1]")),
            vec![vec![1]]
        );
        assert!(Solution::zigzag_level_order(None).is_empty());
    }

    #[test]
    fn four_levels() {
        let values: Vec<Option<i32>> = (1..=15).map(Some).collect();
        assert_eq!(
            Solution::zigzag_level_order(tree(&values)),
            vec![
                vec![1],
                vec![3, 2],
                vec![4, 5, 6, 7],
                vec![15, 14, 13, 12, 11, 10, 9, 8]
            ]
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** ordinary level-order BFS; every second row is reversed before it is stored.

**Complexity:** `O(n)` time, `O(w)` extra space.

**Rust concepts:**
- `row.reverse()` reverses a `Vec` in place in `O(len)`.
- Keeping traversal and presentation separate (BFS order vs. output order) avoids subtle bugs.

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut levels = Vec::new();
        let mut level: Vec<Rc<RefCell<TreeNode>>> = root.into_iter().collect();
        let mut right_to_left = false;
        while !level.is_empty() {
            let mut row: Vec<i32> = level.iter().map(|node| node.borrow().val).collect();
            if right_to_left {
                row.reverse();
            }
            levels.push(row);
            right_to_left = !right_to_left;
            level = level
                .iter()
                .flat_map(|node| {
                    let node = node.borrow();
                    [node.left.clone(), node.right.clone()]
                })
                .flatten()
                .collect();
        }
        levels
    }
}
```

Full tested solution: [`src/solutions/p0684_binary_tree_zigzag_level_order_traversal.rs`](../src/solutions/p0684_binary_tree_zigzag_level_order_traversal.rs)

</details>
