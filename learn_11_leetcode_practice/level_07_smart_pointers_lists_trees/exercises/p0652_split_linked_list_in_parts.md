# Problem 0652: Split Linked List in Parts

- **Difficulty:** Medium
- **Topics:** cutting a list into owned pieces, `Vec<Option<Box<ListNode>>>`, size arithmetic
- **Source:** [LeetCode 725. Split Linked List in Parts](https://leetcode.com/problems/split-linked-list-in-parts/)

## Problem

Split a linked list into exactly `k` consecutive parts. The sizes of any two parts differ by at
most one, and earlier parts are never shorter than later ones. Some parts may be empty (`None`)
when the list has fewer than `k` nodes. Return the parts in order.

Implement
`Solution::split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>>`.

## Examples

```text
Input:  head = [1, 2, 3], k = 5
Output: [[1], [2], [3], [], []]

Input:  head = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], k = 3
Output: [[1, 2, 3, 4], [5, 6, 7], [8, 9, 10]]
```

## Constraints

- `0 <= n <= 1000`, `0 <= val <= 1000`, `1 <= k <= 50`

## Hints

1. With `n` nodes, every part gets `n / k` nodes and the first `n % k` parts get one extra.
2. For each part: the part starts as the whole remaining list; walk a cursor over its first
   `size` nodes, and `take()` the link after them. That link is the new remaining list.

## Starter code

Write your code in [`src/practice/p0652_split_linked_list_in_parts.rs`](../src/practice/p0652_split_linked_list_in_parts.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0652
```

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
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

    fn as_vecs(parts: &[Option<Box<ListNode>>]) -> Vec<Vec<i32>> {
        parts.iter().map(list_to_vec).collect()
    }

    #[test]
    fn more_parts_than_nodes() {
        let parts = Solution::split_list_to_parts(list(&[1, 2, 3]), 5);
        assert_eq!(
            as_vecs(&parts),
            vec![vec![1], vec![2], vec![3], vec![], vec![]]
        );
    }

    #[test]
    fn uneven_split() {
        let values: Vec<i32> = (1..=10).collect();
        let parts = Solution::split_list_to_parts(list(&values), 3);
        assert_eq!(
            as_vecs(&parts),
            vec![vec![1, 2, 3, 4], vec![5, 6, 7], vec![8, 9, 10]]
        );
    }

    #[test]
    fn empty_list_and_single_part() {
        let parts = Solution::split_list_to_parts(None, 3);
        assert_eq!(parts.len(), 3);
        assert!(parts.iter().all(Option::is_none));
        let parts = Solution::split_list_to_parts(list(&[4, 5]), 1);
        assert_eq!(as_vecs(&parts), vec![vec![4, 5]]);
    }

    #[test]
    fn thousand_nodes_into_fifty_parts() {
        let values: Vec<i32> = (0..1_000).collect();
        let parts = as_vecs(&Solution::split_list_to_parts(list(&values), 50));
        assert!(parts.iter().all(|p| p.len() == 20));
        assert_eq!(parts.concat(), values);
        let parts = as_vecs(&Solution::split_list_to_parts(list(&values[..999]), 50));
        assert_eq!(parts[48].len(), 20);
        assert_eq!(parts[49].len(), 19);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** compute the sizes, then repeatedly cut `size` nodes off the front of the remaining
list. Cutting means walking a `&mut Option<Box<ListNode>>` cursor `size` links forward and
calling `take()` on the link it ends on.

**Complexity:** `O(n + k)` time; the output vector holds `k` heads.

**Rust concepts:**
- Each part is an independent owned list: the cut (`take()`) is what separates ownership.
- `usize::from(i < extra)` converts a `bool` to `0` or `1`.

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let n = std::iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        let k = k as usize;
        let (base, extra) = (n / k, n % k);
        let mut parts = Vec::with_capacity(k);
        let mut rest = head;
        for i in 0..k {
            let size = base + usize::from(i < extra);
            let mut part = rest;
            let mut link = &mut part;
            for _ in 0..size {
                if let Some(node) = link {
                    link = &mut node.next;
                }
            }
            rest = link.take();
            parts.push(part);
        }
        parts
    }
}
```

Full tested solution: [`src/solutions/p0652_split_linked_list_in_parts.rs`](../src/solutions/p0652_split_linked_list_in_parts.rs)

</details>
