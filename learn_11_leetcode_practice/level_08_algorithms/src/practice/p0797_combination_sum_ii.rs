// ==================================================================
// PRACTICE 0797: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0797_combination_sum_ii.md
// Run the tests:          cargo test practice::p0797
// ==================================================================

//! # Problem 0797: Combination Sum II
//!
//! - **Difficulty:** Medium
//! - **Topics:** backtracking, each candidate used once, skipping equal siblings, sorted pruning
//! - **Source:** [LeetCode 40. Combination Sum II](https://leetcode.com/problems/combination-sum-ii/)
//!
//! ## Problem
//!
//! Given a vector `candidates` (which **may contain duplicates**) and a `target`, return every
//! distinct combination of candidates that sums to `target`, where each **position** of
//! `candidates` may be used **at most once**. Combinations with the same values (same
//! multiplicities) count once. The combinations, and the values inside them, may be returned in any
//! order.
//!
//! Implement `Solution::combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  candidates = [10, 1, 2, 7, 6, 1, 5], target = 8
//! Output: [[1, 1, 6], [1, 2, 5], [1, 7], [2, 6]]
//!
//! Input:  candidates = [2, 5, 2, 1, 2], target = 5
//! Output: [[1, 2, 2], [5]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= candidates.len() <= 100`
//! - `1 <= candidates[i] <= 50`
//! - `1 <= target <= 30`
//!
//! ## Hints
//!
//! 1. This is Combination Sum where the recursive call moves past the chosen index (`i + 1`).
//! 2. Duplicates in the input create duplicate combinations. Use the Subsets II rule: at one level,
//!    try each distinct value only once.

pub struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalized(mut combos: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for c in &mut combos {
            c.sort_unstable();
        }
        combos.sort();
        combos
    }

    #[test]
    fn example_target_eight() {
        assert_eq!(
            normalized(Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }

    #[test]
    fn example_target_five() {
        assert_eq!(
            normalized(Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5)),
            vec![vec![1, 2, 2], vec![5]]
        );
    }

    #[test]
    fn no_combination() {
        assert!(Solution::combination_sum2(vec![3], 2).is_empty());
        assert!(Solution::combination_sum2(vec![2, 4, 6], 5).is_empty());
    }

    #[test]
    fn each_position_used_once() {
        // Only one 3 is available, so [3, 3] is not allowed.
        assert_eq!(
            normalized(Solution::combination_sum2(vec![3, 6], 6)),
            vec![vec![6]]
        );
    }

    #[test]
    fn hundred_equal_values() {
        let expected = vec![vec![1; 30]];
        assert_eq!(Solution::combination_sum2(vec![1; 100], 30), expected);
    }
}
