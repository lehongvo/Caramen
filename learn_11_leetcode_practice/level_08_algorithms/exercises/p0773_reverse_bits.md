# Problem 0773: Reverse Bits

- **Difficulty:** Easy
- **Topics:** bit manipulation, shifting bits out and in, `u32::reverse_bits`
- **Source:** Adapted from [LeetCode 190. Reverse Bits](https://leetcode.com/problems/reverse-bits/)

## Problem

Given an unsigned 32-bit integer `x`, return the number whose 32-bit binary representation is
the reverse of `x`'s: bit `0` of the result is bit `31` of `x`, bit `1` is bit `30`, and so on.
All 32 bits count, including leading zeros.

Do it with shifts and masks (do not call `u32::reverse_bits`, except to check your answer).
(LeetCode's own template for this problem has changed its input type over time; this version
fixes the signature below.)

Implement `Solution::reverse_bits(x: u32) -> u32`.

## Examples

```text
Input:  x = 43261596      (00000010100101000001111010011100)
Output: 964176192         (00111001011110000010100101000000)

Input:  x = 4294967293    (11111111111111111111111111111101)
Output: 3221225471        (10111111111111111111111111111111)
```

## Constraints

- `0 <= x <= 2^32 - 1`

## Hints

1. Take the bits of `x` from the lowest one (`x & 1`, then `x >>= 1`).
2. Push each bit into the result from the right: `result = (result << 1) | bit`. After 32 steps,
   the first bit you pushed has travelled to the top.

## Starter code

Write your code in [`src/practice/p0773_reverse_bits.rs`](../src/practice/p0773_reverse_bits.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0773
```

```rust
pub struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
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
    fn example_one() {
        assert_eq!(Solution::reverse_bits(43_261_596), 964_176_192);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::reverse_bits(4_294_967_293), 3_221_225_471);
    }

    #[test]
    fn edges() {
        assert_eq!(Solution::reverse_bits(0), 0);
        assert_eq!(Solution::reverse_bits(1), 1 << 31);
        assert_eq!(Solution::reverse_bits(1 << 31), 1);
        assert_eq!(Solution::reverse_bits(u32::MAX), u32::MAX);
    }

    #[test]
    fn reversing_twice_gives_the_original() {
        for x in [5, 0xF0F0_F0F0, 0x1234_5678, 999_999_999] {
            assert_eq!(Solution::reverse_bits(Solution::reverse_bits(x)), x);
        }
    }

    #[test]
    fn matches_std_reverse_bits() {
        for x in (0..u32::MAX).step_by(104_729) {
            assert_eq!(Solution::reverse_bits(x), x.reverse_bits(), "x = {x}");
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** bit manipulation: repeat 32 times: shift the result left by one to make room, put
the lowest bit of `x` into it (`| (x & 1)`), and shift `x` right to expose the next bit. The bit
read first (bit `0` of `x`) is shifted left 31 more times, ending at bit `31`: the order is
reversed. The loop must run exactly 32 times, even when `x` becomes `0` early, so that the
leading zeros of `x` become trailing zeros of the result.

**Complexity:** `O(1)` time (32 iterations), `O(1)` space.

**Rust concepts:**
- Shifting a `u32` left drops the bits that fall off the top; Rust only panics (in debug
  builds) when the shift **amount** is 32 or more, not when bits are lost.
- The standard library has [`u32::reverse_bits`](https://doc.rust-lang.org/std/primitive.u32.html#method.reverse_bits),
  which compiles to a single instruction on some CPUs; the tests use it as a reference.

```rust
pub struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let (mut x, mut result) = (x, 0u32);
        for _ in 0..32 {
            result = (result << 1) | (x & 1);
            x >>= 1;
        }
        result
    }
}
```

Full tested solution: [`src/solutions/p0773_reverse_bits.rs`](../src/solutions/p0773_reverse_bits.rs)

</details>
