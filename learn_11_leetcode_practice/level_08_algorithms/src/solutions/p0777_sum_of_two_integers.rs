//! # Problem 0777: Sum of Two Integers
//!
//! - **Difficulty:** Medium
//! - **Topics:** addition with XOR and carries, `<<` on signed integers, two's complement
//! - **Source:** [LeetCode 371. Sum of Two Integers](https://leetcode.com/problems/sum-of-two-integers/)
//!
//! ## Problem
//!
//! Given two integers `a` and `b`, return their sum **without** using the operators `+` and `-`
//! (and without methods such as `wrapping_add` that do the addition for you). Use bitwise
//! operators.
//!
//! Implement `Solution::get_sum(a: i32, b: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  a = 1, b = 2
//! Output: 3
//!
//! Input:  a = 2, b = 3
//! Output: 5
//!
//! Input:  a = -1, b = 1
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `-1000 <= a, b <= 1000`
//!
//! ## Hints
//!
//! 1. Adding two bits without the carry is XOR. The carry is AND, moved one position to the left.
//! 2. Adding the carry may create a new carry. Repeat until there is no carry left.
//! 3. Negative numbers are stored in two's complement, so the same bit rules apply to them.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** bitwise addition: `a ^ b` adds every bit position without carrying, and
//! `(a & b) << 1` is the carry that each position sends to the next one. Their sum equals
//! `a + b`, so replace `(a, b)` by `(a ^ b, carry)` and repeat. Every round pushes the lowest
//! possible carry position one step left, so after at most 32 rounds the carry is `0` (for
//! negative numbers the carries eventually fall off the top of the 32 bits, exactly as the
//! hardware adder does) and `a` holds the sum.
//!
//! **Complexity:** `O(1)` time (at most 32 rounds), `O(1)` space.
//!
//! **Rust concepts:**
//! - In two's complement, `-1` is all ones, which is why the same XOR / AND rules work for
//!   negative numbers.
//! - `(a & b) << 1` never panics: Rust's `<<` only checks that the shift amount is below 32; bits
//!   shifted out of the top are simply lost.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        // @solution-start
        let (mut a, mut b) = (a, b);
        while b != 0 {
            let carry = (a & b) << 1;
            a ^= b; // sum without carries
            b = carry;
        }
        a
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_plus_two() {
        assert_eq!(Solution::get_sum(1, 2), 3);
    }

    #[test]
    fn example_two_plus_three() {
        assert_eq!(Solution::get_sum(2, 3), 5);
    }

    #[test]
    fn example_opposites() {
        assert_eq!(Solution::get_sum(-1, 1), 0);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(Solution::get_sum(-1000, -1000), -2000);
        assert_eq!(Solution::get_sum(1000, -1), 999);
        assert_eq!(Solution::get_sum(-7, 3), -4);
    }

    #[test]
    fn every_small_pair() {
        for a in -60..=60 {
            for b in -60..=60 {
                assert_eq!(Solution::get_sum(a, b), a + b, "{a} + {b}");
            }
        }
    }
}
