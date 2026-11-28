// ==================================================================
// PRACTICE 0082: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0082_self_dividing_numbers.md
// Run the tests:          cargo test practice::p0082
// ==================================================================

//! # Problem 0082: Self Dividing Numbers
//!
//! - **Difficulty:** Easy
//! - **Topics:** helper function returning `bool`, early `return false`, `for` over an inclusive range
//! - **Source:** [LeetCode 728. Self Dividing Numbers](https://leetcode.com/problems/self-dividing-numbers/)
//!
//! ## Problem
//!
//! A **self-dividing number** is divisible by every one of its digits. For example, `128` is
//! self-dividing because `128 % 1 == 0`, `128 % 2 == 0` and `128 % 8 == 0`. A number containing the
//! digit `0` is never self-dividing.
//!
//! Given two integers `left` and `right`, return all self-dividing numbers in the range
//! `[left, right]` (both included), in increasing order.
//!
//! Implement `Solution::self_dividing_numbers(left: i32, right: i32) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  left = 1, right = 22
//! Output: [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
//!
//! Input:  left = 47, right = 85
//! Output: [48, 55, 66, 77]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= left <= right <= 10^4`
//!
//! ## Hints
//!
//! 1. Write `fn is_self_dividing(n: i32) -> bool` and use it in a loop over `left..=right`.
//! 2. Inside the helper, check the digit `0` **before** computing `n % d` (dividing by zero panics).

pub struct Solution;

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_to_twenty_two() {
        assert_eq!(
            Solution::self_dividing_numbers(1, 22),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        );
    }

    #[test]
    fn example_47_to_85() {
        assert_eq!(
            Solution::self_dividing_numbers(47, 85),
            vec![48, 55, 66, 77]
        );
    }

    #[test]
    fn single_number_ranges() {
        assert_eq!(Solution::self_dividing_numbers(128, 128), vec![128]);
        assert_eq!(Solution::self_dividing_numbers(10, 10), Vec::<i32>::new());
        assert_eq!(Solution::self_dividing_numbers(13, 13), Vec::<i32>::new());
    }

    #[test]
    fn full_range() {
        let all = Solution::self_dividing_numbers(1, 10_000);
        assert_eq!(all.len(), 339);
        assert_eq!(all.last(), Some(&9999));
        assert!(!all.contains(&10_000));
    }
}
