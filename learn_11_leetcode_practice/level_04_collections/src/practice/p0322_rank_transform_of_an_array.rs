// ==================================================================
// PRACTICE 0322: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0322_rank_transform_of_an_array.md
// Run the tests:          cargo test practice::p0322
// ==================================================================

//! # Problem 0322: Rank Transform of an Array
//!
//! - **Difficulty:** Easy
//! - **Topics:** `sort_unstable` + `dedup` to get distinct sorted values, `binary_search` as a lookup
//! - **Source:** [LeetCode 1331. Rank Transform of an Array](https://leetcode.com/problems/rank-transform-of-an-array/)
//!
//! ## Problem
//!
//! Replace every element of `arr` by its **rank**:
//!
//! - ranks start at `1`;
//! - a larger element has a larger rank;
//! - equal elements have the same rank;
//! - ranks are as small as possible (no gaps: the distinct values get ranks `1, 2, 3, ...`).
//!
//! Return the vector of ranks.
//!
//! Implement `Solution::array_rank_transform(arr: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  arr = [40, 10, 20, 30]
//! Output: [4, 1, 2, 3]
//!
//! Input:  arr = [100, 100, 100]
//! Output: [1, 1, 1]
//!
//! Input:  arr = [37, 12, 28, 9, 100, 56, 80, 5, 12]
//! Output: [5, 3, 4, 2, 8, 6, 7, 1, 3]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= arr.len() <= 10^5`
//! - `-10^9 <= arr[i] <= 10^9`
//!
//! ## Hints
//!
//! 1. The rank of `x` is one plus the number of **distinct** values smaller than `x`.
//! 2. Sort a copy and remove duplicates; then the index of `x` in that copy is its rank minus one.

pub struct Solution;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::array_rank_transform(vec![40, 10, 20, 30]),
            vec![4, 1, 2, 3]
        );
    }

    #[test]
    fn example_two_all_equal() {
        assert_eq!(
            Solution::array_rank_transform(vec![100, 100, 100]),
            vec![1, 1, 1]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::array_rank_transform(vec![37, 12, 28, 9, 100, 56, 80, 5, 12]),
            vec![5, 3, 4, 2, 8, 6, 7, 1, 3]
        );
    }

    #[test]
    fn empty_and_extremes() {
        assert_eq!(Solution::array_rank_transform(vec![]), Vec::<i32>::new());
        assert_eq!(
            Solution::array_rank_transform(vec![1_000_000_000, -1_000_000_000]),
            vec![2, 1]
        );
    }

    #[test]
    fn large_input_with_duplicates() {
        let arr: Vec<i32> = (0..100_000).map(|i| (i % 1000) * 1000).collect();
        let result = Solution::array_rank_transform(arr);
        assert_eq!(result[0], 1);
        assert_eq!(result[999], 1000);
        assert_eq!(result[99_999], 1000);
    }
}
