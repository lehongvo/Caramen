// ==================================================================
// PRACTICE 0810: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0810_coin_change_ii.md
// Run the tests:          cargo test practice::p0810
// ==================================================================

//! # Problem 0810: Coin Change II
//!
//! - **Difficulty:** Medium
//! - **Topics:** counting combinations (unbounded knapsack), loop order, `u64::wrapping_add`
//! - **Source:** [LeetCode 518. Coin Change II](https://leetcode.com/problems/coin-change-ii/)
//!
//! ## Problem
//!
//! You have an unlimited supply of coins of each value in `coins` (all values are distinct).
//! Return the number of **combinations** of coins that add up to exactly `amount`. Two
//! combinations are the same if they use the same number of each coin; the order of the coins does
//! not matter (`1 + 2` and `2 + 1` are one combination). If no combination exists, return `0`.
//! For `amount = 0` there is exactly one combination: use no coins.
//!
//! Implement `Solution::change(amount: i32, coins: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  amount = 5, coins = [1, 2, 5]
//! Output: 4
//! Explanation: 5, 2+2+1, 2+1+1+1, 1+1+1+1+1.
//!
//! Input:  amount = 3, coins = [2]
//! Output: 0
//!
//! Input:  amount = 10, coins = [10]
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= coins.len() <= 300`
//! - `1 <= coins[i] <= 5000`, all values distinct
//! - `0 <= amount <= 5000`
//! - The answer fits in an `i32` (intermediate counts for smaller amounts may not).
//!
//! ## Hints
//!
//! 1. Let `ways[a]` be the number of combinations for amount `a` using the coins processed so far.
//! 2. Process the coins one at a time in the **outer** loop. Swapping the loops counts ordered
//!    sequences instead (that is problem 0811).

pub struct Solution;

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_amount_five() {
        assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
    }

    #[test]
    fn example_impossible() {
        assert_eq!(Solution::change(3, vec![2]), 0);
    }

    #[test]
    fn example_single_coin() {
        assert_eq!(Solution::change(10, vec![10]), 1);
    }

    #[test]
    fn zero_amount_has_one_combination() {
        assert_eq!(Solution::change(0, vec![7]), 1);
    }

    #[test]
    fn coin_larger_than_amount() {
        assert_eq!(Solution::change(4, vec![5, 1]), 1);
    }

    #[test]
    fn large_count() {
        assert_eq!(
            Solution::change(500, vec![3, 5, 7, 8, 9, 10, 11]),
            35_502_874
        );
    }
}
