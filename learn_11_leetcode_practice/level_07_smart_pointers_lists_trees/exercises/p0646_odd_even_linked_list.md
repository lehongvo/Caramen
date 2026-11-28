# Problem 0646: Odd Even Linked List

- **Difficulty:** Medium
- **Topics:** splitting by position with two tail cursors, `Option::insert`, a flipping `bool`, joining lists
- **Source:** [LeetCode 328. Odd Even Linked List](https://leetcode.com/problems/odd-even-linked-list/)

## Problem

Number the nodes of a linked list by **position**, starting at `1`. Reorder the list so that
all nodes at odd positions come first, followed by all nodes at even positions; each group keeps
its original order. (The node values do not matter, only their positions.) Use `O(1)` extra
space besides the nodes themselves.

Implement `Solution::odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.

## Examples

```text
Input:  head = [1, 2, 3, 4, 5]
Output: [1, 3, 5, 2, 4]

Input:  head = [2, 1, 3, 5, 6, 4, 7]
Output: [2, 3, 6, 7, 1, 5, 4]
```

## Constraints

- `0 <= n <= 10^4`, `-10^6 <= val <= 10^6`

## Hints

1. This is problem 0645 with a different rule: the position decides the group, not the value.
2. Keep a counter (or a `bool` that flips) while you detach the nodes one by one.

## Starter code

Write your code in [`src/practice/p0646_odd_even_linked_list.rs`](../src/practice/p0646_odd_even_linked_list.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0646
```

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
        let result = Solution::odd_even_list(list(&[1, 2, 3, 4, 5]));
        assert_eq!(list_to_vec(&result), vec![1, 3, 5, 2, 4]);
    }

    #[test]
    fn example_two() {
        let result = Solution::odd_even_list(list(&[2, 1, 3, 5, 6, 4, 7]));
        assert_eq!(list_to_vec(&result), vec![2, 3, 6, 7, 1, 5, 4]);
    }

    #[test]
    fn short_lists() {
        assert_eq!(Solution::odd_even_list(None), None);
        assert_eq!(Solution::odd_even_list(list(&[1])), list(&[1]));
        assert_eq!(Solution::odd_even_list(list(&[1, 2])), list(&[1, 2]));
    }

    #[test]
    fn long_list() {
        let values: Vec<i32> = (1..=5_000).collect();
        let result = list_to_vec(&Solution::odd_even_list(list(&values)));
        let mut expected: Vec<i32> = (1..=5_000).step_by(2).collect();
        expected.extend((2..=5_000).step_by(2));
        assert_eq!(result, expected);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** detach the nodes in order and append them alternately to the `odd` and `even` lists;
finally attach `even` after `odd`.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- A flipping `bool` (`is_odd = !is_odd`) is clearer than computing `index % 2`.
- Moving boxes between lists never allocates, which is what "`O(1)` extra space" asks for.

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut odd = None;
        let mut odd_tail = &mut odd;
        let mut even = None;
        let mut even_tail = &mut even;
        let mut is_odd = true;
        let mut rest = head;
        while let Some(mut node) = rest {
            rest = node.next.take();
            if is_odd {
                odd_tail = &mut odd_tail.insert(node).next;
            } else {
                even_tail = &mut even_tail.insert(node).next;
            }
            is_odd = !is_odd;
        }
        *odd_tail = even;
        odd
    }
}
```

Full tested solution: [`src/solutions/p0646_odd_even_linked_list.rs`](../src/solutions/p0646_odd_even_linked_list.rs)

</details>
