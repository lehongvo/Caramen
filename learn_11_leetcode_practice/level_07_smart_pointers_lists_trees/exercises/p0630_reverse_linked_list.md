# Problem 0630: Reverse Linked List

- **Difficulty:** Easy
- **Topics:** `Option::take`, moving boxes between two lists, `while let`, recursion vs iteration
- **Source:** [LeetCode 206. Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/)

## Problem

Reverse a singly linked list and return the new head. Reuse the existing nodes: change their
`next` links instead of creating new nodes.

Implement `Solution::reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.

## Examples

```text
Input:  head = [1, 2, 3, 4, 5]
Output: [5, 4, 3, 2, 1]

Input:  head = [1, 2]
Output: [2, 1]

Input:  head = []
Output: []
```

## Constraints

- `0 <= n <= 5000`, `-5000 <= val <= 5000`

## Hints

1. Think of two lists: `remaining` (starts as `head`) and `reversed` (starts empty). Moving the
   first node of `remaining` to the front of `reversed`, again and again, reverses the list.
2. With `while let Some(mut node) = remaining`, you own the node. `node.next.take()` detaches
   the rest, and `node.next = reversed;` links it in front.

## Starter code

Write your code in [`src/practice/p0630_reverse_linked_list.rs`](../src/practice/p0630_reverse_linked_list.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0630
```

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
    use crate::common::{list, list_to_vec};

    #[test]
    fn five_nodes() {
        let result = Solution::reverse_list(list(&[1, 2, 3, 4, 5]));
        assert_eq!(list_to_vec(&result), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn two_nodes() {
        assert_eq!(Solution::reverse_list(list(&[1, 2])), list(&[2, 1]));
    }

    #[test]
    fn empty_and_single() {
        assert_eq!(Solution::reverse_list(None), None);
        assert_eq!(Solution::reverse_list(list(&[42])), list(&[42]));
    }

    #[test]
    fn long_list() {
        let values: Vec<i32> = (-2_500..2_500).collect();
        let result = Solution::reverse_list(list(&values));
        let expected: Vec<i32> = values.into_iter().rev().collect();
        assert_eq!(list_to_vec(&result), expected);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** pop nodes off the front of the input and push them onto the front of the result. In
Rust every step is an ownership move: `node.next.take()` hands the rest of the input back to
`remaining`, then the node's `next` takes ownership of `reversed`, and the node itself becomes
the new `reversed`.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- `Option::take` leaves `None` behind, so there is never a moment where a node is owned twice.
- Each box is moved, not copied; no node is allocated or freed.
- The recursive version is shorter but uses `O(n)` stack, which is a real risk for long lists.

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut remaining = head;
        let mut reversed = None;
        while let Some(mut node) = remaining {
            remaining = node.next.take();
            node.next = reversed;
            reversed = Some(node);
        }
        reversed
    }
}
```

Full tested solution: [`src/solutions/p0630_reverse_linked_list.rs`](../src/solutions/p0630_reverse_linked_list.rs)

</details>
