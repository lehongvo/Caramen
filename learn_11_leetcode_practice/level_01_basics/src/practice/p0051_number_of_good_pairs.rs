// ==================================================================
// PRACTICE 0051: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0051_number_of_good_pairs.md
// Run the tests:          cargo test practice::p0051
// ==================================================================

//! # Problem 0051: Number of Good Pairs
//!
//! - **Difficulty:** Easy
//! - **Topics:** fixed-size array as a counter `[i32; 101]`, nested index loops, `as usize`
//! - **Source:** [LeetCode 1512. Number of Good Pairs](https://leetcode.com/problems/number-of-good-pairs/)
//!
//! ## Problem
//!
//! Given a vector of integers `nums`, a pair of indices `(i, j)` is **good** if `nums[i] == nums[j]`
//! and `i < j`. Return the number of good pairs.
//!
//! Implement `Solution::num_identical_pairs(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 3, 1, 1, 3]
//! Output: 4
//! Explanation: the good pairs are (0, 3), (0, 4), (3, 4) and (2, 5).
//!
//! Input:  nums = [1, 1, 1, 1]
//! Output: 6
//! Explanation: every pair of the four positions is good.
//!
//! Input:  nums = [1, 2, 3]
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 100`
//! - `1 <= nums[i] <= 100`
//!
//! ## Hints
//!
//! 1. The direct approach checks every pair with two nested loops: `for i in 0..n` and `for j in i + 1..n`.
//! 2. Faster: when you reach a value, it forms a good pair with **every earlier copy** of the same value.
//! 3. The values are small (`1..=100`), so an array `[0; 101]` can count how often each value was seen.

pub struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute_force(nums: &[i32]) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == nums[j] {
                    count += 1;
                }
            }
        }
        count
    }

    #[test]
    fn example_mixed() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
    }

    #[test]
    fn example_all_equal() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
    }

    #[test]
    fn example_all_different() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
        assert_eq!(Solution::num_identical_pairs(vec![100]), 0);
    }

    #[test]
    fn largest_input_all_equal() {
        // 100 equal values form 100 * 99 / 2 = 4950 pairs.
        assert_eq!(Solution::num_identical_pairs(vec![100; 100]), 4950);
    }

    #[test]
    fn matches_brute_force() {
        for modulus in 1..=20 {
            let nums: Vec<i32> = (0..100).map(|i| (i * 37 % modulus) + 1).collect();
            assert_eq!(
                Solution::num_identical_pairs(nums.clone()),
                brute_force(&nums)
            );
        }
    }
}
