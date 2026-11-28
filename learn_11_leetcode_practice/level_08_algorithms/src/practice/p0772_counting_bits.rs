// ==================================================================
// PRACTICE 0772: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0772_counting_bits.md
// Run the tests:          cargo test practice::p0772
// ==================================================================

//! # Problem 0772: Counting Bits
//!
//! - **Difficulty:** Easy
//! - **Topics:** bit manipulation + dynamic programming, `i >> 1`, `i & 1`
//! - **Source:** [LeetCode 338. Counting Bits](https://leetcode.com/problems/counting-bits/)
//!
//! ## Problem
//!
//! Given an integer `n`, return a vector `ans` of length `n + 1` where `ans[i]` is the number of
//! `1` bits in the binary representation of `i`, for every `i` from `0` to `n`.
//!
//! Aim for `O(n)` time overall: do not count the bits of every number from scratch.
//!
//! Implement `Solution::count_bits(n: i32) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 2
//! Output: [0, 1, 1]               (0 -> 0, 1 -> 1, 2 -> 10)
//!
//! Input:  n = 5
//! Output: [0, 1, 1, 2, 1, 2]      (3 -> 11, 4 -> 100, 5 -> 101)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 10^5`
//!
//! ## Hints
//!
//! 1. Shifting `i` right by one drops its last bit. How does the bit count of `i` relate to the bit
//!    count of `i >> 1`?
//! 2. `i >> 1` is smaller than `i`, so its answer is already known when you reach `i`.

pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn example_five() {
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn zero() {
        assert_eq!(Solution::count_bits(0), vec![0]);
    }

    #[test]
    fn powers_of_two_have_one_bit() {
        let ans = Solution::count_bits(1024);
        for k in 0..=10 {
            assert_eq!(ans[1 << k], 1);
        }
        assert_eq!(ans[1023], 10);
    }

    #[test]
    fn largest_input_matches_count_ones() {
        let ans = Solution::count_bits(100_000);
        assert_eq!(ans.len(), 100_001);
        assert!(
            ans.iter()
                .enumerate()
                .all(|(i, &b)| b == i.count_ones() as i32)
        );
    }
}
