// ==================================================================
// PRACTICE 0795: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0795_permutations_ii.md
// Run the tests:          cargo test practice::p0795
// ==================================================================

//! # Problem 0795: Permutations II
//!
//! - **Difficulty:** Medium
//! - **Topics:** backtracking with duplicates, sorted input, the "previous equal value must be used" rule
//! - **Source:** [LeetCode 47. Permutations II](https://leetcode.com/problems/permutations-ii/)
//!
//! ## Problem
//!
//! Given a vector `nums` that **may contain duplicates**, return all **distinct** permutations of
//! its values, in any order.
//!
//! Implement `Solution::permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 1, 2]
//! Output: [[1, 1, 2], [1, 2, 1], [2, 1, 1]]
//!
//! Input:  nums = [1, 2, 3]
//! Output: [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 8`
//! - `-10 <= nums[i] <= 10`
//!
//! ## Hints
//!
//! 1. Generating all `n!` orderings and removing duplicates with a set works but wastes time.
//! 2. Sort the values. Treat equal values as if they had to be used **in their original order**:
//!    a copy may only be placed after the previous equal copy has been placed.

pub struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
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
    fn example_with_duplicates() {
        assert_eq!(
            sorted(Solution::permute_unique(vec![1, 1, 2])),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );
    }

    #[test]
    fn example_distinct_values() {
        assert_eq!(Solution::permute_unique(vec![1, 2, 3]).len(), 6);
    }

    #[test]
    fn all_equal() {
        assert_eq!(Solution::permute_unique(vec![2, 2, 2]), vec![vec![2, 2, 2]]);
    }

    #[test]
    fn two_pairs() {
        // 4! / (2! * 2!) = 6
        let perms = sorted(Solution::permute_unique(vec![2, 1, 2, 1]));
        assert_eq!(perms.len(), 6);
        let mut deduped = perms.clone();
        deduped.dedup();
        assert_eq!(deduped, perms);
    }

    #[test]
    fn eight_values() {
        // 8! / (4! * 4!) = 70 distinct arrangements, and 8! = 40_320 for distinct values.
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 1, 1, 2, 2, 2, 2]).len(),
            70
        );
        assert_eq!(Solution::permute_unique((1..=8).collect()).len(), 40_320);
    }
}
