# Problem 0770: Hamming Distance

- **Difficulty:** Easy
- **Topics:** bit manipulation, XOR `^`, `count_ones`, clearing the lowest set bit
- **Source:** [LeetCode 461. Hamming Distance](https://leetcode.com/problems/hamming-distance/)

## Problem

The **Hamming distance** between two integers is the number of bit positions at which their
binary representations differ. Given two non-negative integers `x` and `y`, return their
Hamming distance.

Implement `Solution::hamming_distance(x: i32, y: i32) -> i32`.

## Examples

```text
Input:  x = 1, y = 4
Output: 2
        1 = 0 0 1
        4 = 1 0 0
            ^   ^   two positions differ

Input:  x = 3, y = 1
Output: 1
```

## Constraints

- `0 <= x, y <= 2^31 - 1`

## Hints

1. Which bitwise operator produces a `1` exactly where two bits differ?
2. Then count the `1` bits of the result.

## Starter code

Write your code in [`src/practice/p0770_hamming_distance.rs`](../src/practice/p0770_hamming_distance.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0770
```

```rust
pub struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
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
    fn example_one_and_four() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
    }

    #[test]
    fn example_three_and_one() {
        assert_eq!(Solution::hamming_distance(3, 1), 1);
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(Solution::hamming_distance(0, 0), 0);
        assert_eq!(Solution::hamming_distance(i32::MAX, i32::MAX), 0);
    }

    #[test]
    fn complementary_patterns() {
        assert_eq!(Solution::hamming_distance(0b1010, 0b0101), 4);
    }

    #[test]
    fn largest_distance() {
        assert_eq!(Solution::hamming_distance(0, i32::MAX), 31);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** bit manipulation: `x ^ y` (XOR) has a `1` exactly at the positions where `x` and
`y` differ, so the answer is the number of set bits of `x ^ y`. Rust's integer types provide
`count_ones()`, which compiles to a single "popcount" instruction on most CPUs. By hand, the
classic loop is `while v != 0 { v &= v - 1; count += 1; }`: `v & (v - 1)` clears the lowest set
bit, so the loop runs once per `1` bit (next problem).

**Complexity:** `O(1)` time and space (32-bit integers).

**Rust concepts:**
- The bitwise operators are `&`, `|`, `^`, `!` (bitwise not; Rust has no `~`), `<<` and `>>`:
  [The Rust Reference: arithmetic and logical operators](https://doc.rust-lang.org/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators).
- [`i32::count_ones`](https://doc.rust-lang.org/std/primitive.i32.html#method.count_ones)
  returns a `u32`, converted with `as i32`.

```rust
pub struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}
```

Full tested solution: [`src/solutions/p0770_hamming_distance.rs`](../src/solutions/p0770_hamming_distance.rs)

</details>
