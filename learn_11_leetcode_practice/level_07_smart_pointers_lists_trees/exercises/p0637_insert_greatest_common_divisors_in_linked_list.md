# Problem 0637: Insert Greatest Common Divisors in Linked List

- **Difficulty:** Medium
- **Topics:** inserting nodes through `&mut Box<ListNode>`, `Option::take`, Euclid's algorithm
- **Source:** [LeetCode 2807. Insert Greatest Common Divisors in Linked List](https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list/)

## Problem

Between every pair of adjacent nodes of a linked list, insert a new node whose value is the
**greatest common divisor** (GCD) of the two neighbours. Return the head of the list.

Implement
`Solution::insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.

## Examples

```text
Input:  head = [18, 6, 10, 3]
Output: [18, 6, 6, 2, 10, 1, 3]     // gcd(18,6)=6, gcd(6,10)=2, gcd(10,3)=1

Input:  head = [7]
Output: [7]                          // no adjacent pairs
```

## Constraints

- `1 <= n <= 5000`, `1 <= val <= 1000`

## Hints

1. Euclid: `gcd(a, 0) = a` and `gcd(a, b) = gcd(b, a % b)`.
2. Walk `let mut current = head.as_mut();`. For a node with a successor, detach the successor
   with `node.next.take()`, build the new node pointing to it, and store the new node in
   `node.next`.
3. Then jump **two** nodes ahead (over the node you inserted).

## Starter code

Write your code in [`src/practice/p0637_insert_greatest_common_divisors_in_linked_list.rs`](../src/practice/p0637_insert_greatest_common_divisors_in_linked_list.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0637
```

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
        let result = Solution::insert_greatest_common_divisors(list(&[18, 6, 10, 3]));
        assert_eq!(list_to_vec(&result), vec![18, 6, 6, 2, 10, 1, 3]);
    }

    #[test]
    fn single_node() {
        assert_eq!(
            Solution::insert_greatest_common_divisors(list(&[7])),
            list(&[7])
        );
    }

    #[test]
    fn equal_and_coprime_neighbours() {
        let result = Solution::insert_greatest_common_divisors(list(&[4, 4, 9, 1000]));
        assert_eq!(list_to_vec(&result), vec![4, 4, 4, 1, 9, 1, 1000]);
    }

    #[test]
    fn long_list() {
        let values: Vec<i32> = (1..=2_000).map(|i| i * 2).collect();
        let result = list_to_vec(&Solution::insert_greatest_common_divisors(list(&values)));
        assert_eq!(result.len(), 3_999);
        // gcd(2i, 2i + 2) = 2 for every pair of consecutive even numbers.
        assert!(result.iter().skip(1).step_by(2).all(|&g| g == 2));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** for each original node `a` with successor `b`, splice a new node `gcd(a, b)` between
them, then continue from `b`. The helper `gcd` is an associated function, so the whole
`impl Solution` block can be pasted into LeetCode.

**Complexity:** `O(n log V)` time for values up to `V` (each GCD is logarithmic); `O(n)` new
nodes.

**Rust concepts:**
- `node.next.take()` gives ownership of the rest of the list, which the new node then owns.
- `node.next.as_mut().and_then(|mid| mid.next.as_mut())` moves the cursor two steps in a way the
  borrow checker can follow.

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }

    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = head.as_mut();
        while let Some(node) = current {
            let Some(next) = node.next.take() else {
                break;
            };
            let middle = ListNode {
                val: Self::gcd(node.val, next.val),
                next: Some(next),
            };
            node.next = Some(Box::new(middle));
            current = node.next.as_mut().and_then(|middle| middle.next.as_mut());
        }
        head
    }
}
```

Full tested solution: [`src/solutions/p0637_insert_greatest_common_divisors_in_linked_list.rs`](../src/solutions/p0637_insert_greatest_common_divisors_in_linked_list.rs)

</details>
