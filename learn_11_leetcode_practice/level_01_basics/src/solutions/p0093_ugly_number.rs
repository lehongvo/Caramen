//! # Problem 0093: Ugly Number
//!
//! - **Difficulty:** Easy
//! - **Topics:** `for` over an array literal, nested `while`, dividing out factors, early `return`
//! - **Source:** [LeetCode 263. Ugly Number](https://leetcode.com/problems/ugly-number/)
//!
//! ## Problem
//!
//! An **ugly number** is a **positive** integer whose prime factors are only `2`, `3` and `5`.
//! (`1` counts as ugly: it has no prime factors at all.) Given an integer `n`, return `true` if `n`
//! is an ugly number.
//!
//! Implement `Solution::is_ugly(n: i32) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 6
//! Output: true
//! Explanation: 6 = 2 * 3.
//!
//! Input:  n = 1
//! Output: true
//!
//! Input:  n = 14
//! Output: false
//! Explanation: 14 = 2 * 7, and 7 is not allowed.
//! ```
//!
//! ## Constraints
//!
//! - `-2^31 <= n <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. Zero and negative numbers are never ugly.
//! 2. Divide out every factor 2, then every factor 3, then every factor 5. What must be left?
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** reject `n <= 0`. Then, for each allowed prime `p` in `[2, 3, 5]`, divide `n` by `p`
//! as long as it is divisible. The number is ugly exactly when what remains is `1`.
//!
//! **Complexity:** `O(log n)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - `for p in [2, 3, 5]` iterates over an array **by value** (arrays implement `IntoIterator`
//!   since the 2021 edition).
//! - The `n <= 0` guard matters: `0 % 2 == 0` forever, so without it the inner loop would never end.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        // @solution-start
        if n <= 0 {
            return false;
        }
        let mut n = n;
        for p in [2, 3, 5] {
            while n % p == 0 {
                n /= p;
            }
        }
        n == 1
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_six() {
        assert!(Solution::is_ugly(6));
    }

    #[test]
    fn example_one() {
        assert!(Solution::is_ugly(1));
    }

    #[test]
    fn example_fourteen() {
        assert!(!Solution::is_ugly(14));
    }

    #[test]
    fn zero_and_negatives() {
        assert!(!Solution::is_ugly(0));
        assert!(!Solution::is_ugly(-6));
        assert!(!Solution::is_ugly(i32::MIN));
    }

    #[test]
    fn large_values() {
        assert!(Solution::is_ugly(1 << 30));
        assert!(Solution::is_ugly(2_125_764_000)); // 2^5 * 3^12 * 5^3
        assert!(!Solution::is_ugly(i32::MAX)); // 2147483647 is prime
        let ugly: Vec<i32> = (1..=20).filter(|&n| Solution::is_ugly(n)).collect();
        assert_eq!(ugly, vec![1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 16, 18, 20]);
    }
}
