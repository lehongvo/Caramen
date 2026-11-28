//! # Problem 0309: Array Partition
//!
//! - **Difficulty:** Easy
//! - **Topics:** `sort_unstable`, `slice::chunks_exact`, greedy pairing, `sum`
//! - **Source:** [LeetCode 561. Array Partition](https://leetcode.com/problems/array-partition/)
//!
//! ## Problem
//!
//! You are given a vector `nums` with `2n` integers. Split them into `n` pairs `(a1, b1)`,
//! `(a2, b2)`, ..., `(an, bn)` so that every number is used in exactly one pair, and make the sum
//! `min(a1, b1) + min(a2, b2) + ... + min(an, bn)` as **large** as possible. Return that largest
//! possible sum.
//!
//! Implement `Solution::array_pair_sum(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 4, 3, 2]
//! Output: 4
//! Explanation: the pairs (1, 2) and (3, 4) give min 1 + min 3 = 4, the best possible.
//!
//! Input:  nums = [6, 2, 6, 5, 1, 2]
//! Output: 9
//! Explanation: (1, 2), (2, 5), (6, 6) give 1 + 2 + 6 = 9.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 10^4`, so `nums.len() == 2n`
//! - `-10^4 <= nums[i] <= 10^4`
//!
//! ## Hints
//!
//! 1. The smallest number is always the minimum of its pair. Which partner wastes the least?
//! 2. After sorting, pair each element with its neighbour.
//! 3. `chunks_exact(2)` yields the pairs `[x0, x1]`, `[x2, x3]`, ...
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the smallest value is the minimum of its pair whatever happens, so pairing it with
//! the second smallest "wastes" the least. Repeating the argument, sort the numbers and pair
//! neighbours; the answer is the sum of the elements at even positions of the sorted vector.
//!
//! **Complexity:** `O(n log n)` time for the sort, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - `chunks_exact(2)` guarantees every chunk has exactly two elements, so `pair[0]` is always
//!   valid. (`step_by(2)` on the iterator would work too.)
//! - `sum()` needs to know the result type; here it is inferred from the return type `i32`.
//!   The total fits: at most `10^4` values of absolute value `10^4`.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        // @solution-start
        nums.sort_unstable();
        nums.chunks_exact(2).map(|pair| pair[0]).sum()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::array_pair_sum(vec![1, 4, 3, 2]), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::array_pair_sum(vec![6, 2, 6, 5, 1, 2]), 9);
    }

    #[test]
    fn single_pair_with_negatives() {
        assert_eq!(Solution::array_pair_sum(vec![-3, -7]), -7);
    }

    #[test]
    fn largest_input() {
        // Values 1..=20000: sorted pairs (1, 2), (3, 4), ... sum of odd numbers = 10000^2.
        let nums: Vec<i32> = (1..=20_000).rev().collect();
        assert_eq!(Solution::array_pair_sum(nums), 100_000_000);
    }
}
