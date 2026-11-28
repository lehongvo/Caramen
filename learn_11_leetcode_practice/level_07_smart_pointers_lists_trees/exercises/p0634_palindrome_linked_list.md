# Problem 0634: Palindrome Linked List

- **Difficulty:** Easy
- **Topics:** collecting a list into a `Vec`, `std::iter::successors`, comparing with a reversed iterator
- **Source:** [LeetCode 234. Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/)

## Problem

Return `true` if the values of a singly linked list read the same forwards and backwards, and
`false` otherwise.

Implement `Solution::is_palindrome(head: Option<Box<ListNode>>) -> bool`.

## Examples

```text
Input:  head = [1, 2, 2, 1]
Output: true

Input:  head = [1, 2]
Output: false

Input:  head = [3]
Output: true
```

## Constraints

- `1 <= n <= 10^5`, `0 <= val <= 9`

## Hints

1. A singly linked list cannot be walked backwards, but a `Vec` can. Copy the values out first.
2. Compare the first half of the `Vec` with the second half read in reverse.
3. Follow-up (`O(1)` extra space): find the middle, reverse the second half in place
   (problem 0630), and compare the two halves node by node.

## Starter code

Write your code in [`src/practice/p0634_palindrome_linked_list.rs`](../src/practice/p0634_palindrome_linked_list.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0634
```

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
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
    use crate::common::list;

    #[test]
    fn even_palindrome() {
        assert!(Solution::is_palindrome(list(&[1, 2, 2, 1])));
    }

    #[test]
    fn not_a_palindrome() {
        assert!(!Solution::is_palindrome(list(&[1, 2])));
        assert!(!Solution::is_palindrome(list(&[1, 2, 3, 1])));
    }

    #[test]
    fn odd_lengths() {
        assert!(Solution::is_palindrome(list(&[3])));
        assert!(Solution::is_palindrome(list(&[1, 2, 3, 2, 1])));
        assert!(!Solution::is_palindrome(list(&[1, 2, 3, 3, 1])));
    }

    #[test]
    fn long_palindrome_with_one_mismatch() {
        let mut values: Vec<i32> = (0..2_500).map(|i| i % 10).collect();
        let mirror: Vec<i32> = values.iter().rev().copied().collect();
        values.extend(mirror);
        assert!(Solution::is_palindrome(list(&values)));
        values[1_234] = (values[1_234] + 1) % 10;
        assert!(!Solution::is_palindrome(list(&values)));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** collect the values into a `Vec<i32>` and check `values[i] == values[n - 1 - i]` for
the first half, written with iterators as `iter().zip(iter().rev())`.

**Complexity:** `O(n)` time, `O(n)` extra space.

**Rust concepts:**
- `std::iter::successors(head.as_deref(), |node| node.next.as_deref())` is a reusable way to
  iterate over `&ListNode`s.
- `Vec`'s iterator is double-ended, so `.rev()` walks it backwards for free.
- `take(n / 2)` avoids comparing every pair twice.

```rust
use crate::common::ListNode;

pub struct Solution;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let values: Vec<i32> = std::iter::successors(head.as_deref(), |node| node.next.as_deref())
            .map(|node| node.val)
            .collect();
        values
            .iter()
            .zip(values.iter().rev())
            .take(values.len() / 2)
            .all(|(front, back)| front == back)
    }
}
```

Full tested solution: [`src/solutions/p0634_palindrome_linked_list.rs`](../src/solutions/p0634_palindrome_linked_list.rs)

</details>
