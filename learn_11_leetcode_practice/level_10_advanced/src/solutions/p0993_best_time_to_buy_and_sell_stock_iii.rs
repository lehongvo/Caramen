//! # Problem 0993: Best Time to Buy and Sell Stock III
//!
//! - **Difficulty:** Hard
//! - **Topics:** state-machine DP with four running states, `fold` over prices, `i32::MIN / 2` style sentinels avoided by ordering updates
//! - **Source:** [LeetCode 123. Best Time to Buy and Sell Stock III](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/)
//!
//! ## Problem
//!
//! `prices[i]` is a stock's price on day `i`. You may complete **at most two** transactions (buy one
//! share, then sell it), and you cannot hold more than one share at a time (sell before buying
//! again). Return the maximum profit; `0` if no transaction helps.
//!
//! Implement `Solution::max_profit(prices: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! prices = [3, 3, 5, 0, 0, 3, 1, 4]   => 6    (buy 0 sell 3, buy 1 sell 4)
//! prices = [1, 2, 3, 4, 5]            => 4    (one transaction is enough)
//! prices = [7, 6, 4, 3, 1]            => 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= prices.len() <= 10^5`, `0 <= prices[i] <= 10^5`
//!
//! ## Hints
//!
//! 1. Track the best balance after each possible event so far: first buy, first sell, second buy,
//!    second sell.
//! 2. Day by day: `buy1 = max(buy1, -p)`, `sell1 = max(sell1, buy1 + p)`,
//!    `buy2 = max(buy2, sell1 - p)`, `sell2 = max(sell2, buy2 + p)`.
//! 3. Updating in that order on the same day is fine: buying and selling on the same day only adds
//!    zero-profit transactions.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** four variables summarise every strategy: the best cash after having bought once, sold
//! once, bought twice and sold twice. Each day can improve any of them using the previous state of
//! the stage before. The answer is `sell2` (which is at least `sell1` and at least `0`).
//!
//! **Complexity:** `O(n)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - A tuple accumulator in `fold` keeps the four states together.
//! - Starting `buy1 = buy2 = -prices[0]` avoids `i32::MIN` sentinels (and their overflow risk).
// @solution-end

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // @solution-start
        let first = prices[0];
        let (_, _, _, sell2) =
            prices
                .iter()
                .fold((-first, 0, -first, 0), |(buy1, sell1, buy2, sell2), &p| {
                    let buy1 = buy1.max(-p);
                    let sell1 = sell1.max(buy1 + p);
                    let buy2 = buy2.max(sell1 - p);
                    let sell2 = sell2.max(buy2 + p);
                    (buy1, sell1, buy2, sell2)
                });
        sell2
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(prices: &[i32]) -> i32 {
        let n = prices.len();
        let one = |from: usize, to: usize| {
            let mut best = 0;
            let mut low = i32::MAX;
            for &p in &prices[from..to] {
                low = low.min(p);
                best = best.max(p - low);
            }
            best
        };
        (0..=n)
            .map(|split| one(0, split) + one(split, n))
            .max()
            .unwrap_or(0)
    }

    #[test]
    fn examples() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn small_cases() {
        assert_eq!(Solution::max_profit(vec![1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0]), 13);
        assert_eq!(Solution::max_profit(vec![2, 1, 4]), 3);
    }

    #[test]
    fn matches_split_brute_force() {
        let prices: Vec<i32> = (0..300).map(|i: i32| (i * 7_919) % 101).collect();
        assert_eq!(Solution::max_profit(prices.clone()), naive(&prices));
        for len in 1..40 {
            let slice = &prices[..len];
            assert_eq!(
                Solution::max_profit(slice.to_vec()),
                naive(slice),
                "len = {len}"
            );
        }
    }

    #[test]
    fn large_zigzag() {
        let prices: Vec<i32> = (0..100_000)
            .map(|i| if i % 2 == 0 { 0 } else { 100_000 })
            .collect();
        assert_eq!(Solution::max_profit(prices), 200_000);
    }
}
