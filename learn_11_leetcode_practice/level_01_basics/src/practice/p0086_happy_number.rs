// ==================================================================
// PRACTICE 0086: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0086_happy_number.md
// Run the tests:          cargo test practice::p0086
// ==================================================================

//! # Problem 0086: Happy Number
//!
//! - **Difficulty:** Easy
//! - **Topics:** digit squares, cycle detection with two pointers (Floyd), `loop` with `break` value
//! - **Source:** [LeetCode 202. Happy Number](https://leetcode.com/problems/happy-number/)
//!
//! ## Problem
//!
//! Start with a positive integer and repeatedly replace it by the **sum of the squares of its
//! digits**. If this process eventually reaches `1`, the starting number is **happy**. Otherwise the
//! process loops forever in a cycle that does not contain `1`.
//!
//! Given `n`, return `true` if it is a happy number and `false` otherwise.
//!
//! Implement `Solution::is_happy(n: i32) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 19
//! Output: true
//! Explanation: 1^2 + 9^2 = 82 -> 8^2 + 2^2 = 68 -> 6^2 + 8^2 = 100 -> 1^2 + 0^2 + 0^2 = 1.
//!
//! Input:  n = 2
//! Output: false
//! Explanation: 2 -> 4 -> 16 -> 37 -> 58 -> 89 -> 145 -> 42 -> 20 -> 4 -> ... (a cycle without 1).
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. Write a helper `next(n)` that returns the sum of the squares of the digits.
//! 2. To detect a cycle without remembering every value, move a `slow` value one step at a time and a
//!    `fast` value two steps at a time. If there is a cycle, they will meet.
//! 3. They also "meet" at 1, because `next(1) == 1`. Check which value they met at.

pub struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_happy() {
        assert!(Solution::is_happy(19));
    }

    #[test]
    fn example_unhappy() {
        assert!(!Solution::is_happy(2));
    }

    #[test]
    fn one_and_powers_of_ten_are_happy() {
        assert!(Solution::is_happy(1));
        assert!(Solution::is_happy(10));
        assert!(Solution::is_happy(1_000_000_000));
    }

    #[test]
    fn happy_numbers_up_to_50() {
        let happy: Vec<i32> = (1..=50).filter(|&n| Solution::is_happy(n)).collect();
        assert_eq!(happy, vec![1, 7, 10, 13, 19, 23, 28, 31, 32, 44, 49]);
    }

    #[test]
    fn largest_input() {
        // 2147483647 -> 260 -> 40 -> 16 -> ... joins the 4 -> 16 -> 37 cycle.
        assert!(!Solution::is_happy(i32::MAX));
    }
}
