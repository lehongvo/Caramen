# Problem 0649: Remove Nodes From Linked List

- **Difficulty:** Medium
- **Topics:** reversing to look "to the right", running maximum, prepending restores the order
- **Source:** [LeetCode 2487. Remove Nodes From Linked List](https://leetcode.com/problems/remove-nodes-from-linked-list/)

## Problem

Remove every node that has a node with a **strictly greater** value somewhere to its right.
Return the head of the remaining list (it is non-increasing from head to tail).

Implement `Solution::remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.

## Examples

```text
Input:  head = [5, 2, 13, 3, 8]
Output: [13, 8]      // 5 and 2 are beaten by 13, 3 is beaten by 8

Input:  head = [1, 1, 1, 1]
Output: [1, 1, 1, 1] // equal values do not remove each other
```

## Constraints

- `1 <= n <= 10^5`, `1 <= val <= 10^5`

## Hints

1. A node survives if it is at least as large as **every** node after it. Walking from the end,
   that is a running maximum.
2. A singly linked list can only be walked from the front, so reverse it first (problem 0630).
3. Walk the reversed list, keep the nodes with `val >= max`, and **prepend** each kept node to
   the result: prepending reverses a second time, which restores the original order.

## Starter code

Write your code in [`src/practice/p0649_remove_nodes_from_linked_list.rs`](../src/practice/p0649_remove_nodes_from_linked_list.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0649
```

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
        let result = Solution::remove_nodes(list(&[5, 2, 13, 3, 8]));
        assert_eq!(list_to_vec(&result), vec![13, 8]);
    }

    #[test]
    fn equal_values_survive() {
        let result = Solution::remove_nodes(list(&[1, 1, 1, 1]));
        assert_eq!(list_to_vec(&result), vec![1, 1, 1, 1]);
    }

    #[test]
    fn increasing_list_keeps_only_the_last() {
        let values: Vec<i32> = (1..=5_000).collect();
        assert_eq!(Solution::remove_nodes(list(&values)), list(&[5_000]));
    }

    #[test]
    fn decreasing_list_keeps_everything() {
        let values: Vec<i32> = (1..=100).rev().collect();
        assert_eq!(list_to_vec(&Solution::remove_nodes(list(&values))), values);
        assert_eq!(Solution::remove_nodes(list(&[7])), list(&[7]));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** reverse the list; now "to the right" means "already seen". Track the maximum seen so
far; a node is kept when its value is at least that maximum. Prepending the kept nodes to an
initially empty list produces them in the original left-to-right order.

**Complexity:** `O(n)` time, `O(1)` extra space (two passes, nodes reused).

**Rust concepts:**
- `node.next = kept; kept = Some(node);` is the prepend operation: two moves, no allocation.
- A private helper inside `impl Solution` (here `reverse`) keeps the block pasteable into
  LeetCode.

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    fn reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut remaining = head;
        let mut reversed = None;
        while let Some(mut node) = remaining {
            remaining = node.next.take();
            node.next = reversed;
            reversed = Some(node);
        }
        reversed
    }

    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut rest = Self::reverse(head);
        let mut kept = None;
        let mut max = i32::MIN;
        while let Some(mut node) = rest {
            rest = node.next.take();
            if node.val >= max {
                max = node.val;
                node.next = kept;
                kept = Some(node);
            }
        }
        kept
    }
}
```

Full tested solution: [`src/solutions/p0649_remove_nodes_from_linked_list.rs`](../src/solutions/p0649_remove_nodes_from_linked_list.rs)

</details>
