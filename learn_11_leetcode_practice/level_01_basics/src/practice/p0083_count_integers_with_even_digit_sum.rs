// ==================================================================
// PRACTICE 0083: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0083_count_integers_with_even_digit_sum.md
// Run the tests:          cargo test practice::p0083
// ==================================================================

//! # Problem 0083: Count Integers With Even Digit Sum
//!
//! - **Difficulty:** Easy
//! - **Topics:** `for` over `1..=num`, a nested helper function, digit sums
//! - **Source:** [LeetCode 2180. Count Integers With Even Digit Sum](https://leetcode.com/problems/count-integers-with-even-digit-sum/)
//!
//! ## Problem
//!
//! Given a positive integer `num`, return how many positive integers `x <= num` have an **even**
//! digit sum (the sum of the decimal digits of `x`).
//!
//! Implement `Solution::count_even(num: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  num = 4
//! Output: 2
//! Explanation: 2 and 4.
//!
//! Input:  num = 30
//! Output: 14
//! Explanation: 2, 4, 6, 8, 11, 13, 15, 17, 19, 20, 22, 24, 26 and 28.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= num <= 1000`
//!
//! ## Hints
//!
//! 1. Check every `x` in `1..=num` with a digit-sum helper.
//! 2. Pattern spotting (optional): among every 10 consecutive numbers `10k ..= 10k + 9`, exactly 5 have an even digit sum.

pub struct Solution;

impl Solution {
    pub fn count_even(num: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_four() {
        assert_eq!(Solution::count_even(4), 2);
    }

    #[test]
    fn example_thirty() {
        assert_eq!(Solution::count_even(30), 14);
    }

    #[test]
    fn smallest_inputs() {
        assert_eq!(Solution::count_even(1), 0);
        assert_eq!(Solution::count_even(2), 1);
    }

    #[test]
    fn largest_input() {
        // 1..=999 contains 499 such numbers (0 is excluded from 0..=999's 500); 1000 has digit sum 1.
        assert_eq!(Solution::count_even(999), 499);
        assert_eq!(Solution::count_even(1000), 499);
    }
}
