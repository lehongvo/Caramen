// ==================================================================
// PRACTICE 0975: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0975_find_k_th_smallest_pair_distance.md
// Run the tests:          cargo test practice::p0975
// ==================================================================

//! # Problem 0975: Find K-th Smallest Pair Distance
//!
//! - **Difficulty:** Hard
//! - **Topics:** sort + binary search on the distance, counting pairs with two pointers, `sort_unstable`
//! - **Source:** [LeetCode 719. Find K-th Smallest Pair Distance](https://leetcode.com/problems/find-k-th-smallest-pair-distance/)
//!
//! ## Problem
//!
//! The **distance** of a pair `(a, b)` is `|a - b|`. Given an integer array `nums`, consider all
//! `n (n - 1) / 2` pairs `nums[i], nums[j]` with `i < j`, and return the `k`-th smallest distance.
//!
//! Implement `Solution::smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 3, 1], k = 1
//! Output: 0          (distances: 2, 0, 2)
//!
//! Input:  nums = [1, 1, 1], k = 2       Output: 0
//! Input:  nums = [1, 6, 1], k = 3       Output: 5
//! ```
//!
//! ## Constraints
//!
//! - `2 <= nums.len() <= 10^4`
//! - `0 <= nums[i] <= 10^6`
//! - `1 <= k <= n (n - 1) / 2`
//!
//! ## Hints
//!
//! 1. Sort the array; distances do not depend on order.
//! 2. For a distance `d`, count the pairs with distance `<= d` using two pointers: for each right end
//!    `j`, move `i` right while `nums[j] - nums[i] > d`; then `j - i` pairs end at `j`.
//! 3. Binary search the smallest `d` whose count is at least `k`.

pub struct Solution;

impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(nums: &[i32], k: i32) -> i32 {
        let mut distances = Vec::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                distances.push((nums[i] - nums[j]).abs());
            }
        }
        distances.sort_unstable();
        distances[k as usize - 1]
    }

    #[test]
    fn examples() {
        assert_eq!(Solution::smallest_distance_pair(vec![1, 3, 1], 1), 0);
        assert_eq!(Solution::smallest_distance_pair(vec![1, 1, 1], 2), 0);
        assert_eq!(Solution::smallest_distance_pair(vec![1, 6, 1], 3), 5);
    }

    #[test]
    fn two_elements() {
        assert_eq!(
            Solution::smallest_distance_pair(vec![0, 1_000_000], 1),
            1_000_000
        );
        assert_eq!(Solution::smallest_distance_pair(vec![5, 5], 1), 0);
    }

    #[test]
    fn matches_naive_for_every_k() {
        let nums: Vec<i32> = (0..60).map(|i: i32| (i * 7_919) % 1_009).collect();
        let total = (nums.len() * (nums.len() - 1) / 2) as i32;
        for k in (1..=total).step_by(37) {
            assert_eq!(
                Solution::smallest_distance_pair(nums.clone(), k),
                naive(&nums, k),
                "k = {k}"
            );
        }
        assert_eq!(
            Solution::smallest_distance_pair(nums.clone(), total),
            naive(&nums, total)
        );
    }

    #[test]
    fn large_input() {
        let nums: Vec<i32> = (0..10_000).collect();
        // Distance d occurs 10_000 - d times; the 10_000 - 1 smallest are all 1.
        assert_eq!(Solution::smallest_distance_pair(nums.clone(), 9_999), 1);
        assert_eq!(Solution::smallest_distance_pair(nums.clone(), 10_000), 2);
        assert_eq!(Solution::smallest_distance_pair(nums, 49_995_000), 9_999);
    }
}
