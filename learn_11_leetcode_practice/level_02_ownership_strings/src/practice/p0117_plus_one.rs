// ==================================================================
// PRACTICE 0117: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0117_plus_one.md
// Run the tests:          cargo test practice::p0117
// ==================================================================

//! # Problem 0117: Plus One
//!
//! - **Difficulty:** Easy
//! - **Topics:** owned `Vec` parameter made `mut`, `iter_mut().rev()`, carrying, `Vec::insert`
//! - **Source:** [LeetCode 66. Plus One](https://leetcode.com/problems/plus-one/)
//!
//! ## Problem
//!
//! A non-negative integer is stored as a vector of its decimal digits, most significant digit first
//! (`[1, 2, 3]` means 123). The number has no leading zeros, except for the number 0 itself (`[0]`).
//!
//! Implement `Solution::plus_one(digits: Vec<i32>) -> Vec<i32>`, which returns the digits of the
//! number plus one, in the same format.
//!
//! The function receives the vector **by value**, so it owns it and may modify it and return it,
//! instead of allocating a new one (write the parameter as `mut digits: Vec<i32>`; the type, and so
//! the LeetCode signature, stays the same).
//!
//! ## Examples
//!
//! ```text
//! Input:  digits = [1, 2, 3]
//! Output: [1, 2, 4]
//!
//! Input:  digits = [1, 2, 9, 9]
//! Output: [1, 3, 0, 0]
//!
//! Input:  digits = [9, 9]
//! Output: [1, 0, 0]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= digits.len() <= 100`
//! - `0 <= digits[i] <= 9`
//! - `digits` has no leading zeros (except `[0]`).
//!
//! ## Hints
//!
//! 1. Walk from the last digit to the first with `digits.iter_mut().rev()`.
//! 2. A `9` becomes `0` and the carry continues; any other digit is incremented and you are done.
//! 3. If every digit was `9`, the answer needs one more digit in front: `insert(0, 1)`.

pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_no_carry() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn example_carry_through_nines() {
        assert_eq!(Solution::plus_one(vec![1, 2, 9, 9]), vec![1, 3, 0, 0]);
    }

    #[test]
    fn example_all_nines() {
        assert_eq!(Solution::plus_one(vec![9, 9]), vec![1, 0, 0]);
    }

    #[test]
    fn zero_and_single_nine() {
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }

    #[test]
    fn longest_input() {
        let result = Solution::plus_one(vec![9; 100]);
        assert_eq!(result.len(), 101);
        assert_eq!(result[0], 1);
        assert!(result[1..].iter().all(|&d| d == 0));
    }
}
