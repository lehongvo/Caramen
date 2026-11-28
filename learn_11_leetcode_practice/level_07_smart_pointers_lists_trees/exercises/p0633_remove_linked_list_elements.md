# Problem 0633: Remove Linked List Elements

- **Difficulty:** Easy
- **Topics:** rebuilding a list with a tail cursor, `Option::take`, why the "cursor on the current slot" loop fights the borrow checker
- **Source:** [LeetCode 203. Remove Linked List Elements](https://leetcode.com/problems/remove-linked-list-elements/)

## Problem

Given the head of a linked list and an integer `val`, remove **every** node whose value equals
`val` and return the new head. The remaining nodes keep their order. The head itself may be
removed, and so may all nodes.

Implement `Solution::remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>>`.

## Examples

```text
Input:  head = [1, 2, 6, 3, 4, 5, 6], val = 6
Output: [1, 2, 3, 4, 5]

Input:  head = [], val = 1
Output: []

Input:  head = [7, 7, 7, 7], val = 7
Output: []
```

## Constraints

- `0 <= n <= 10^4`, `1 <= node.val <= 50`, `0 <= val <= 50`

## Hints

1. Instead of unlinking in place, move every node into a new list and only *keep* the ones
   you want: `while let Some(mut node) = rest { rest = node.next.take(); ... }`.
2. Append kept nodes with a tail cursor: `tail = &mut tail.insert(node).next;`. A rejected
   `node` is simply dropped at the end of the loop body.

## Starter code

Write your code in [`src/practice/p0633_remove_linked_list_elements.rs`](../src/practice/p0633_remove_linked_list_elements.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0633
```

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
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
    fn example_one() {
        let result = Solution::remove_elements(list(&[1, 2, 6, 3, 4, 5, 6]), 6);
        assert_eq!(list_to_vec(&result), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn empty_list() {
        assert_eq!(Solution::remove_elements(None, 1), None);
    }

    #[test]
    fn every_node_removed() {
        assert_eq!(Solution::remove_elements(list(&[7, 7, 7, 7]), 7), None);
    }

    #[test]
    fn head_removed_and_value_absent() {
        let result = Solution::remove_elements(list(&[2, 2, 1, 2]), 2);
        assert_eq!(list_to_vec(&result), vec![1]);
        let result = Solution::remove_elements(list(&[1, 2, 3]), 0);
        assert_eq!(list_to_vec(&result), vec![1, 2, 3]);
    }

    #[test]
    fn long_alternating_list() {
        let values: Vec<i32> = (0..5_000)
            .map(|i| if i % 2 == 0 { 9 } else { i % 50 + 1 })
            .collect();
        let result = Solution::remove_elements(list(&values), 9);
        let expected: Vec<i32> = values.into_iter().filter(|&v| v != 9).collect();
        assert_eq!(list_to_vec(&result), expected);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** detach the nodes one by one from the input and re-link only those with a different
value onto the result. Nothing is allocated; removed nodes are freed as they are skipped.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- The tempting in-place loop `while let Some(node) = cur { if .. { *cur = node.next.take() }
  else { cur = &mut node.next } }` is rejected by today's borrow checker (error `E0506`): the
  borrow taken by `node` must last as long as `cur` in one branch, so it is still active in the
  other. Rebuilding with a separate tail cursor avoids the problem without `unwrap`.
- Moving a `Box` from one list to another moves only a pointer.

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut tail = &mut result;
        let mut rest = head;
        while let Some(mut node) = rest {
            rest = node.next.take();
            if node.val != val {
                tail = &mut tail.insert(node).next;
            }
        }
        result
    }
}
```

Full tested solution: [`src/solutions/p0633_remove_linked_list_elements.rs`](../src/solutions/p0633_remove_linked_list_elements.rs)

</details>
