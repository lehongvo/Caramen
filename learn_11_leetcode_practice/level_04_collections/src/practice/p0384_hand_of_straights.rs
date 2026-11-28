// ==================================================================
// PRACTICE 0384: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0384_hand_of_straights.md
// Run the tests:          cargo test practice::p0384
// ==================================================================

//! # Problem 0384: Hand of Straights
//!
//! - **Difficulty:** Medium
//! - **Topics:** `BTreeMap` counts, `first_key_value` for the smallest key, decrement-and-remove with `get_mut`, `while let`
//! - **Source:** [LeetCode 846. Hand of Straights](https://leetcode.com/problems/hand-of-straights/)
//!
//! ## Problem
//!
//! Alice holds a hand of cards, `hand[i]` being the value of the `i`-th card. She wants to split
//! the whole hand into groups of exactly `group_size` cards, where each group consists of
//! `group_size` **consecutive** values (for example `3, 4, 5`). Every card must be used exactly
//! once. Return `true` if this is possible.
//!
//! Implement `Solution::is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  hand = [1, 2, 3, 6, 2, 3, 4, 7, 8], group_size = 3
//! Output: true      ([1, 2, 3], [2, 3, 4], [6, 7, 8])
//!
//! Input:  hand = [1, 2, 3, 4, 5], group_size = 4
//! Output: false     (5 cards cannot form groups of 4)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= hand.len() <= 10^4`
//! - `0 <= hand[i] <= 10^9`
//! - `1 <= group_size <= hand.len()`
//!
//! ## Hints
//!
//! 1. The smallest remaining card must start a group: nothing smaller is left to precede it.
//! 2. Keep counts in a `BTreeMap`, so the smallest remaining value is `first_key_value()`.
//! 3. Use up one card of each value `start, start + 1, ..., start + group_size - 1`; if one is
//!    missing, the answer is `false`.

use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert!(Solution::is_n_straight_hand(
            vec![1, 2, 3, 6, 2, 3, 4, 7, 8],
            3
        ));
    }

    #[test]
    fn example_two() {
        assert!(!Solution::is_n_straight_hand(vec![1, 2, 3, 4, 5], 4));
    }

    #[test]
    fn group_size_one_and_gaps() {
        assert!(Solution::is_n_straight_hand(vec![5, 5, 9], 1));
        // [1, 2] works, but 4 would need a 5.
        assert!(!Solution::is_n_straight_hand(vec![1, 2, 4, 6], 2));
        assert!(Solution::is_n_straight_hand(vec![4, 1, 5, 2], 2));
    }

    #[test]
    fn duplicates_need_matching_runs() {
        assert!(Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3], 3));
        assert!(!Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 4], 3));
    }

    #[test]
    fn large_hand() {
        // 2_000 groups of 5 consecutive values near 10^9, shuffled.
        let mut hand: Vec<i32> = (0..10_000)
            .map(|i| 999_990_000 + (i % 5) + 5 * (i / 5))
            .collect();
        hand.reverse();
        hand.rotate_left(4_321);
        assert!(Solution::is_n_straight_hand(hand.clone(), 5));
        hand[0] += 1;
        assert!(!Solution::is_n_straight_hand(hand, 5));
    }
}
