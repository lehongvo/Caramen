# Problem 0638: Delete the Middle Node of a Linked List

- **Difficulty:** Medium
- **Topics:** counting pass, stopping a `&mut` cursor before the target, unlinking with `take`
- **Source:** [LeetCode 2095. Delete the Middle Node of a Linked List](https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list/)

## Problem

Delete the middle node of a linked list and return the head. For a list of `n` nodes, the middle
node is the one at 0-based index `n / 2` (rounded down). A list with one node becomes empty.

Implement `Solution::delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.

## Examples

```text
Input:  head = [1, 3, 4, 7, 1, 2, 6]
Output: [1, 3, 4, 1, 2, 6]       // n = 7, index 3 (value 7) removed

Input:  head = [1, 2, 3, 4]
Output: [1, 2, 4]                // n = 4, index 2 removed

Input:  head = [2, 1]
Output: [2]
```

## Constraints

- `1 <= n <= 10^5`, `1 <= val <= 10^5`

## Hints

1. Count the nodes. If there is only one, return `None`.
2. Otherwise stop on the node **before** the middle (index `n / 2 - 1`) and replace its `next`
   by the middle's `next`.

## Starter code

Write your code in [`src/practice/p0638_delete_the_middle_node_of_a_linked_list.rs`](../src/practice/p0638_delete_the_middle_node_of_a_linked_list.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0638
```

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
    fn odd_length() {
        let result = Solution::delete_middle(list(&[1, 3, 4, 7, 1, 2, 6]));
        assert_eq!(list_to_vec(&result), vec![1, 3, 4, 1, 2, 6]);
    }

    #[test]
    fn even_length() {
        let result = Solution::delete_middle(list(&[1, 2, 3, 4]));
        assert_eq!(list_to_vec(&result), vec![1, 2, 4]);
    }

    #[test]
    fn two_and_one_nodes() {
        assert_eq!(Solution::delete_middle(list(&[2, 1])), list(&[2]));
        assert_eq!(Solution::delete_middle(list(&[1])), None);
    }

    #[test]
    fn long_list() {
        let values: Vec<i32> = (1..=5_000).collect();
        let result = list_to_vec(&Solution::delete_middle(list(&values)));
        assert_eq!(result.len(), 4_999);
        assert!(!result.contains(&2_501));
        assert_eq!(result[2_500], 2_502);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** count, then walk a mutable cursor `n / 2 - 1` steps and unlink the following node:
`node.next = node.next.take().and_then(|middle| middle.next);`.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- A cursor of type `Option<&mut Box<ListNode>>` advanced with
  `current.and_then(|node| node.next.as_mut())` consumes the old cursor at each step.
- The unlinked middle box is dropped automatically once its `next` has been moved out.

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let n = std::iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        if n <= 1 {
            return None;
        }
        let mut head = head;
        let mut current = head.as_mut();
        for _ in 0..n / 2 - 1 {
            current = current.and_then(|node| node.next.as_mut());
        }
        if let Some(before) = current {
            before.next = before.next.take().and_then(|middle| middle.next);
        }
        head
    }
}
```

Full tested solution: [`src/solutions/p0638_delete_the_middle_node_of_a_linked_list.rs`](../src/solutions/p0638_delete_the_middle_node_of_a_linked_list.rs)

</details>
