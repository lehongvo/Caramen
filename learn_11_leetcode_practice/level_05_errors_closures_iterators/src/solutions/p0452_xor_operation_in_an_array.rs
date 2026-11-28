//! # Problem 0452: XOR Operation in an Array
//!
//! - **Difficulty:** Easy
//! - **Topics:** ranges as iterators, `map`, `fold` with `^`, `reduce`
//! - **Source:** [LeetCode 1486. XOR Operation in an Array](https://leetcode.com/problems/xor-operation-in-an-array/)
//!
//! ## Problem
//!
//! Consider the array `nums` of length `n` defined by `nums[i] = start + 2 * i` for
//! `i = 0, 1, ..., n - 1`. Return the bitwise XOR of all its elements:
//! `nums[0] ^ nums[1] ^ ... ^ nums[n - 1]`.
//!
//! Implement `Solution::xor_operation(n: i32, start: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 5, start = 0
//! Output: 8
//! Explanation: nums = [0, 2, 4, 6, 8] and 0 ^ 2 ^ 4 ^ 6 ^ 8 = 8.
//!
//! Input:  n = 4, start = 3
//! Output: 8
//! Explanation: nums = [3, 5, 7, 9] and 3 ^ 5 ^ 7 ^ 9 = 8.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 1000`
//! - `0 <= start <= 1000`
//!
//! ## Hints
//!
//! 1. You do not need to build the array: the range `0..n` mapped with `|i| start + 2 * i`
//!    produces the elements one by one.
//! 2. `fold(initial, |acc, x| ...)` combines all elements into one value; `0` is the neutral
//!    element of XOR (`x ^ 0 == x`).
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** generate the elements lazily from a range and fold them with XOR, starting from `0`.
//!
//! **Complexity:** `O(n)` time, `O(1)` space (no array is allocated).
//!
//! **Rust concepts:**
//! - A range such as `0..n` is an iterator, so `map` can turn indexes into values.
//! - `fold(0, |acc, x| acc ^ x)` is the general "combine everything" consumer. `reduce(|a, b| a ^ b)`
//!   would work too, but returns an `Option` because the iterator might be empty.
//! - `i32` implements `BitXor`, so `^` works directly on the numbers.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        // @solution-start
        (0..n).map(|i| start + 2 * i).fold(0, |acc, x| acc ^ x)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_start_zero() {
        assert_eq!(Solution::xor_operation(5, 0), 8);
    }

    #[test]
    fn example_start_three() {
        assert_eq!(Solution::xor_operation(4, 3), 8);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::xor_operation(1, 7), 7);
        assert_eq!(Solution::xor_operation(1, 0), 0);
    }

    #[test]
    fn largest_input_matches_a_loop() {
        let mut expected = 0;
        for i in 0..1000 {
            expected ^= 1000 + 2 * i;
        }
        assert_eq!(Solution::xor_operation(1000, 1000), expected);
        assert_eq!(Solution::xor_operation(2, 5), 5 ^ 7);
    }
}
