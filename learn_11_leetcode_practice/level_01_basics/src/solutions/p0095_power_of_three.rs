//! # Problem 0095: Power of Three
//!
//! - **Difficulty:** Easy
//! - **Topics:** `while` with `%` and `/=`, guarding non-positive input, a divisibility trick
//! - **Source:** [LeetCode 326. Power of Three](https://leetcode.com/problems/power-of-three/)
//!
//! ## Problem
//!
//! Given an integer `n`, return `true` if it is a power of three, meaning `n == 3^x` for some integer
//! `x >= 0` (so `1, 3, 9, 27, ...`).
//!
//! Implement `Solution::is_power_of_three(n: i32) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 27
//! Output: true
//!
//! Input:  n = 0
//! Output: false
//!
//! Input:  n = -1
//! Output: false
//!
//! Input:  n = 45
//! Output: false
//! Explanation: 45 = 3^2 * 5.
//! ```
//!
//! ## Constraints
//!
//! - `-2^31 <= n <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. Keep dividing by 3 while the number is divisible by 3. A power of three ends at exactly 1.
//! 2. Follow-up without a loop: the largest power of three that fits in an `i32` is `3^19 = 1162261467`.
//!    Which numbers divide it?
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** for `n > 0`, divide by 3 while `n % 3 == 0`; `n` is a power of three exactly when the
//! result is 1. Loop-free alternative: since 3 is prime, the divisors of `3^19` are exactly the
//! powers of three up to `3^19`, so `n > 0 && 1_162_261_467 % n == 0` also works.
//!
//! **Complexity:** `O(log n)` time with the loop (at most 19 divisions), `O(1)` space.
//!
//! **Rust concepts:**
//! - The guard `n > 0` is essential: with `n = 0`, `0 % 3 == 0` is always true and the loop would
//!   never stop.
//! - `n /= 3` is compound assignment, like `+=`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        // @solution-start
        if n <= 0 {
            return false;
        }
        let mut n = n;
        while n % 3 == 0 {
            n /= 3;
        }
        n == 1
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_twenty_seven() {
        assert!(Solution::is_power_of_three(27));
    }

    #[test]
    fn example_zero_and_negative() {
        assert!(!Solution::is_power_of_three(0));
        assert!(!Solution::is_power_of_three(-1));
        assert!(!Solution::is_power_of_three(-27));
    }

    #[test]
    fn example_forty_five() {
        assert!(!Solution::is_power_of_three(45));
    }

    #[test]
    fn all_powers_of_three() {
        let mut p: i32 = 1;
        for _ in 0..=19 {
            assert!(Solution::is_power_of_three(p), "{p}");
            // 2 * 3^19 does not fit in an i32, so saturate instead of overflowing.
            assert!(!Solution::is_power_of_three(p.saturating_mul(2)));
            assert!(!Solution::is_power_of_three(p + 1));
            p = p.saturating_mul(3);
        }
        assert!(Solution::is_power_of_three(1_162_261_467));
        assert!(!Solution::is_power_of_three(i32::MAX));
    }

    #[test]
    fn matches_divisibility_trick() {
        for n in 1..=100_000 {
            assert_eq!(
                Solution::is_power_of_three(n),
                1_162_261_467 % n == 0,
                "n = {n}"
            );
        }
    }
}
