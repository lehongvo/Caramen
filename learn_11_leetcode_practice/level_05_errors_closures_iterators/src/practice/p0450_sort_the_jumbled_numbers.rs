// ==================================================================
// PRACTICE 0450: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0450_sort_the_jumbled_numbers.md
// Run the tests:          cargo test practice::p0450
// ==================================================================

//! # Problem 0450: Sort the Jumbled Numbers
//!
//! - **Difficulty:** Medium
//! - **Topics:** `sort_by_cached_key` with a closure capturing `mapping`, stable sorting, digits with `to_string().bytes()`, `fold`
//! - **Source:** [LeetCode 2191. Sort the Jumbled Numbers](https://leetcode.com/problems/sort-the-jumbled-numbers/)
//!
//! ## Problem
//!
//! `mapping` is a permutation of the digits `0..=9`: digit `d` is "jumbled" into `mapping[d]`. The
//! *mapped value* of a number replaces each of its decimal digits `d` with `mapping[d]` (leading zeros
//! in the result are fine, so it is just a number again).
//!
//! Sort `nums` in **non-decreasing order of mapped values**. Numbers with the same mapped value
//! must keep their relative order from the input. The returned numbers are the original numbers,
//! not their mapped values.
//!
//! Implement `Solution::sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  mapping = [8,9,4,0,2,1,3,5,7,6], nums = [991,338,38]
//! Output: [338,38,991]
//! Explanation: 991 -> 669, 338 -> 007 = 7, 38 -> 07 = 7. 338 and 38 tie, so they keep their order.
//!
//! Input:  mapping = [0,1,2,3,4,5,6,7,8,9], nums = [789,456,123]
//! Output: [123,456,789]
//! ```
//!
//! ## Constraints
//!
//! - `mapping.len() == 10`, `0 <= mapping[i] <= 9`, all `mapping[i]` are distinct.
//! - `1 <= nums.len() <= 3 * 10^4`
//! - `0 <= nums[i] < 10^9`
//!
//! ## Hints
//!
//! 1. Write a closure `mapped(n)` that captures `mapping` and builds the mapped value digit by
//!    digit. Remember that `0` has one digit.
//! 2. A **stable** sort keeps equal keys in input order. `sort_by_key` and `sort_by_cached_key` are
//!    stable; `sort_unstable_by_key` is not.

pub struct Solution;

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_with_ties() {
        assert_eq!(
            Solution::sort_jumbled(vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6], vec![991, 338, 38]),
            vec![338, 38, 991]
        );
    }

    #[test]
    fn example_identity_mapping() {
        assert_eq!(
            Solution::sort_jumbled((0..10).collect(), vec![789, 456, 123]),
            vec![123, 456, 789]
        );
    }

    #[test]
    fn zero_has_one_digit() {
        // 0 -> 9, 9 -> 0, so 0 maps to 9 and 9 maps to 0.
        let mapping = vec![9, 1, 2, 3, 4, 5, 6, 7, 8, 0];
        assert_eq!(
            Solution::sort_jumbled(mapping, vec![0, 9, 5]),
            vec![9, 5, 0]
        );
    }

    #[test]
    fn reversed_mapping_and_stability() {
        let mapping: Vec<i32> = (0..10).rev().collect();
        // 1 -> 8, 10 -> 89, 100 -> 899; 8 -> 1.
        assert_eq!(
            Solution::sort_jumbled(mapping, vec![100, 1, 10, 8]),
            vec![8, 1, 10, 100]
        );
        let identity = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(
            Solution::sort_jumbled(identity, vec![5, 5, 3, 5]),
            vec![3, 5, 5, 5]
        );
    }

    #[test]
    fn large_input() {
        let nums: Vec<i32> = (0..30_000).rev().map(|i| i * 33_333).collect();
        let sorted = Solution::sort_jumbled((0..10).collect(), nums.clone());
        let mut expected = nums;
        expected.sort();
        assert_eq!(sorted, expected);
    }
}
