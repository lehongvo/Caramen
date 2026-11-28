# Problem 0117: Plus One

- **Difficulty:** Easy
- **Topics:** owned `Vec` parameter made `mut`, `iter_mut().rev()`, carrying, `Vec::insert`
- **Source:** [LeetCode 66. Plus One](https://leetcode.com/problems/plus-one/)

## Problem

A non-negative integer is stored as a vector of its decimal digits, most significant digit first
(`[1, 2, 3]` means 123). The number has no leading zeros, except for the number 0 itself (`[0]`).

Implement `Solution::plus_one(digits: Vec<i32>) -> Vec<i32>`, which returns the digits of the
number plus one, in the same format.

The function receives the vector **by value**, so it owns it and may modify it and return it,
instead of allocating a new one (write the parameter as `mut digits: Vec<i32>`; the type, and so
the LeetCode signature, stays the same).

## Examples

```text
Input:  digits = [1, 2, 3]
Output: [1, 2, 4]

Input:  digits = [1, 2, 9, 9]
Output: [1, 3, 0, 0]

Input:  digits = [9, 9]
Output: [1, 0, 0]
```

## Constraints

- `1 <= digits.len() <= 100`
- `0 <= digits[i] <= 9`
- `digits` has no leading zeros (except `[0]`).

## Hints

1. Walk from the last digit to the first with `digits.iter_mut().rev()`.
2. A `9` becomes `0` and the carry continues; any other digit is incremented and you are done.
3. If every digit was `9`, the answer needs one more digit in front: `insert(0, 1)`.

## Starter code

Write your code in [`src/practice/p0117_plus_one.rs`](../src/practice/p0117_plus_one.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0117
```

```rust
pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
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

    #[test]
    fn example_no_carry() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn example_carry_through_nines() {
        assert_eq!(Solution::plus_one(vec![1, 2, 9, 9]), vec![1, 3, 0, 0]);
    }

    #[test]
    fn example_all_nines() {
        assert_eq!(Solution::plus_one(vec![9, 9]), vec![1, 0, 0]);
    }

    #[test]
    fn zero_and_single_nine() {
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }

    #[test]
    fn longest_input() {
        let result = Solution::plus_one(vec![9; 100]);
        assert_eq!(result.len(), 101);
        assert_eq!(result[0], 1);
        assert!(result[1..].iter().all(|&d| d == 0));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** add one at the least significant digit. Trailing nines turn into zeros and pass the
carry to the left; the first digit that is not a nine absorbs it, and we can return immediately.
If the loop finishes, all digits were nines and the result is `1` followed by zeros.

**Complexity:** `O(n)` time; `O(1)` extra space (the input vector is reused, plus one slot in
the all-nines case).

**Rust concepts:**
- `mut digits: Vec<i32>` makes the owned parameter mutable; the caller is unaffected.
- `iter_mut().rev()` yields `&mut i32` from the back; `*d += 1` writes through the reference.
- Returning `digits` from inside the loop is accepted: the iterator's mutable borrow is not used
  after that point (non-lexical lifetimes).
- `Vec::insert(0, x)` shifts every element right, `O(n)`, which happens at most once.

```rust
pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for d in digits.iter_mut().rev() {
            if *d == 9 {
                *d = 0;
            } else {
                *d += 1;
                return digits;
            }
        }
        // Every digit was 9: 99...9 + 1 = 100...0.
        digits.insert(0, 1);
        digits
    }
}
```

Full tested solution: [`src/solutions/p0117_plus_one.rs`](../src/solutions/p0117_plus_one.rs)

</details>
