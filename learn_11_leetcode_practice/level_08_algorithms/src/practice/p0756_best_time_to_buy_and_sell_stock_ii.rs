// ==================================================================
// PRACTICE 0756: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0756_best_time_to_buy_and_sell_stock_ii.md
// Run the tests:          cargo test practice::p0756
// ==================================================================

//! # Problem 0756: Best Time to Buy and Sell Stock II
//!
//! - **Difficulty:** Medium
//! - **Topics:** greedy sum of rises, `windows(2)`, iterator `map` + `sum`
//! - **Source:** [LeetCode 122. Best Time to Buy and Sell Stock II](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/)
//!
//! ## Problem
//!
//! `prices[i]` is the price of a stock on day `i`. Every day you may buy one share, sell the share
//! you hold, or do nothing; you can hold **at most one** share at a time, and you may buy and sell
//! on the same day. Make as many transactions as you like.
//!
//! Return the maximum total profit.
//!
//! Implement `Solution::max_profit(prices: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  prices = [7, 1, 5, 3, 6, 4]
//! Output: 7             (buy at 1, sell at 5; buy at 3, sell at 6: 4 + 3)
//!
//! Input:  prices = [1, 2, 3, 4, 5]
//! Output: 4             (buy at 1, sell at 5, or sell and re-buy every day: same total)
//!
//! Input:  prices = [7, 6, 4, 3, 1]
//! Output: 0             (never buy)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= prices.len() <= 3 * 10^4`
//! - `0 <= prices[i] <= 10^4`
//!
//! ## Hints
//!
//! 1. A long rise from day `a` to day `b` earns the same as all the one-day rises inside it.
//! 2. So the question is only: which one-day changes do you want to collect?

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_trades() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn example_steady_rise() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn example_steady_fall() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn single_day() {
        assert_eq!(Solution::max_profit(vec![5]), 0);
    }

    #[test]
    fn flat_prices() {
        assert_eq!(Solution::max_profit(vec![3, 3, 3]), 0);
    }

    #[test]
    fn long_zigzag() {
        // 0, 10_000, 0, 10_000, ...: 15_000 rises of 10_000.
        let prices: Vec<i32> = (0..30_000)
            .map(|i| if i % 2 == 0 { 0 } else { 10_000 })
            .collect();
        assert_eq!(Solution::max_profit(prices), 150_000_000);
    }
}
