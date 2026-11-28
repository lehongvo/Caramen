// ==================================================================
// PRACTICE 0817: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0817_best_time_to_buy_and_sell_stock_with_cooldown.md
// Run the tests:          cargo test practice::p0817
// ==================================================================

//! # Problem 0817: Best Time to Buy and Sell Stock with Cooldown
//!
//! - **Difficulty:** Medium
//! - **Topics:** state machine DP (holding / just sold / resting), rolling variables
//! - **Source:** [LeetCode 309. Best Time to Buy and Sell Stock with Cooldown](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/)
//!
//! ## Problem
//!
//! `prices[i]` is the price of a stock on day `i`. You may complete as many buy/sell transactions
//! as you like, with two rules:
//!
//! - you hold at most one share at a time (sell before you buy again);
//! - after you sell, you must wait one day (the **cooldown**) before buying again.
//!
//! Return the maximum total profit. Doing nothing gives a profit of `0`.
//!
//! Implement `Solution::max_profit(prices: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  prices = [1, 2, 3, 0, 2]
//! Output: 3
//! Explanation: buy (1), sell (2), cooldown, buy (0), sell (2): 1 + 2 = 3.
//!
//! Input:  prices = [1]
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= prices.len() <= 5000`
//! - `0 <= prices[i] <= 1000`
//!
//! ## Hints
//!
//! 1. At the end of each day you are in one of three states: holding a share, just sold today, or
//!    resting (no share and free to buy).
//! 2. Write the best profit of each state for day `i` in terms of day `i - 1`.
//! 3. You can buy only from the resting state; "just sold" turns into "resting" the next day.

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
    fn example_with_cooldown() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    }

    #[test]
    fn example_single_day() {
        assert_eq!(Solution::max_profit(vec![1]), 0);
    }

    #[test]
    fn one_long_transaction() {
        assert_eq!(Solution::max_profit(vec![1, 2, 4]), 3);
    }

    #[test]
    fn cooldown_changes_the_plan() {
        // Buy 1 sell 6, cooldown on 4, buy 0 (day 5), sell 2: 5 + 2 = 7.
        assert_eq!(Solution::max_profit(vec![6, 1, 6, 4, 3, 0, 2]), 7);
    }

    #[test]
    fn falling_prices_give_zero() {
        assert_eq!(Solution::max_profit((0..=1000).rev().collect()), 0);
    }

    #[test]
    fn long_repeating_pattern() {
        // [1, 3, 0] repeated, then [1, 3]: the cooldown always falls on a 0, and buying at 1 and
        // selling at 3 in each of the 1667 periods (+2 each) is optimal.
        let mut prices = [1, 3, 0].repeat(1666);
        prices.extend([1, 3]);
        assert_eq!(prices.len(), 5000);
        assert_eq!(Solution::max_profit(prices), 3334);
    }
}
