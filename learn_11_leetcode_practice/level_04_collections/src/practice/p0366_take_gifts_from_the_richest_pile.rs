// ==================================================================
// PRACTICE 0366: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0366_take_gifts_from_the_richest_pile.md
// Run the tests:          cargo test practice::p0366
// ==================================================================

//! # Problem 0366: Take Gifts From the Richest Pile
//!
//! - **Difficulty:** Easy
//! - **Topics:** `BinaryHeap` simulation with `pop` + `push`, integer square root, summing into `i64`
//! - **Source:** [LeetCode 2558. Take Gifts From the Richest Pile](https://leetcode.com/problems/take-gifts-from-the-richest-pile/)
//!
//! ## Problem
//!
//! `gifts[i]` is the number of gifts in pile `i`. Every second, for `k` seconds:
//!
//! - choose the pile with the **most** gifts (any of them if several are tied);
//! - leave behind only `floor(sqrt(g))` gifts in it, where `g` is its current size, and take the
//!   rest.
//!
//! Return the total number of gifts remaining in all piles after `k` seconds.
//!
//! Implement `Solution::pick_gifts(gifts: Vec<i32>, k: i32) -> i64`.
//!
//! ## Examples
//!
//! ```text
//! Input:  gifts = [25, 64, 9, 4, 100], k = 4
//! Output: 29
//! Explanation: 100 -> 10, then 64 -> 8, then 25 -> 5, then 10 -> 3.
//!              Remaining piles [5, 8, 9, 4, 3]: 29 gifts.
//!
//! Input:  gifts = [1, 1, 1, 1], k = 4
//! Output: 4      (floor(sqrt(1)) = 1, nothing changes)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= gifts.len() <= 10^3`
//! - `1 <= gifts[i] <= 10^9`
//! - `1 <= k <= 10^3`
//!
//! ## Hints
//!
//! 1. A max-heap gives the richest pile in `O(log n)`.
//! 2. The total can exceed `i32` (`1000 * 10^9`): sum as `i64`.

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4), 29);
    }

    #[test]
    fn example_two_ones() {
        assert_eq!(Solution::pick_gifts(vec![1, 1, 1, 1], 4), 4);
    }

    #[test]
    fn square_root_floors() {
        // 99 -> 9 -> 3 -> 1
        assert_eq!(Solution::pick_gifts(vec![99], 1), 9);
        assert_eq!(Solution::pick_gifts(vec![99], 3), 1);
        // 1_000_000_000 -> 31_622
        assert_eq!(Solution::pick_gifts(vec![1_000_000_000], 1), 31_622);
    }

    #[test]
    fn total_needs_i64() {
        let gifts = vec![1_000_000_000; 1000];
        // Only one second: one pile shrinks to 31_622, the others stay full.
        assert_eq!(Solution::pick_gifts(gifts, 1), 999 * 1_000_000_000 + 31_622);
    }

    #[test]
    fn many_seconds() {
        let gifts: Vec<i32> = (1..=1000).collect();
        // The 969 piles above 31 are reduced first (each to at most 31), so after 1000 seconds no
        // pile holds more than 31 gifts, and every pile still holds at least 1.
        let total = Solution::pick_gifts(gifts, 1000);
        assert!(total <= 31 * 1000);
        assert!(total >= 1000);
    }
}
