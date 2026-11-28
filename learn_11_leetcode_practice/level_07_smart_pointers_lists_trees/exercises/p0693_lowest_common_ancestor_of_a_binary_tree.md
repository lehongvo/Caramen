# Problem 0693: Lowest Common Ancestor of a Binary Tree

- **Difficulty:** Medium
- **Topics:** post-order search returning `Option<Rc<..>>`, combining two results with `match`, `Option::or`
- **Source:** [LeetCode 236. Lowest Common Ancestor of a Binary Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/)

## Problem

Same question as problem 0692, but the tree is an **ordinary** binary tree: the values give no
hint about where `p` and `q` are. Return the lowest node that has both `p` and `q` as
descendants (a node is a descendant of itself).

Implement
`Solution::lowest_common_ancestor(root, p, q) -> Option<Rc<RefCell<TreeNode>>>`, where all three
parameters are `Option<Rc<RefCell<TreeNode>>>`.

## Examples

```text
Input:  root = [3, 5, 1, 6, 2, 0, 8, null, null, 7, 4], p = 5, q = 1
Output: 3

Input:  root = [3, 5, 1, 6, 2, 0, 8, null, null, 7, 4], p = 5, q = 4
Output: 5

Input:  root = [1, 2], p = 1, q = 2
Output: 1
```

## Constraints

- `2 <= n <= 10^5`, `-10^9 <= val <= 10^9`, all values distinct.
- `p != q`, and both are nodes of the tree.

## Hints

1. Write `find(node) -> Option<Rc<..>>` that returns: `None` if the subtree contains neither
   target; the node itself if it **is** one of the targets; otherwise the results of its children
   combined.
2. If both children report something, the targets are on different sides and the current node
   is the LCA. If only one side reports something, pass that result up.
3. Values are distinct, so you can identify `p` and `q` by value.

## Starter code

Write your code in [`src/practice/p0693_lowest_common_ancestor_of_a_binary_tree.rs`](../src/practice/p0693_lowest_common_ancestor_of_a_binary_tree.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0693
```

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
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
    use crate::common::{find_node, parse_tree};

    const TREE: &str = "[3,5,1,6,2,0,8,null,null,7,4]";

    fn lca(text: &str, p: i32, q: i32) -> i32 {
        let root = parse_tree(text);
        let (p, q) = (find_node(&root, p), find_node(&root, q));
        let answer = Solution::lowest_common_ancestor(root.clone(), p, q).unwrap();
        let val = answer.borrow().val;
        assert!(Rc::ptr_eq(&answer, &find_node(&root, val).unwrap()));
        val
    }

    #[test]
    fn targets_on_both_sides() {
        assert_eq!(lca(TREE, 5, 1), 3);
    }

    #[test]
    fn target_is_an_ancestor_of_the_other() {
        assert_eq!(lca(TREE, 5, 4), 5);
        assert_eq!(lca("[1,2]", 1, 2), 1);
    }

    #[test]
    fn deep_targets() {
        assert_eq!(lca(TREE, 7, 4), 2);
        assert_eq!(lca(TREE, 6, 4), 5);
        assert_eq!(lca(TREE, 7, 8), 3);
        assert_eq!(lca(TREE, 0, 8), 1);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a single post-order search. Each call reports "a target or the LCA found in this
subtree". The first node that receives reports from both children is the LCA; if a target is an
ancestor of the other, the search stops at it and that node is reported all the way up.

**Complexity:** `O(n)` time, `O(h)` stack space.

**Rust concepts:**
- `match (left, right) { (Some(_), Some(_)) => ..., (left, right) => left.or(right) }` covers
  all four combinations in two arms.
- Returning `Rc::clone(rc)` hands out a handle to the node inside the caller's tree.

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Solution;

impl Solution {
    fn find(node: &Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let rc = node.as_ref()?;
        let current = rc.borrow();
        if current.val == p || current.val == q {
            return Some(Rc::clone(rc));
        }
        match (
            Self::find(&current.left, p, q),
            Self::find(&current.right, p, q),
        ) {
            (Some(_), Some(_)) => Some(Rc::clone(rc)),
            (left, right) => left.or(right),
        }
    }

    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p?.borrow().val;
        let q = q?.borrow().val;
        Self::find(&root, p, q)
    }
}
```

Full tested solution: [`src/solutions/p0693_lowest_common_ancestor_of_a_binary_tree.rs`](../src/solutions/p0693_lowest_common_ancestor_of_a_binary_tree.rs)

</details>
