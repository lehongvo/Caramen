// ==================================================================
// PRACTICE 0794: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0794_permutations.md
// Run the tests:          cargo test practice::p0794
// ==================================================================

//! # Problem 0794: Permutations
//!
//! - **Difficulty:** Medium
//! - **Topics:** backtracking with a `used` mask, `Vec<bool>`, recursion depth `n`
//! - **Source:** [LeetCode 46. Permutations](https://leetcode.com/problems/permutations/)
//!
//! ## Problem
//!
//! Given a vector `nums` of **distinct** integers, return all possible permutations (orderings) of
//! its values, in any order.
//!
//! Implement `Solution::permute(nums: Vec<i32>) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 3]
//! Output: [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
//!
//! Input:  nums = [0, 1]
//! Output: [[0, 1], [1, 0]]
//!
//! Input:  nums = [1]
//! Output: [[1]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 6`
//! - `-10 <= nums[i] <= 10`, all distinct
//!
//! ## Hints
//!
//! 1. Unlike subsets and combinations, order matters: any unused value may come next.
//! 2. Keep a `used` flag per position, set it before recursing and clear it afterwards.

pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut perms: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        perms.sort();
        perms
    }

    #[test]
    fn example_three_values() {
        assert_eq!(
            sorted(Solution::permute(vec![1, 2, 3])),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }

    #[test]
    fn example_two_values() {
        assert_eq!(
            sorted(Solution::permute(vec![0, 1])),
            vec![vec![0, 1], vec![1, 0]]
        );
    }

    #[test]
    fn example_single_value() {
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }

    #[test]
    fn six_values_give_720_distinct_permutations() {
        let perms = sorted(Solution::permute(vec![-10, -3, 0, 4, 7, 10]));
        assert_eq!(perms.len(), 720);
        let mut deduped = perms.clone();
        deduped.dedup();
        assert_eq!(deduped.len(), 720);
        assert!(perms.iter().all(|p| {
            let mut s = p.clone();
            s.sort_unstable();
            s == vec![-10, -3, 0, 4, 7, 10]
        }));
    }
}
