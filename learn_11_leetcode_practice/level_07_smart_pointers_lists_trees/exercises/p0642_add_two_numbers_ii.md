# Problem 0642: Add Two Numbers II

- **Difficulty:** Medium
- **Topics:** `Vec` as a stack of digits, prepending nodes (`next: result`), carry arithmetic
- **Source:** [LeetCode 445. Add Two Numbers II](https://leetcode.com/problems/add-two-numbers-ii/)

## Problem

Two non-negative integers are stored as linked lists of decimal digits, this time with the
**most significant digit first**. Return their sum as a list in the same order. The numbers
have no leading zeros, except `0` itself.

Implement
`Solution::add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>`.

## Examples

```text
Input:  l1 = [7, 2, 4, 3], l2 = [5, 6, 4]
Output: [7, 8, 0, 7]              // 7243 + 564 = 7807

Input:  l1 = [2, 4, 3], l2 = [5, 6, 4]
Output: [8, 0, 7]

Input:  l1 = [0], l2 = [0]
Output: [0]
```

## Constraints

- Each list has `1` to `100` nodes, and every `val` is a digit `0..=9`.
- The numbers have no leading zeros (except `0` itself).

## Hints

1. Addition starts at the ones digit, which is at the **end** here. Copy the digits of each
   list into a `Vec`; `pop()` then yields them from the least significant one.
2. The result digits are also produced least significant first, so **prepend** each new node:
   `result = Some(Box::new(ListNode { val: digit, next: result }));`.

## Starter code

Write your code in [`src/practice/p0642_add_two_numbers_ii.rs`](../src/practice/p0642_add_two_numbers_ii.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0642
```

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
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
    fn example_one() {
        let sum = Solution::add_two_numbers(list(&[7, 2, 4, 3]), list(&[5, 6, 4]));
        assert_eq!(list_to_vec(&sum), vec![7, 8, 0, 7]);
    }

    #[test]
    fn example_two() {
        let sum = Solution::add_two_numbers(list(&[2, 4, 3]), list(&[5, 6, 4]));
        assert_eq!(list_to_vec(&sum), vec![8, 0, 7]);
    }

    #[test]
    fn zeros() {
        assert_eq!(
            Solution::add_two_numbers(list(&[0]), list(&[0])),
            list(&[0])
        );
    }

    #[test]
    fn carry_creates_a_new_leading_digit() {
        let sum = Solution::add_two_numbers(list(&[9; 100]), list(&[1]));
        let mut expected = vec![1];
        expected.extend([0; 100]);
        assert_eq!(list_to_vec(&sum), expected);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** two stacks of digits give the reversed order for free. Pop one digit from each (or
`0`), add the carry, and push the new digit onto the **front** of the result, so the final list
is most significant first without any reversal.

**Complexity:** `O(n + m)` time and space.

**Rust concepts:**
- `Vec::pop` returns `Option<i32>`; `unwrap_or(0)` treats a finished number as zeros.
- Prepending moves the whole current result into the new node's `next` in `O(1)`.

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    fn digits(list: &Option<Box<ListNode>>) -> Vec<i32> {
        std::iter::successors(list.as_deref(), |node| node.next.as_deref())
            .map(|node| node.val)
            .collect()
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut x, mut y) = (Self::digits(&l1), Self::digits(&l2));
        let mut result: Option<Box<ListNode>> = None;
        let mut carry = 0;
        while !x.is_empty() || !y.is_empty() || carry > 0 {
            let sum = x.pop().unwrap_or(0) + y.pop().unwrap_or(0) + carry;
            carry = sum / 10;
            result = Some(Box::new(ListNode {
                val: sum % 10,
                next: result,
            }));
        }
        result
    }
}
```

Full tested solution: [`src/solutions/p0642_add_two_numbers_ii.rs`](../src/solutions/p0642_add_two_numbers_ii.rs)

</details>
