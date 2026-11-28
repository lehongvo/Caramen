// ==================================================================
// PRACTICE 0809: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0809_coin_change.md
// Run the tests:          cargo test practice::p0809
// ==================================================================

//! # Problem 0809: Coin Change
//!
//! - **Difficulty:** Medium
//! - **Topics:** unbounded knapsack (minimum), `Option<u32>` as "unreachable", `filter_map`
//! - **Source:** [LeetCode 322. Coin Change](https://leetcode.com/problems/coin-change/)
//!
//! ## Problem
//!
//! You have an unlimited supply of coins of each value in `coins`. Return the fewest coins whose
//! values add up to exactly `amount`, or `-1` if that amount cannot be made. An `amount` of `0`
//! needs `0` coins.
//!
//! Implement `Solution::coin_change(coins: Vec<i32>, amount: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  coins = [1, 2, 5], amount = 11
//! Output: 3
//! Explanation: 11 = 5 + 5 + 1.
//!
//! Input:  coins = [2], amount = 3
//! Output: -1
//!
//! Input:  coins = [1], amount = 0
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= coins.len() <= 12`
//! - `1 <= coins[i] <= 2^31 - 1`
//! - `0 <= amount <= 10^4`
//!
//! ## Hints
//!
//! 1. Greedy (always take the largest coin) fails: with coins `[1, 3, 4]` and amount `6`, greedy
//!    uses `4 + 1 + 1` but `3 + 3` is better.
//! 2. Let `dp[a]` be the fewest coins for amount `a`. The last coin used is some `c <= a`.
//! 3. Some amounts are unreachable; represent that explicitly instead of with a magic number.

pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_eleven() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn example_unreachable() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }

    #[test]
    fn example_zero_amount() {
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }

    #[test]
    fn greedy_would_fail() {
        assert_eq!(Solution::coin_change(vec![1, 3, 4], 6), 2);
    }

    #[test]
    fn huge_coin_is_ignored() {
        assert_eq!(Solution::coin_change(vec![i32::MAX, 7], 14), 2);
    }

    #[test]
    fn larger_amount() {
        assert_eq!(Solution::coin_change(vec![186, 419, 83, 408], 6249), 20);
    }
}
