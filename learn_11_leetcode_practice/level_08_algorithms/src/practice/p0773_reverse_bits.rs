// ==================================================================
// PRACTICE 0773: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0773_reverse_bits.md
// Run the tests:          cargo test practice::p0773
// ==================================================================

//! # Problem 0773: Reverse Bits
//!
//! - **Difficulty:** Easy
//! - **Topics:** bit manipulation, shifting bits out and in, `u32::reverse_bits`
//! - **Source:** Adapted from [LeetCode 190. Reverse Bits](https://leetcode.com/problems/reverse-bits/)
//!
//! ## Problem
//!
//! Given an unsigned 32-bit integer `x`, return the number whose 32-bit binary representation is
//! the reverse of `x`'s: bit `0` of the result is bit `31` of `x`, bit `1` is bit `30`, and so on.
//! All 32 bits count, including leading zeros.
//!
//! Do it with shifts and masks (do not call `u32::reverse_bits`, except to check your answer).
//! (LeetCode's own template for this problem has changed its input type over time; this version
//! fixes the signature below.)
//!
//! Implement `Solution::reverse_bits(x: u32) -> u32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  x = 43261596      (00000010100101000001111010011100)
//! Output: 964176192         (00111001011110000010100101000000)
//!
//! Input:  x = 4294967293    (11111111111111111111111111111101)
//! Output: 3221225471        (10111111111111111111111111111111)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= x <= 2^32 - 1`
//!
//! ## Hints
//!
//! 1. Take the bits of `x` from the lowest one (`x & 1`, then `x >>= 1`).
//! 2. Push each bit into the result from the right: `result = (result << 1) | bit`. After 32 steps,
//!    the first bit you pushed has travelled to the top.

pub struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        todo!("Write your solution here")
    }
}

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
