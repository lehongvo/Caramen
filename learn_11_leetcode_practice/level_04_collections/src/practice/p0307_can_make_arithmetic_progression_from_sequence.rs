// ==================================================================
// PRACTICE 0307: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0307_can_make_arithmetic_progression_from_sequence.md
// Run the tests:          cargo test practice::p0307
// ==================================================================

//! # Problem 0307: Can Make Arithmetic Progression From Sequence
//!
//! - **Difficulty:** Easy
//! - **Topics:** `sort_unstable`, `slice::windows`, `Iterator::all`
//! - **Source:** [LeetCode 1502. Can Make Arithmetic Progression From Sequence](https://leetcode.com/problems/can-make-arithmetic-progression-from-sequence/)
//!
//! ## Problem
//!
//! A sequence is an **arithmetic progression** when the difference between every two consecutive
//! elements is the same (for example `1, 4, 7, 10`, with difference `3`).
//!
//! Given a vector `arr`, return `true` if its elements can be **rearranged** into an arithmetic
//! progression, and `false` otherwise.
//!
//! Implement `Solution::can_make_arithmetic_progression(arr: Vec<i32>) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  arr = [3, 5, 1]
//! Output: true    (reorder to [1, 3, 5] or [5, 3, 1]: difference 2 or -2)
//!
//! Input:  arr = [1, 2, 4]
//! Output: false
//! ```
//!
//! ## Constraints
//!
//! - `2 <= arr.len() <= 1000`
//! - `-10^6 <= arr[i] <= 10^6`
//!
//! ## Hints
//!
//! 1. If an arrangement exists, the sorted order is one of them.
//! 2. After sorting, look at every pair of neighbours with `windows(2)`.

pub struct Solution;

impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert!(Solution::can_make_arithmetic_progression(vec![3, 5, 1]));
    }

    #[test]
    fn example_two() {
        assert!(!Solution::can_make_arithmetic_progression(vec![1, 2, 4]));
    }

    #[test]
    fn two_elements_always_work() {
        assert!(Solution::can_make_arithmetic_progression(vec![7, -3]));
    }

    #[test]
    fn constant_sequence_has_step_zero() {
        assert!(Solution::can_make_arithmetic_progression(vec![4, 4, 4]));
        assert!(!Solution::can_make_arithmetic_progression(vec![4, 4, 5]));
    }

    #[test]
    fn large_shuffled_progression() {
        let mut arr: Vec<i32> = (0..1000).map(|i| -1_000_000 + i * 2000).collect();
        arr.reverse();
        arr.swap(10, 500);
        assert!(Solution::can_make_arithmetic_progression(arr.clone()));
        arr[0] += 1;
        assert!(!Solution::can_make_arithmetic_progression(arr));
    }
}
