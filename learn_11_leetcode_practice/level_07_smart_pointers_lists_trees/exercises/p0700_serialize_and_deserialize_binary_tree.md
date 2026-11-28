# Problem 0700: Serialize and Deserialize Binary Tree

- **Difficulty:** Hard
- **Topics:** design struct, level-order encoding with `VecDeque`, parsing with `split` and `str::parse`, rebuilding `Rc<RefCell<TreeNode>>` links
- **Source:** [LeetCode 297. Serialize and Deserialize Binary Tree](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/)

## Problem

Design a `Codec` that turns a binary tree into a `String` and back:

- `Codec::new() -> Self`;
- `serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String`;
- `deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>>`.

The format is up to you; the only requirement is that `deserialize(serialize(tree))` rebuilds
a tree with exactly the same shape and values, for every tree (including the empty one and
trees with negative values). A good choice is LeetCode's own notation: the level-order values,
with `null` for missing children, for example `"[1,2,3,null,null,4,5]"`.

## Examples

```text
let codec = Codec::new();
let text = codec.serialize(parse_tree("[1,2,3,null,null,4,5]"));
codec.deserialize(text)   -> [1, 2, 3, null, null, 4, 5]

codec.deserialize(codec.serialize(None)) -> []
```

## Constraints

- `0 <= n <= 10^4`, `-1000 <= val <= 1000`

## Hints

1. Serialize with a breadth-first traversal over `Option` slots: a `Some` node writes its value
   and enqueues both children (even missing ones); a `None` slot writes `null`. Trailing `null`s
   can be dropped.
2. Deserialize with the same queue discipline: the first token is the root; then each dequeued
   node takes the next two tokens as its left and right child.
3. A level-order format is iterative in both directions, so deep trees cannot overflow the stack.

## Starter code

Write your code in [`src/practice/p0700_serialize_and_deserialize_binary_tree.rs`](../src/practice/p0700_serialize_and_deserialize_binary_tree.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0700
```

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Codec {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl Codec {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        todo!("Write your solution here")
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
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

    fn round_trip(root: Option<Rc<RefCell<TreeNode>>>) {
        let codec = Codec::new();
        let text = codec.serialize(root.clone());
        assert_eq!(codec.deserialize(text), root);
    }

    #[test]
    fn example_tree() {
        round_trip(parse_tree("[1,2,3,null,null,4,5]"));
    }

    #[test]
    fn empty_and_single() {
        round_trip(None);
        round_trip(parse_tree("[-1000]"));
    }

    #[test]
    fn lopsided_and_negative_trees() {
        round_trip(parse_tree("[1,null,2,null,3,null,4]"));
        round_trip(parse_tree("[-1,-2,null,-3,null,-4]"));
        round_trip(parse_tree("[5,4,8,11,null,13,4,7,2,null,null,null,1]"));
    }

    #[test]
    fn large_tree() {
        let values: Vec<Option<i32>> = (0..10_000)
            .map(|i| {
                if i % 7 == 3 {
                    None
                } else {
                    Some(i % 2001 - 1000)
                }
            })
            .collect();
        round_trip(tree(&values));
    }

    #[test]
    fn serialization_is_stable() {
        // Serializing twice gives the same text, so the format is stable.
        let codec = Codec::new();
        let first = codec.serialize(parse_tree("[3,9,20,null,null,15,7]"));
        let second = codec.serialize(codec.deserialize(first.clone()));
        assert_eq!(first, second);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** LeetCode's level-order format. Serialization walks `Option<Rc<..>>` slots in BFS
order. Deserialization pairs every created node with the next two tokens, again in BFS order,
so the shapes line up exactly. Missing trailing tokens simply mean "no more children".

**Complexity:** both directions are `O(n)` time and space.

**Rust concepts:**
- The parser keeps the token iterator outside the loop and calls `next()` on it twice per
  node; running out of tokens ends the construction.
- Each new child is created once, stored in its parent through `borrow_mut()`, and queued as an
  `Rc` clone so its own children can be attached later.
- `#[allow(clippy::new_without_default)]` keeps LeetCode's exact `new()`.

```rust
use crate::common::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Codec {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl Codec {
    pub fn new() -> Self {
        Codec {}
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut tokens: Vec<String> = Vec::new();
        let mut queue = VecDeque::from([root]);
        while let Some(slot) = queue.pop_front() {
            match slot {
                None => tokens.push("null".to_string()),
                Some(node) => {
                    let node = node.borrow();
                    tokens.push(node.val.to_string());
                    queue.push_back(node.left.clone());
                    queue.push_back(node.right.clone());
                }
            }
        }
        while tokens.last().is_some_and(|token| token == "null") {
            tokens.pop();
        }
        format!("[{}]", tokens.join(","))
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let inner = data.trim().trim_start_matches('[').trim_end_matches(']');
        let mut tokens = inner
            .split(',')
            .map(str::trim)
            .filter(|token| !token.is_empty())
            .map(|token| token.parse::<i32>().ok());
        // `next()` gives `Option<Option<i32>>`: no token at all, or a `null` root, means an empty tree.
        let root = Rc::new(RefCell::new(TreeNode::new(tokens.next()??)));
        let mut queue = VecDeque::from([Rc::clone(&root)]);
        while let Some(parent) = queue.pop_front() {
            for is_left in [true, false] {
                let Some(token) = tokens.next() else {
                    return Some(root);
                };
                if let Some(val) = token {
                    let child = Rc::new(RefCell::new(TreeNode::new(val)));
                    queue.push_back(Rc::clone(&child));
                    let mut parent = parent.borrow_mut();
                    if is_left {
                        parent.left = Some(child);
                    } else {
                        parent.right = Some(child);
                    }
                }
            }
        }
        Some(root)
    }
}
```

Full tested solution: [`src/solutions/p0700_serialize_and_deserialize_binary_tree.rs`](../src/solutions/p0700_serialize_and_deserialize_binary_tree.rs)

</details>
