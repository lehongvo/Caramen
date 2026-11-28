// ==================================================================
// PRACTICE 0045: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0045_kids_with_the_greatest_number_of_candies.md
// Run the tests:          cargo test practice::p0045
// ==================================================================

//! # Problem 0045: Kids With the Greatest Number of Candies
//!
//! - **Difficulty:** Easy
//! - **Topics:** two passes over a `Vec`, `Vec<bool>`, comparison operators
//! - **Source:** [LeetCode 1431. Kids With the Greatest Number of Candies](https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/)
//!
//! ## Problem
//!
//! There are `n` kids; kid `i` has `candies[i]` candies. You also hold `extra_candies` candies.
//! For each kid, imagine giving **all** the extra candies to that kid alone: would they then have at
//! least as many candies as every other kid? Return a `Vec<bool>` with the answer for each kid (ties
//! count as having the greatest number).
//!
//! Implement `Solution::kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  candies = [2, 3, 5, 1, 3], extra_candies = 3
//! Output: [true, true, true, false, true]
//! Explanation: the current maximum is 5. Kid 3 would have 1 + 3 = 4 < 5.
//!
//! Input:  candies = [4, 2, 1, 1, 2], extra_candies = 1
//! Output: [true, false, false, false, false]
//!
//! Input:  candies = [12, 1, 12], extra_candies = 10
//! Output: [true, false, true]
//! ```
//!
//! ## Constraints
//!
//! - `2 <= candies.len() <= 100`
//! - `1 <= candies[i] <= 100`
//! - `1 <= extra_candies <= 50`
//!
//! ## Hints
//!
//! 1. Giving candies to one kid does not change anybody else, so only the current maximum matters.
//! 2. First pass: find the maximum. Second pass: compare `candies[i] + extra_candies` with it.

pub struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
    }

    #[test]
    fn example_ties() {
        assert_eq!(
            Solution::kids_with_candies(vec![12, 1, 12], 10),
            vec![true, false, true]
        );
    }

    #[test]
    fn everyone_equal() {
        assert_eq!(Solution::kids_with_candies(vec![5, 5], 1), vec![true, true]);
    }

    #[test]
    fn exact_reach_counts() {
        // 50 + 50 == 100, so the second kid exactly reaches the maximum.
        assert_eq!(
            Solution::kids_with_candies(vec![100, 50, 49], 50),
            vec![true, true, false]
        );
    }
}
