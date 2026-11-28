//! # Problem 0323: Longest Subsequence With Limited Sum
//!
//! - **Difficulty:** Easy
//! - **Topics:** sort, in-place prefix sums, `partition_point` per query
//! - **Source:** [LeetCode 2389. Longest Subsequence With Limited Sum](https://leetcode.com/problems/longest-subsequence-with-limited-sum/)
//!
//! ## Problem
//!
//! You are given a vector `nums` of positive integers and a vector `queries`. For each query `q`,
//! find the largest number of elements you can pick from `nums` (each element at most once, in any
//! positions) so that their sum is **at most** `q`. Return a vector with one answer per query, in
//! the same order as `queries`.
//!
//! (LeetCode phrases this with subsequences; since only the sum matters, the order of the chosen
//! elements is irrelevant.)
//!
//! Implement `Solution::answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [4, 5, 2, 1], queries = [3, 10, 21]
//! Output: [2, 3, 4]
//! Explanation: sum <= 3 allows two elements {2, 1}; sum <= 10 allows three, e.g. {5, 4, 1};
//!              sum <= 21 allows all four (their sum is 12).
//!
//! Input:  nums = [2, 3, 4, 5], queries = [1]
//! Output: [0]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len(), queries.len() <= 1000`
//! - `1 <= nums[i], queries[i] <= 10^6`
//!
//! ## Hints
//!
//! 1. To pick as many elements as possible, always pick the smallest ones.
//! 2. After sorting, `prefix[i]` (the sum of the `i + 1` smallest values) is increasing, so a binary
//!    search finds how many prefixes fit under `q`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** sort `nums` and turn it into prefix sums in place. The answer for `q` is the number of
//! prefix sums that are `<= q`, which `partition_point` finds by binary search because the prefix
//! sums are strictly increasing (all values are positive).
//!
//! **Complexity:** `O((n + m) log n)` time, `O(1)` extra space besides the output.
//!
//! **Rust concepts:**
//! - In-place prefix sums: `for i in 1..nums.len() { nums[i] += nums[i - 1]; }`.
//! - The largest prefix sum is `1000 * 10^6 = 10^9`, which still fits in `i32`.
//! - Mapping over `queries` with `into_iter()` consumes the query vector; the result has the same
//!   order.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn answer_queries(mut nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        // @solution-start
        nums.sort_unstable();
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        queries
            .into_iter()
            .map(|q| nums.partition_point(|&sum| sum <= q) as i32)
            .collect()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::answer_queries(vec![4, 5, 2, 1], vec![3, 10, 21]),
            vec![2, 3, 4]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(Solution::answer_queries(vec![2, 3, 4, 5], vec![1]), vec![0]);
    }

    #[test]
    fn exact_sums_count() {
        assert_eq!(
            Solution::answer_queries(vec![1, 2, 3], vec![1, 3, 6, 5]),
            vec![1, 2, 3, 2]
        );
    }

    #[test]
    fn largest_values() {
        let nums = vec![1_000_000; 1000];
        let queries = vec![999_999, 1_000_000, 500_000_000, 1_000_000];
        assert_eq!(Solution::answer_queries(nums, queries), vec![0, 1, 500, 1]);
    }
}
