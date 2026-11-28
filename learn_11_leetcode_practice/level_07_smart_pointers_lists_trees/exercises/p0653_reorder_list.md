# Problem 0653: Reorder List

- **Difficulty:** Medium
- **Topics:** `&mut Option<Box<ListNode>>` parameters, splitting, reversing and weaving owned lists
- **Source:** [LeetCode 143. Reorder List](https://leetcode.com/problems/reorder-list/)

## Problem

A list `L0 -> L1 -> ... -> Ln-1 -> Ln` must be rearranged **in place** into
`L0 -> Ln -> L1 -> Ln-1 -> L2 -> Ln-2 -> ...`: first node, last node, second node, second to
last node, and so on. Move the nodes; do not change their values.

Implement `Solution::reorder_list(head: &mut Option<Box<ListNode>>)`. The function returns
nothing: it rearranges the list that `head` points to.

## Examples

```text
Input:  head = [1, 2, 3, 4]
After:  head = [1, 4, 2, 3]

Input:  head = [1, 2, 3, 4, 5]
After:  head = [1, 5, 2, 4, 3]
```

## Constraints

- `1 <= n <= 5 * 10^4`, `1 <= val <= 1000`

## Hints

1. Split the list in two halves: the first `n - n / 2` nodes stay in the first half.
2. Reverse the second half (problem 0630). Now the "last" nodes come first.
3. Weave: take one node from the first half, one from the reversed second half, and so on.
   Rebuild the result directly into `*head` with a tail cursor; `head.take()` gives you the
   original list to work with.

## Starter code

Write your code in [`src/practice/p0653_reorder_list.rs`](../src/practice/p0653_reorder_list.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0653
```

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
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

    fn reordered(values: &[i32]) -> Vec<i32> {
        let mut head = list(values);
        Solution::reorder_list(&mut head);
        list_to_vec(&head)
    }

    #[test]
    fn even_length() {
        assert_eq!(reordered(&[1, 2, 3, 4]), vec![1, 4, 2, 3]);
    }

    #[test]
    fn odd_length() {
        assert_eq!(reordered(&[1, 2, 3, 4, 5]), vec![1, 5, 2, 4, 3]);
    }

    #[test]
    fn short_lists_are_unchanged() {
        assert_eq!(reordered(&[1]), vec![1]);
        assert_eq!(reordered(&[1, 2]), vec![1, 2]);
        assert_eq!(reordered(&[1, 2, 3]), vec![1, 3, 2]);
    }

    #[test]
    fn long_list() {
        let values: Vec<i32> = (0..5_000).collect();
        let result = reordered(&values);
        let mut expected = Vec::new();
        let (mut lo, mut hi) = (0, 4_999);
        while lo <= hi {
            expected.push(lo);
            if lo != hi {
                expected.push(hi);
            }
            lo += 1;
            hi -= 1;
        }
        assert_eq!(result, expected);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** split, reverse, merge alternately. The function receives `&mut Option<Box<ListNode>>`,
so it first takes the whole list out of `*head`, then writes the woven list back through a tail
cursor that starts at `head` itself.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- A `&mut Option<Box<ListNode>>` parameter lets the function replace the caller's list;
  `head.take()` moves it out and leaves `None` until the new list is written back.
- `let mut tail = head;` moves the mutable reference into the cursor; the caller's variable is
  updated through it.

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

    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let n = std::iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        if n < 3 {
            return;
        }
        let keep = n - n / 2;
        let mut cursor = head.as_mut();
        for _ in 0..keep - 1 {
            cursor = cursor.and_then(|node| node.next.as_mut());
        }
        let mut second = Self::reverse(cursor.and_then(|node| node.next.take()));
        let mut first = head.take();
        let mut tail = head;
        while let Some(mut a) = first {
            first = a.next.take();
            tail = &mut tail.insert(a).next;
            if let Some(mut b) = second.take() {
                second = b.next.take();
                tail = &mut tail.insert(b).next;
            }
        }
    }
}
```

Full tested solution: [`src/solutions/p0653_reorder_list.rs`](../src/solutions/p0653_reorder_list.rs)

</details>
