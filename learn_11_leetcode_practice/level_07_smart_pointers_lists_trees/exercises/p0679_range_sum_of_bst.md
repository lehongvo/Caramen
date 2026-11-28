# Problem 0679: Range Sum of BST

- **Difficulty:** Easy
- **Topics:** pruned DFS with an explicit stack, using the BST order to skip subtrees
- **Source:** [LeetCode 938. Range Sum of BST](https://leetcode.com/problems/range-sum-of-bst/)

## Problem

Given the root of a binary search tree and two integers `low <= high`, return the sum of the
values of all nodes with `low <= val <= high`.

Implement
`Solution::range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32`.

## Examples

```text
Input:  root = [10, 5, 15, 3, 7, null, 18], low = 7, high = 15
Output: 32             // 7 + 10 + 15

Input:  root = [10, 5, 15, 3, 7, 13, 18, 1, null, 6], low = 6, high = 10
Output: 23             // 6 + 7 + 10
```

## Constraints

- `1 <= n <= 2 * 10^4`, `1 <= val <= 10^5`, all values distinct
- `1 <= low <= high <= 10^5`; the answer fits in an `i32`.

## Hints

1. If a node's value is below `low`, nothing in its **left** subtree can be in range.
2. If it is above `high`, nothing in its **right** subtree can be in range.
3. Only descend into the sides that can still contain values in `[low, high]`.

## Starter code

Write your code in [`src/practice/p0679_range_sum_of_bst.rs`](../src/practice/p0679_range_sum_of_bst.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0679
```

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
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
        let root = parse_tree("[10,5,15,3,7,null,18]");
        assert_eq!(Solution::range_sum_bst(root, 7, 15), 32);
    }

    #[test]
    fn example_two() {
        let root = parse_tree("[10,5,15,3,7,13,18,1,null,6]");
        assert_eq!(Solution::range_sum_bst(root, 6, 10), 23);
    }

    #[test]
    fn single_value_ranges() {
        let root = parse_tree("[10,5,15]");
        assert_eq!(Solution::range_sum_bst(root.clone(), 5, 5), 5);
        assert_eq!(Solution::range_sum_bst(root.clone(), 11, 14), 0);
        assert_eq!(Solution::range_sum_bst(root, 1, 100), 30);
    }

    #[test]
    fn large_balanced_bst() {
        let root = parse_tree("[8,4,12,2,6,10,14,1,3,5,7,9,11,13,15]");
        assert_eq!(Solution::range_sum_bst(root, 4, 11), (4..=11).sum());
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** depth-first search that uses the BST property to prune: go left only if
`val > low`, go right only if `val < high`, and add the value when it is inside the range.

**Complexity:** `O(n)` time in the worst case, often much less; `O(h)` to `O(n)` stack space.

**Rust concepts:**
- `(low..=high).contains(&val)` reads like the mathematical condition.
- The explicit `Vec` stack avoids recursion and holds cheap `Rc` clones.

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut sum = 0;
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = root.into_iter().collect();
        while let Some(node) = stack.pop() {
            let node = node.borrow();
            if (low..=high).contains(&node.val) {
                sum += node.val;
            }
            if node.val > low {
                stack.extend(node.left.clone());
            }
            if node.val < high {
                stack.extend(node.right.clone());
            }
        }
        sum
    }
}
```

Full tested solution: [`src/solutions/p0679_range_sum_of_bst.rs`](../src/solutions/p0679_range_sum_of_bst.rs)

</details>
