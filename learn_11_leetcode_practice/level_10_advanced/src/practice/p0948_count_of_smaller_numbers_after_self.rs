// ==================================================================
// PRACTICE 0948: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0948_count_of_smaller_numbers_after_self.md
// Run the tests:          cargo test practice::p0948
// ==================================================================

//! # Problem 0948: Count of Smaller Numbers After Self
//!
//! - **Difficulty:** Hard
//! - **Topics:** Fenwick tree over values, coordinate compression with `sort_unstable` + `dedup` + `binary_search`, scanning from the right
//! - **Source:** [LeetCode 315. Count of Smaller Numbers After Self](https://leetcode.com/problems/count-of-smaller-numbers-after-self/)
//!
//! ## Problem
//!
//! Given an integer array `nums`, return a vector `counts` of the same length where `counts[i]` is
//! the number of elements to the **right** of position `i` that are **strictly smaller** than
//! `nums[i]`.
//!
//! Implement `Solution::count_smaller(nums: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [5, 2, 6, 1]
//! Output: [2, 1, 1, 0]
//! (5 has 2 and 1 after it, 2 has 1, 6 has 1, 1 has nothing.)
//!
//! Input:  nums = [-1, -1]
//! Output: [0, 0]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `-10^4 <= nums[i] <= 10^4`
//!
//! ## Hints
//!
//! 1. Walk from right to left, keeping a multiset of the values already seen.
//! 2. "How many seen values are smaller than `x`" is a prefix count over values: a Fenwick tree
//!    indexed by value rank answers it in `O(log n)`.
//! 3. Map each value to its rank among the distinct sorted values (coordinate compression) so the
//!    tree has at most `n` slots.

pub struct Solution;

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(nums: &[i32]) -> Vec<i32> {
        (0..nums.len())
            .map(|i| nums[i + 1..].iter().filter(|&&x| x < nums[i]).count() as i32)
            .collect()
    }

    #[test]
    fn examples() {
        assert_eq!(Solution::count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
        assert_eq!(Solution::count_smaller(vec![-1]), vec![0]);
        assert_eq!(Solution::count_smaller(vec![-1, -1]), vec![0, 0]);
    }

    #[test]
    fn duplicates_are_not_smaller() {
        assert_eq!(Solution::count_smaller(vec![2, 2, 1, 2]), vec![1, 1, 0, 0]);
        assert_eq!(
            Solution::count_smaller(vec![10_000, -10_000, 0]),
            vec![2, 0, 0]
        );
    }

    #[test]
    fn matches_naive_on_pseudo_random_input() {
        let nums: Vec<i32> = (0..500).map(|i| (i * 7_919 % 1_001) - 500).collect();
        assert_eq!(Solution::count_smaller(nums.clone()), naive(&nums));
    }

    #[test]
    fn large_decreasing_input() {
        let n = 100_000;
        let nums: Vec<i32> = (0..n).map(|i| 10_000 - (i % 20_001)).collect();
        let counts = Solution::count_smaller(nums);
        assert_eq!(counts.len(), n as usize);
        assert_eq!(counts[n as usize - 1], 0);
        let sorted_desc: Vec<i32> = (0..1_000).rev().collect();
        let counts = Solution::count_smaller(sorted_desc);
        assert_eq!(counts[0], 999);
        assert_eq!(counts[999], 0);
    }
}
