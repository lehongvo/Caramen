// ==================================================================
// PRACTICE 0056: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0056_sum_multiples.md
// Run the tests:          cargo test practice::p0056
// ==================================================================

//! # Problem 0056: Sum Multiples
//!
//! - **Difficulty:** Easy
//! - **Topics:** `for` over `1..=n`, `%`, `||`, accumulators
//! - **Source:** [LeetCode 2652. Sum Multiples](https://leetcode.com/problems/sum-multiples/)
//!
//! ## Problem
//!
//! Given a positive integer `n`, return the sum of all integers from `1` to `n` (inclusive) that are
//! divisible by **at least one** of `3`, `5` and `7`.
//!
//! Implement `Solution::sum_of_multiples(n: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 7
//! Output: 21
//! Explanation: 3 + 5 + 6 + 7 = 21.
//!
//! Input:  n = 10
//! Output: 40
//! Explanation: 3 + 5 + 6 + 7 + 9 + 10 = 40.
//!
//! Input:  n = 9
//! Output: 30
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 10^3`
//!
//! ## Hints
//!
//! 1. Loop over `1..=n` and test each number.
//! 2. A number counts once even if it is divisible by several of the three (like 15 or 21).

pub struct Solution;

impl Solution {
    pub fn sum_of_multiples(n: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_seven() {
        assert_eq!(Solution::sum_of_multiples(7), 21);
    }

    #[test]
    fn example_ten() {
        assert_eq!(Solution::sum_of_multiples(10), 40);
    }

    #[test]
    fn example_nine() {
        assert_eq!(Solution::sum_of_multiples(9), 30);
    }

    #[test]
    fn small_values() {
        assert_eq!(Solution::sum_of_multiples(1), 0);
        assert_eq!(Solution::sum_of_multiples(2), 0);
        assert_eq!(Solution::sum_of_multiples(3), 3);
    }

    #[test]
    fn largest_input_matches_inclusion_exclusion() {
        let n = 1000;
        let s = |k: i32| {
            let m = n / k;
            k * m * (m + 1) / 2
        };
        let expected = s(3) + s(5) + s(7) - s(15) - s(21) - s(35) + s(105);
        assert_eq!(Solution::sum_of_multiples(n), expected);
        assert_eq!(expected, 272_066);
    }
}
