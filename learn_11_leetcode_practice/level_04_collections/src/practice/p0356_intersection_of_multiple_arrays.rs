// ==================================================================
// PRACTICE 0356: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0356_intersection_of_multiple_arrays.md
// Run the tests:          cargo test practice::p0356
// ==================================================================

//! # Problem 0356: Intersection of Multiple Arrays
//!
//! - **Difficulty:** Easy
//! - **Topics:** `BTreeSet::retain`, `split_first`, sorted output from a set
//! - **Source:** [LeetCode 2248. Intersection of Multiple Arrays](https://leetcode.com/problems/intersection-of-multiple-arrays/)
//!
//! ## Problem
//!
//! You are given a list `nums` of non-empty vectors; each inner vector contains **distinct**
//! positive integers. Return, in **ascending order**, the integers that appear in **every** inner
//! vector.
//!
//! Implement `Solution::intersection(nums: Vec<Vec<i32>>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [[3, 1, 2, 4, 5], [1, 2, 3, 4], [3, 4, 5, 6]]
//! Output: [3, 4]
//!
//! Input:  nums = [[1, 2, 3], [4, 5, 6]]
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 1000`
//! - `1 <= sum of nums[i].len() <= 1000`
//! - `1 <= nums[i][j] <= 1000`, values within one `nums[i]` are distinct
//!
//! ## Hints
//!
//! 1. Start with all values of the first vector in a `BTreeSet`.
//! 2. For every other vector, keep only the values it contains: `set.retain(|x| ...)`.
//! 3. A `BTreeSet` already iterates in ascending order.

use std::collections::{BTreeSet, HashSet};

pub struct Solution;

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::intersection(vec![
                vec![3, 1, 2, 4, 5],
                vec![1, 2, 3, 4],
                vec![3, 4, 5, 6]
            ]),
            vec![3, 4]
        );
    }

    #[test]
    fn example_two_empty() {
        assert_eq!(
            Solution::intersection(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn single_row_is_sorted() {
        assert_eq!(Solution::intersection(vec![vec![9, 2, 7]]), vec![2, 7, 9]);
    }

    #[test]
    fn many_rows() {
        // 100 rows of 10 distinct values (11..=810 are spread over the rows); every row also
        // contains 1000 and 7.
        let nums: Vec<Vec<i32>> = (0..100)
            .map(|r| {
                let mut row: Vec<i32> = (1..=8).map(|k| 10 + r * 8 + k).collect();
                row.push(1000);
                row.push(7);
                row
            })
            .collect();
        assert_eq!(Solution::intersection(nums), vec![7, 1000]);
    }
}
