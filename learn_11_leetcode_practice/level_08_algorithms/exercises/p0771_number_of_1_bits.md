# Problem 0771: Number of 1 Bits

- **Difficulty:** Easy
- **Topics:** bit manipulation, `n & (n - 1)` clears the lowest set bit, `u32`
- **Source:** Adapted from [LeetCode 191. Number of 1 Bits](https://leetcode.com/problems/number-of-1-bits/)

## Problem

Given an unsigned 32-bit integer `n`, return the number of `1` bits in its binary
representation (its "population count" or "Hamming weight").

Do **not** call `count_ones`: count the bits yourself with the classic trick described in the
hints. (LeetCode's own template for this problem has changed its input type over time; this
version fixes the signature below.)

Implement `Solution::hamming_weight(n: u32) -> i32`.

## Examples

```text
Input:  n = 11              (binary 1011)
Output: 3

Input:  n = 128             (binary 1000_0000)
Output: 1

Input:  n = 2147483645      (binary 0111_1111_1111_1111_1111_1111_1111_1101)
Output: 30
```

## Constraints

- `0 <= n <= 2^32 - 1`

## Hints

1. Write `n` and `n - 1` in binary, one above the other. What happens to the lowest `1` bit and
   to the zeros below it?
2. `n & (n - 1)` is `n` with its lowest set bit cleared. How many times can you do that before
   `n` becomes `0`?

## Starter code

Write your code in [`src/practice/p0771_number_of_1_bits.rs`](../src/practice/p0771_number_of_1_bits.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0771
```

```rust
pub struct Solution;

impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
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
    fn example_eleven() {
        assert_eq!(Solution::hamming_weight(11), 3);
    }

    #[test]
    fn example_power_of_two() {
        assert_eq!(Solution::hamming_weight(128), 1);
    }

    #[test]
    fn example_thirty_bits() {
        assert_eq!(Solution::hamming_weight(2_147_483_645), 30);
    }

    #[test]
    fn extremes() {
        assert_eq!(Solution::hamming_weight(0), 0);
        assert_eq!(Solution::hamming_weight(u32::MAX), 32);
        assert_eq!(Solution::hamming_weight(1 << 31), 1);
    }

    #[test]
    fn matches_count_ones() {
        for n in (0..u32::MAX).step_by(65_537) {
            assert_eq!(
                Solution::hamming_weight(n),
                n.count_ones() as i32,
                "n = {n}"
            );
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** bit manipulation (Brian Kernighan's trick): subtracting `1` flips the lowest `1` bit
to `0` and every `0` below it to `1`, leaving higher bits unchanged. So `n & (n - 1)` keeps the
higher bits and clears exactly the lowest set bit. Repeating it until `n == 0` takes one step per
`1` bit, which is the answer. (Checking all 32 bits with `n & 1` and `n >>= 1` works too, but
always needs 32 steps.)

**Complexity:** `O(number of set bits)` time (at most 32), `O(1)` space.

**Rust concepts:**
- `n - 1` is evaluated only when `n != 0`, so the unsigned subtraction never underflows (which
  would panic in a debug build).
- In real code, use [`u32::count_ones`](https://doc.rust-lang.org/std/primitive.u32.html#method.count_ones).

```rust
pub struct Solution;

impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let (mut n, mut count) = (n, 0);
        while n != 0 {
            n &= n - 1; // clear the lowest set bit
            count += 1;
        }
        count
    }
}
```

Full tested solution: [`src/solutions/p0771_number_of_1_bits.rs`](../src/solutions/p0771_number_of_1_bits.rs)

</details>
