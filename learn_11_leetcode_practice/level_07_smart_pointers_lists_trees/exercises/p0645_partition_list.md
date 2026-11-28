# Problem 0645: Partition List

- **Difficulty:** Medium
- **Topics:** two output lists with two tail cursors, stable partition, joining lists
- **Source:** [LeetCode 86. Partition List](https://leetcode.com/problems/partition-list/)

## Problem

Rearrange a linked list so that every node with a value **less than** `x` comes before every
node with a value **greater than or equal to** `x`. Inside each of the two groups the nodes keep
their original relative order. Return the new head.

Implement `Solution::partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>>`.

## Examples

```text
Input:  head = [1, 4, 3, 2, 5, 2], x = 3
Output: [1, 2, 2, 4, 3, 5]

Input:  head = [2, 1], x = 2
Output: [1, 2]
```

## Constraints

- `0 <= n <= 200`, `-100 <= val <= 100`, `-200 <= x <= 200`

## Hints

1. Build two lists while you walk the input: `small` and `large`, each with its own tail cursor.
2. At the end, attach `large` to the end of `small`: `*small_tail = large;`.

## Starter code

Write your code in [`src/practice/p0645_partition_list.rs`](../src/practice/p0645_partition_list.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0645
```

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
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
        let result = Solution::partition(list(&[1, 4, 3, 2, 5, 2]), 3);
        assert_eq!(list_to_vec(&result), vec![1, 2, 2, 4, 3, 5]);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::partition(list(&[2, 1]), 2), list(&[1, 2]));
    }

    #[test]
    fn all_on_one_side() {
        assert_eq!(Solution::partition(list(&[3, 1, 2]), 10), list(&[3, 1, 2]));
        assert_eq!(Solution::partition(list(&[3, 1, 2]), -10), list(&[3, 1, 2]));
        assert_eq!(Solution::partition(None, 0), None);
    }

    #[test]
    fn stable_for_many_values() {
        let values: Vec<i32> = (0..200).map(|i| (i * 37) % 201 - 100).collect();
        let result = list_to_vec(&Solution::partition(list(&values), 0));
        let mut expected: Vec<i32> = values.iter().copied().filter(|&v| v < 0).collect();
        expected.extend(values.iter().copied().filter(|&v| v >= 0));
        assert_eq!(result, expected);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** detach each node from the input and append it to `small` or `large` depending on its
value. Appending at the tail keeps the original order within each group. Finally, link the two
lists.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- Two independent `&mut Option<Box<ListNode>>` cursors are fine because they point into two
  different lists.
- `*small_tail = large;` is allowed once `large_tail` is no longer used: non-lexical lifetimes
  end that borrow early.

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut small = None;
        let mut small_tail = &mut small;
        let mut large = None;
        let mut large_tail = &mut large;
        let mut rest = head;
        while let Some(mut node) = rest {
            rest = node.next.take();
            if node.val < x {
                small_tail = &mut small_tail.insert(node).next;
            } else {
                large_tail = &mut large_tail.insert(node).next;
            }
        }
        *small_tail = large;
        small
    }
}
```

Full tested solution: [`src/solutions/p0645_partition_list.rs`](../src/solutions/p0645_partition_list.rs)

</details>
