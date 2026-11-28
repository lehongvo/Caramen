// ==================================================================
// PRACTICE 0467: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0467_best_time_to_buy_and_sell_stock.md
// Run the tests:          cargo test practice::p0467
// ==================================================================

//! # Problem 0467: Best Time to Buy and Sell Stock
//!
//! - **Difficulty:** Easy
//! - **Topics:** `fold` with a tuple state `(lowest price so far, best profit)`, one-pass greedy
//! - **Source:** [LeetCode 121. Best Time to Buy and Sell Stock](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/)
//!
//! ## Problem
//!
//! `prices[i]` is the price of a stock on day `i`. You may buy one share on some day and sell it on
//! a **later** day. Return the maximum profit you can make. If no transaction makes money, return
//! `0` (you simply do not trade).
//!
//! Implement `Solution::max_profit(prices: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  prices = [7, 1, 5, 3, 6, 4]
//! Output: 5
//! Explanation: buy on day 1 (price 1) and sell on day 4 (price 6).
//!
//! Input:  prices = [7, 6, 4, 3, 1]
//! Output: 0
//! Explanation: the price only goes down, so no profitable trade exists.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= prices.len() <= 10^5`
//! - `0 <= prices[i] <= 10^4`
//!
//! ## Hints
//!
//! 1. If you sell on day `i`, the best buying day is the cheapest day **before** `i`.
//! 2. Walk the prices once and remember two numbers: the lowest price seen so far and the best
//!    profit seen so far. `fold` with a tuple accumulator holds both.

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
    fn example_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn example_falling_prices() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn one_day_and_flat_prices() {
        assert_eq!(Solution::max_profit(vec![5]), 0);
        assert_eq!(Solution::max_profit(vec![3, 3, 3]), 0);
    }

    #[test]
    fn minimum_after_the_best_sale() {
        assert_eq!(Solution::max_profit(vec![2, 9, 1, 4]), 7);
        assert_eq!(Solution::max_profit(vec![3, 2, 6, 5, 0, 3]), 4);
    }

    #[test]
    fn large_input() {
        let mut prices: Vec<i32> = (0..100_000).map(|i| 10_000 - (i % 10_001)).collect();
        prices.push(10_000);
        assert_eq!(Solution::max_profit(prices), 10_000);
    }
}
