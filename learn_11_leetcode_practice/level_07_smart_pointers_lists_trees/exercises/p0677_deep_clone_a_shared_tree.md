# Problem 0677: Deep Clone a Shared Tree

- **Difficulty:** Medium
- **Topics:** `Rc::clone` (shallow) vs deep copy, `Rc::ptr_eq`/`Rc::as_ptr`, structural sharing between trees
- **Source:** Original · [`Rc::ptr_eq`](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.ptr_eq) · [The Rust Book 15.4](https://doc.rust-lang.org/book/ch15-04-rc.html)

## Problem

For a tree `root: Option<Rc<RefCell<TreeNode>>>`, `root.clone()` copies only the outer `Rc`:
both "trees" are the same nodes, and a change made through one is visible through the other.
(`TreeNode` does not even implement `Clone`.) Write the tools to control sharing:

- `deep_clone(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>`: a
  completely independent copy with the same shape and values;
- `shares_nodes(a: &Option<..>, b: &Option<..>) -> bool`: `true` if at least one node
  allocation belongs to both trees;
- `replace_values(root: &Option<..>, from: i32, to: i32) -> Option<..>`: a new tree in which
  every `from` becomes `to`. The input must stay unchanged. Copy **only** the nodes that are on a
  path from the root to a replaced value; every subtree without a `from` value must be **shared**
  with the input (an `Rc::clone`, not a copy). If nothing is replaced, return a handle to the
  original root.

## Examples

```text
let root = parse_tree("[1,2,3,4,5]");
let copy = deep_clone(&root);
copy == root (same values),  shares_nodes(&copy, &root) == false
shares_nodes(&root.clone(), &root) == true      // the shallow clone shares everything

let replaced = replace_values(&root, 5, 50);    // [1,2,3,4,50]
// copied: 1, 2 and 5 (the path to the replaced value); shared: the subtrees 3 and 4
tree_to_string(&root) == "[1,2,3,4,5]"         // unchanged
```

## Constraints

- At most `10^4` nodes; depth at most `1000`.

## Hints

1. `deep_clone` is a post-order recursion that builds a new node from copied children.
2. For `shares_nodes`, collect `Rc::as_ptr` of every node of `a` into a `HashSet`, then look for
   any node of `b` in it.
3. For `replace_values`, let a helper return the new subtree **and** whether anything changed.
   If nothing changed below a node and the node itself is not `from`, return `Rc::clone` of
   the original node.

## Starter code

Write your code in [`src/practice/p0677_deep_clone_a_shared_tree.rs`](../src/practice/p0677_deep_clone_a_shared_tree.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0677
```

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

pub fn deep_clone(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    todo!("Write your solution here")
}

pub fn shares_nodes(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    todo!("Write your solution here")
}

pub fn replace_values(
    root: &Option<Rc<RefCell<TreeNode>>>,
    from: i32,
    to: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    todo!("Write your solution here")
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{find_node, parse_tree, tree_to_string};

    #[test]
    fn deep_clone_is_independent() {
        let root = parse_tree("[1,2,3,4,5]");
        let copy = deep_clone(&root);
        assert_eq!(copy, root);
        assert!(!shares_nodes(&copy, &root));
        find_node(&copy, 4).unwrap().borrow_mut().val = 40;
        assert_eq!(tree_to_string(&root), "[1,2,3,4,5]");
        assert_eq!(tree_to_string(&copy), "[1,2,3,40,5]");
    }

    #[test]
    fn shallow_clone_shares_everything() {
        let root = parse_tree("[1,2,3]");
        let alias = root.clone();
        assert!(shares_nodes(&alias, &root));
        find_node(&alias, 2).unwrap().borrow_mut().val = 20;
        assert_eq!(tree_to_string(&root), "[1,20,3]");
        assert!(!shares_nodes(&None, &root));
        assert_eq!(deep_clone(&None), None);
    }

    #[test]
    fn replace_values_copies_only_the_changed_path() {
        let root = parse_tree("[1,2,3,4,5]");
        let replaced = replace_values(&root, 5, 50);
        assert_eq!(tree_to_string(&replaced), "[1,2,3,4,50]");
        assert_eq!(tree_to_string(&root), "[1,2,3,4,5]");
        // Shared: the subtrees rooted at 3 and at 4.
        for value in [3, 4] {
            let original = find_node(&root, value).unwrap();
            let new = find_node(&replaced, value).unwrap();
            assert!(Rc::ptr_eq(&original, &new), "node {value} should be shared");
        }
        // Copied: 1, 2 and the replaced node.
        let (old_two, new_two) = (
            find_node(&root, 2).unwrap(),
            find_node(&replaced, 2).unwrap(),
        );
        assert!(!Rc::ptr_eq(&old_two, &new_two));
        assert!(!Rc::ptr_eq(
            root.as_ref().unwrap(),
            replaced.as_ref().unwrap()
        ));
    }

    #[test]
    fn nothing_to_replace_returns_the_same_root() {
        let root = parse_tree("[7,8,9]");
        let same = replace_values(&root, 100, 0);
        assert!(Rc::ptr_eq(root.as_ref().unwrap(), same.as_ref().unwrap()));
        assert_eq!(replace_values(&None, 1, 2), None);
    }

    #[test]
    fn every_occurrence_is_replaced() {
        let root = parse_tree("[5,5,1,null,5,1,1]");
        let replaced = replace_values(&root, 5, 0);
        assert_eq!(tree_to_string(&replaced), "[0,0,1,null,0,1,1]");
        let shared_one = find_node(&root, 1).unwrap();
        assert!(Rc::ptr_eq(&shared_one, &find_node(&replaced, 1).unwrap()));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `deep_clone` rebuilds every node. `shares_nodes` compares allocation addresses.
`replace_values` is a persistent update: it rebuilds the changed spine and reuses the untouched
subtrees, the same trick that makes persistent data structures cheap (problem 0612).

**Complexity:** each function is `O(n)` time; `replace_values` allocates only the copied nodes.

**Rust concepts:**
- `Rc::ptr_eq` / `Rc::as_ptr` answer "same allocation?", while `==` answers "same values?".
- Structural sharing plus `RefCell` means a later `borrow_mut()` on a shared subtree changes it
  in **both** trees. Sharing is safe for read-only use; copy before mutating.

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

fn collect_ptrs(root: &Option<Rc<RefCell<TreeNode>>>, out: &mut HashSet<*const RefCell<TreeNode>>) {
    if let Some(node) = root {
        out.insert(Rc::as_ptr(node));
        let node = node.borrow();
        collect_ptrs(&node.left, out);
        collect_ptrs(&node.right, out);
    }
}

/// Returns the updated subtree and whether anything in it was replaced.
fn replace(
    root: &Option<Rc<RefCell<TreeNode>>>,
    from: i32,
    to: i32,
) -> (Option<Rc<RefCell<TreeNode>>>, bool) {
    let Some(rc) = root else {
        return (None, false);
    };
    let node = rc.borrow();
    let (left, left_changed) = replace(&node.left, from, to);
    let (right, right_changed) = replace(&node.right, from, to);
    if !left_changed && !right_changed && node.val != from {
        return (Some(Rc::clone(rc)), false);
    }
    let val = if node.val == from { to } else { node.val };
    let copy = TreeNode { val, left, right };
    (Some(Rc::new(RefCell::new(copy))), true)
}

pub fn deep_clone(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let node = root.as_ref()?.borrow();
    Some(Rc::new(RefCell::new(TreeNode {
        val: node.val,
        left: deep_clone(&node.left),
        right: deep_clone(&node.right),
    })))
}

pub fn shares_nodes(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut seen = HashSet::new();
    collect_ptrs(a, &mut seen);
    let mut in_b = HashSet::new();
    collect_ptrs(b, &mut in_b);
    !seen.is_disjoint(&in_b)
}

pub fn replace_values(
    root: &Option<Rc<RefCell<TreeNode>>>,
    from: i32,
    to: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    replace(root, from, to).0
}
```

Full tested solution: [`src/solutions/p0677_deep_clone_a_shared_tree.rs`](../src/solutions/p0677_deep_clone_a_shared_tree.rs)

</details>
