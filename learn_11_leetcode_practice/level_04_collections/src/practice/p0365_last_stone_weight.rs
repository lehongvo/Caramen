// ==================================================================
// PRACTICE 0365: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0365_last_stone_weight.md
// Run the tests:          cargo test practice::p0365
// ==================================================================

//! # Problem 0365: Last Stone Weight
//!
//! - **Difficulty:** Easy
//! - **Topics:** `BinaryHeap::from(Vec)`, repeated `pop` of the two largest, `while let` with a tuple
//! - **Source:** [LeetCode 1046. Last Stone Weight](https://leetcode.com/problems/last-stone-weight/)
//!
//! ## Problem
//!
//! You have a collection of stones with positive integer weights. On each turn, take the **two
//! heaviest** stones, with weights `x <= y`, and smash them together:
//!
//! - if `x == y`, both stones are destroyed;
//! - otherwise the stone of weight `x` is destroyed and the other becomes a stone of weight `y - x`.
//!
//! Repeat until at most one stone is left. Return the weight of the last stone, or `0` if none is
//! left.
//!
//! Implement `Solution::last_stone_weight(stones: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  stones = [2, 7, 4, 1, 8, 1]
//! Output: 1
//! Explanation: 8 and 7 -> 1, stones [2, 4, 1, 1, 1]; 4 and 2 -> 2, stones [2, 1, 1, 1];
//!              2 and 1 -> 1, stones [1, 1, 1]; 1 and 1 -> nothing, stones [1].
//!
//! Input:  stones = [1]
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= stones.len() <= 30`
//! - `1 <= stones[i] <= 1000`
//!
//! ## Hints
//!
//! 1. "Repeatedly take the largest" is exactly what a max-heap is for.
//! 2. Pop twice; if the weights differ, push the difference back.

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]), 1);
    }

    #[test]
    fn example_single_stone() {
        assert_eq!(Solution::last_stone_weight(vec![1]), 1);
    }

    #[test]
    fn everything_cancels() {
        assert_eq!(Solution::last_stone_weight(vec![3, 3]), 0);
        assert_eq!(Solution::last_stone_weight(vec![2, 2, 4]), 0);
    }

    #[test]
    fn thirty_stones() {
        // Every round smashes the big stone against a stone of weight 1, so it shrinks by 1
        // per round: 1000 - 29 = 971.
        let mut stones = vec![1; 29];
        stones.push(1000);
        assert_eq!(Solution::last_stone_weight(stones), 971);
    }
}
