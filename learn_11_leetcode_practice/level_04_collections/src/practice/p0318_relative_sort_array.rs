// ==================================================================
// PRACTICE 0318: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0318_relative_sort_array.md
// Run the tests:          cargo test practice::p0318
// ==================================================================

//! # Problem 0318: Relative Sort Array
//!
//! - **Difficulty:** Easy
//! - **Topics:** rank table in a `Vec<Option<usize>>`, `sort_unstable_by_key` with a tuple key
//! - **Source:** [LeetCode 1122. Relative Sort Array](https://leetcode.com/problems/relative-sort-array/)
//!
//! ## Problem
//!
//! You are given two vectors `arr1` and `arr2`. The values of `arr2` are distinct, and each of
//! them also appears in `arr1` (possibly several times). Sort `arr1` so that:
//!
//! - values that appear in `arr2` come first, in the **same relative order as in `arr2`**
//!   (all copies of `arr2[0]`, then all copies of `arr2[1]`, ...);
//! - values that do not appear in `arr2` come after them, in **ascending** order.
//!
//! Return the sorted vector.
//!
//! Implement `Solution::relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  arr1 = [2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19], arr2 = [2, 1, 4, 3, 9, 6]
//! Output: [2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
//!
//! Input:  arr1 = [28, 6, 22, 8, 44, 17], arr2 = [22, 28, 8, 6]
//! Output: [22, 28, 8, 6, 17, 44]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= arr1.len(), arr2.len() <= 1000`
//! - `0 <= arr1[i], arr2[i] <= 1000`
//! - The values of `arr2` are distinct and all appear in `arr1`.
//!
//! ## Hints
//!
//! 1. Build a table `rank[value] = position of value in arr2` (or "no position").
//! 2. Sort by the key `(rank, value)`, where values without a rank get a rank larger than every
//!    real one, e.g. `arr2.len()`.

pub struct Solution;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::relative_sort_array(
                vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
                vec![2, 1, 4, 3, 9, 6]
            ),
            vec![2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::relative_sort_array(vec![28, 6, 22, 8, 44, 17], vec![22, 28, 8, 6]),
            vec![22, 28, 8, 6, 17, 44]
        );
    }

    #[test]
    fn every_value_ranked() {
        assert_eq!(
            Solution::relative_sort_array(vec![3, 1, 2, 3, 1], vec![3, 2, 1]),
            vec![3, 3, 2, 1, 1]
        );
    }

    #[test]
    fn extreme_values_unranked() {
        assert_eq!(
            Solution::relative_sort_array(vec![1000, 0, 5, 1000, 0], vec![5]),
            vec![5, 0, 0, 1000, 1000]
        );
    }

    #[test]
    fn large_input() {
        let arr1: Vec<i32> = (0..1000).rev().collect();
        let arr2: Vec<i32> = vec![500, 10];
        let result = Solution::relative_sort_array(arr1, arr2);
        assert_eq!(&result[..3], &[500, 10, 0]);
        assert_eq!(result[999], 999);
        assert!(result[2..].windows(2).all(|w| w[0] < w[1]));
    }
}
