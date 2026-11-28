// ==================================================================
// PRACTICE 0067: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0067_find_the_pivot_integer.md
// Run the tests:          cargo test practice::p0067
// ==================================================================

//! # Problem 0067: Find the Pivot Integer
//!
//! - **Difficulty:** Easy
//! - **Topics:** `for` with an early `return`, running sums, sum formula `n * (n + 1) / 2`
//! - **Source:** [LeetCode 2485. Find the Pivot Integer](https://leetcode.com/problems/find-the-pivot-integer/)
//!
//! ## Problem
//!
//! Given a positive integer `n`, find the **pivot integer** `x`: the number with
//! `1 + 2 + ... + x == x + (x + 1) + ... + n` (both sums include `x`). Return `x`, or `-1` if no
//! such integer exists. There is at most one pivot for a given `n`.
//!
//! Implement `Solution::pivot_integer(n: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 8
//! Output: 6
//! Explanation: 1 + 2 + 3 + 4 + 5 + 6 = 21 and 6 + 7 + 8 = 21.
//!
//! Input:  n = 1
//! Output: 1
//!
//! Input:  n = 4
//! Output: -1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 1000`
//!
//! ## Hints
//!
//! 1. The total `1 + ... + n` is `n * (n + 1) / 2`.
//! 2. Walk `x` upward, keeping the left sum `1 + ... + x`. The right sum is `total - left + x`.

pub struct Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_eight() {
        assert_eq!(Solution::pivot_integer(8), 6);
    }

    #[test]
    fn example_one() {
        assert_eq!(Solution::pivot_integer(1), 1);
    }

    #[test]
    fn example_no_pivot() {
        assert_eq!(Solution::pivot_integer(4), -1);
        assert_eq!(Solution::pivot_integer(2), -1);
    }

    #[test]
    fn all_pivots_up_to_1000() {
        let found: Vec<(i32, i32)> = (1..=1000)
            .map(|n| (n, Solution::pivot_integer(n)))
            .filter(|&(_, x)| x != -1)
            .collect();
        assert_eq!(found, vec![(1, 1), (8, 6), (49, 35), (288, 204)]);
    }

    #[test]
    fn matches_square_root_rule() {
        for n in 1..=1000 {
            let total = n * (n + 1) / 2;
            let x = Solution::pivot_integer(n);
            if x == -1 {
                let r = (total as f64).sqrt() as i32;
                assert_ne!(r * r, total, "n = {n}");
            } else {
                assert_eq!(x * x, total, "n = {n}");
            }
        }
    }
}
