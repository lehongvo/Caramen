// ==================================================================
// PRACTICE 0715: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0715_fruit_into_baskets.md
// Run the tests:          cargo test practice::p0715
// ==================================================================

//! # Problem 0715: Fruit Into Baskets
//!
//! - **Difficulty:** Medium
//! - **Topics:** sliding window with at most two distinct values, count table `Vec<usize>`
//! - **Source:** [LeetCode 904. Fruit Into Baskets](https://leetcode.com/problems/fruit-into-baskets/)
//!
//! ## Problem
//!
//! A row of trees is described by `fruits`, where `fruits[i]` is the type of fruit tree `i`
//! produces. You have **two** baskets, and each basket may hold only one type of fruit (but any
//! amount of it). You choose a starting tree and walk to the right, picking exactly one fruit from
//! every tree, and you must stop as soon as a tree's fruit fits in neither basket.
//!
//! Return the maximum number of fruits you can pick. In other words: the length of the longest
//! contiguous subarray that contains at most two distinct values.
//!
//! Implement `Solution::total_fruit(fruits: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  fruits = [1, 2, 1]
//! Output: 3
//!
//! Input:  fruits = [0, 1, 2, 2]
//! Output: 3             (pick [1, 2, 2])
//!
//! Input:  fruits = [1, 2, 3, 2, 2]
//! Output: 4             (pick [2, 3, 2, 2])
//! ```
//!
//! ## Constraints
//!
//! - `1 <= fruits.len() <= 10^5`
//! - `0 <= fruits[i] < fruits.len()`
//!
//! ## Hints
//!
//! 1. Keep a window and the number of fruits of each type inside it.
//! 2. When a third type enters, shrink the window from the left until one type disappears.

pub struct Solution;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_kinds_only() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
    }

    #[test]
    fn example_skip_first() {
        assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
    }

    #[test]
    fn example_middle_window() {
        assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
    }

    #[test]
    fn single_tree() {
        assert_eq!(Solution::total_fruit(vec![0]), 1);
    }

    #[test]
    fn longer_mixed_row() {
        assert_eq!(
            Solution::total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]),
            5
        );
    }

    #[test]
    fn long_alternating_row() {
        let fruits: Vec<i32> = (0..100_000).map(|i| i % 2).collect();
        assert_eq!(Solution::total_fruit(fruits), 100_000);
    }
}
