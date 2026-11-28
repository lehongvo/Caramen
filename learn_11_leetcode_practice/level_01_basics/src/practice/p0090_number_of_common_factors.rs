// ==================================================================
// PRACTICE 0090: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0090_number_of_common_factors.md
// Run the tests:          cargo test practice::p0090
// ==================================================================

//! # Problem 0090: Number of Common Factors
//!
//! - **Difficulty:** Easy
//! - **Topics:** `for` over `1..=min(a, b)`, `%` with `&&`, counting
//! - **Source:** [LeetCode 2427. Number of Common Factors](https://leetcode.com/problems/number-of-common-factors/)
//!
//! ## Problem
//!
//! Given two positive integers `a` and `b`, return the number of their **common factors**: the
//! integers `x` that divide both `a` and `b` exactly.
//!
//! Implement `Solution::common_factors(a: i32, b: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  a = 12, b = 6
//! Output: 4
//! Explanation: 1, 2, 3 and 6.
//!
//! Input:  a = 25, b = 30
//! Output: 2
//! Explanation: 1 and 5.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= a, b <= 1000`
//!
//! ## Hints
//!
//! 1. A common factor cannot be larger than the smaller of the two numbers.
//! 2. The common factors of `a` and `b` are exactly the factors of `gcd(a, b)`.

pub struct Solution;

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_twelve_six() {
        assert_eq!(Solution::common_factors(12, 6), 4);
    }

    #[test]
    fn example_twenty_five_thirty() {
        assert_eq!(Solution::common_factors(25, 30), 2);
    }

    #[test]
    fn coprime_and_equal() {
        assert_eq!(Solution::common_factors(1, 1000), 1);
        assert_eq!(Solution::common_factors(997, 991), 1);
        assert_eq!(Solution::common_factors(1000, 1000), 16);
    }

    #[test]
    fn highly_divisible() {
        // gcd(840, 720) = 120, which has 16 divisors.
        assert_eq!(Solution::common_factors(840, 720), 16);
    }
}
