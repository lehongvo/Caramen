# Problem 0651: Reverse Linked List II

- **Difficulty:** Medium
- **Topics:** dummy head, detaching a segment, reversing by prepending, re-attaching both ends
- **Source:** [LeetCode 92. Reverse Linked List II](https://leetcode.com/problems/reverse-linked-list-ii/)

## Problem

Reverse only the part of a linked list between positions `left` and `right` (1-based, both
included) and return the head of the whole list. The nodes before `left` and after `right` stay
where they are.

Implement
`Solution::reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>>`.

## Examples

```text
Input:  head = [1, 2, 3, 4, 5], left = 2, right = 4
Output: [1, 4, 3, 2, 5]

Input:  head = [5], left = 1, right = 1
Output: [5]

Input:  head = [1, 2, 3], left = 1, right = 3
Output: [3, 2, 1]
```

## Constraints

- `1 <= n <= 500`, `-500 <= val <= 500`, `1 <= left <= right <= n`

## Hints

1. Put a dummy node in front, so the node before position `left` always exists (problem 0640).
2. Walk to that node (`left - 1` steps from the dummy) and detach everything after it.
3. Move `right - left + 1` nodes from the detached part onto the front of a `reversed` list
   (problem 0630). Attach `reversed` after the node you stopped at, walk to its end, and attach
   the remaining nodes there.

## Starter code

Write your code in [`src/practice/p0651_reverse_linked_list_ii.rs`](../src/practice/p0651_reverse_linked_list_ii.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0651
```

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
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
    fn middle_segment() {
        let result = Solution::reverse_between(list(&[1, 2, 3, 4, 5]), 2, 4);
        assert_eq!(list_to_vec(&result), vec![1, 4, 3, 2, 5]);
    }

    #[test]
    fn single_node_and_empty_range() {
        assert_eq!(Solution::reverse_between(list(&[5]), 1, 1), list(&[5]));
        assert_eq!(
            Solution::reverse_between(list(&[1, 2, 3]), 2, 2),
            list(&[1, 2, 3])
        );
    }

    #[test]
    fn whole_list_and_edges() {
        assert_eq!(
            Solution::reverse_between(list(&[1, 2, 3]), 1, 3),
            list(&[3, 2, 1])
        );
        assert_eq!(
            Solution::reverse_between(list(&[1, 2, 3]), 1, 2),
            list(&[2, 1, 3])
        );
        assert_eq!(
            Solution::reverse_between(list(&[1, 2, 3]), 2, 3),
            list(&[1, 3, 2])
        );
    }

    #[test]
    fn every_range_of_a_long_list() {
        let values: Vec<i32> = (0..40).collect();
        for left in 1..=40 {
            for right in left..=40 {
                let result = list_to_vec(&Solution::reverse_between(list(&values), left, right));
                let mut expected = values.clone();
                expected[left as usize - 1..right as usize].reverse();
                assert_eq!(result, expected, "left = {left}, right = {right}");
            }
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** three pieces: the prefix (ending at `before`), the segment (reversed while it is
detached), and the suffix (`rest`). Reversing by repeated prepending needs no index bookkeeping.
Re-linking is two writes: `before.next = reversed` and, at the end of the reversed segment,
`next = rest`.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- `before = before.next.as_mut()?;` walks a `&mut Box<ListNode>` cursor; the `?` cannot fire
  because `left <= n`.
- `let Some(mut node) = rest.take() else { break };` (let-else, stable since 1.65) handles the
  impossible "list too short" case without `unwrap`.

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut before = &mut dummy;
        for _ in 1..left {
            before = before.next.as_mut()?;
        }
        let mut rest = before.next.take();
        let mut reversed = None;
        for _ in left..=right {
            let Some(mut node) = rest.take() else {
                break;
            };
            rest = node.next.take();
            node.next = reversed;
            reversed = Some(node);
        }
        before.next = reversed;
        let mut tail = &mut before.next;
        while let Some(node) = tail {
            tail = &mut node.next;
        }
        *tail = rest;
        dummy.next
    }
}
```

Full tested solution: [`src/solutions/p0651_reverse_linked_list_ii.rs`](../src/solutions/p0651_reverse_linked_list_ii.rs)

</details>
