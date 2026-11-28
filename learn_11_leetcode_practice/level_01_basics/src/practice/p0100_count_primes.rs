// ==================================================================
// PRACTICE 0100: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0100_count_primes.md
// Run the tests:          cargo test practice::p0100
// ==================================================================

//! # Problem 0100: Count Primes
//!
//! - **Difficulty:** Medium
//! - **Topics:** sieve of Eratosthenes, `vec![true; n]`, `step_by`, `usize` indices, `as` casts
//! - **Source:** [LeetCode 204. Count Primes](https://leetcode.com/problems/count-primes/)
//!
//! ## Problem
//!
//! Given an integer `n`, return the number of prime numbers that are **strictly less** than `n`.
//!
//! Implement `Solution::count_primes(n: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 10
//! Output: 4
//! Explanation: 2, 3, 5 and 7.
//!
//! Input:  n = 0
//! Output: 0
//!
//! Input:  n = 1
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 5 * 10^6`
//!
//! ## Hints
//!
//! 1. Testing each number by trial division (problem 0091) is too slow for five million numbers.
//! 2. Sieve of Eratosthenes: start with every number marked "prime"; for each prime `p`, cross out
//!    its multiples.
//! 3. Crossing out can start at `p * p`: smaller multiples were already crossed out by smaller primes.
//!    And you only need primes with `p * p < n`.

pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_ten() {
        assert_eq!(Solution::count_primes(10), 4);
    }

    #[test]
    fn example_zero_and_one() {
        assert_eq!(Solution::count_primes(0), 0);
        assert_eq!(Solution::count_primes(1), 0);
    }

    #[test]
    fn strictly_less_than_n() {
        assert_eq!(Solution::count_primes(2), 0);
        assert_eq!(Solution::count_primes(3), 1);
        assert_eq!(Solution::count_primes(11), 4);
        assert_eq!(Solution::count_primes(12), 5);
    }

    #[test]
    fn known_counts() {
        assert_eq!(Solution::count_primes(100), 25);
        assert_eq!(Solution::count_primes(1_000), 168);
        assert_eq!(Solution::count_primes(1_000_000), 78_498);
    }

    #[test]
    fn largest_input() {
        assert_eq!(Solution::count_primes(5_000_000), 348_513);
    }
}
