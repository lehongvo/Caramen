//! # Problem 0818: Best Time to Buy and Sell Stock with Transaction Fee
//!
//! - **Difficulty:** Medium
//! - **Topics:** two-state DP (cash / holding), `fold` with a tuple state
//! - **Source:** [LeetCode 714. Best Time to Buy and Sell Stock with Transaction Fee](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/)
//!
//! ## Problem
//!
//! `prices[i]` is the price of a stock on day `i` and `fee` is a fee you pay for every completed
//! transaction (one buy plus one sell). You may do as many transactions as you like but hold at most
//! one share at a time. Return the maximum profit.
//!
//! Implement `Solution::max_profit(prices: Vec<i32>, fee: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  prices = [1, 3, 2, 8, 4, 9], fee = 2
//! Output: 8
//! Explanation: buy 1 sell 8 (7 - 2 = 5), buy 4 sell 9 (5 - 2 = 3). Total 8.
//!
//! Input:  prices = [1, 3, 7, 5, 10, 3], fee = 3
//! Output: 6
//! Explanation: one transaction, buy 1 sell 10: 9 - 3 = 6.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= prices.len() <= 5 * 10^4`
//! - `1 <= prices[i] < 5 * 10^4`
//! - `0 <= fee < 5 * 10^4`
//!
//! ## Hints
//!
//! 1. Two states at the end of each day: `cash` (no share) and `hold` (one share).
//! 2. Pay the fee when you sell (or when you buy; either works, as long as it is once per pair).
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `cash = max(cash, hold + price - fee)` (sell today or not) and
//! `hold = max(hold, cash - price)` (buy today or not). Using the updated `cash` in the `hold`
//! formula is fine: selling and re-buying on the same day only loses the fee. Start with
//! `cash = 0` and `hold = -prices[0]`.
//!
//! **Complexity:** `O(n)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - `fold((cash, hold), |(cash, hold), &p| ...)` returns the new state as a tuple; the closure
//!   shadows the outer names, which keeps the formulas readable.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        // @solution-start
        let start = (0, -prices[0]); // prices is never empty
        let (cash, _) = prices.iter().fold(start, |(cash, hold), &p| {
            let cash = cash.max(hold + p - fee);
            (cash, hold.max(cash - p))
        });
        cash
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_transactions() {
        assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
    }

    #[test]
    fn example_one_transaction() {
        assert_eq!(Solution::max_profit(vec![1, 3, 7, 5, 10, 3], 3), 6);
    }

    #[test]
    fn single_day_and_falling_prices() {
        assert_eq!(Solution::max_profit(vec![5], 1), 0);
        assert_eq!(Solution::max_profit(vec![9, 7, 4, 1], 0), 0);
    }

    #[test]
    fn zero_fee_takes_every_rise() {
        assert_eq!(Solution::max_profit(vec![1, 2, 1, 2, 1, 2], 0), 3);
    }

    #[test]
    fn fee_larger_than_every_rise() {
        assert_eq!(Solution::max_profit(vec![1, 3, 1, 3, 1, 3], 5), 0);
    }
}
