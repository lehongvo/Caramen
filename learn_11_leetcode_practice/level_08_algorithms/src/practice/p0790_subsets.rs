// ==================================================================
// PRACTICE 0790: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0790_subsets.md
// Run the tests:          cargo test practice::p0790
// ==================================================================

//! # Problem 0790: Subsets
//!
//! - **Difficulty:** Medium
//! - **Topics:** backtracking (choose / explore / un-choose), recursion with `&mut Vec`, `clone` of the path
//! - **Source:** [LeetCode 78. Subsets](https://leetcode.com/problems/subsets/)
//!
//! ## Problem
//!
//! Given a vector `nums` of **distinct** integers, return all of its subsets (the power set). The
//! result must not contain the same subset twice. The subsets, and the values inside each subset,
//! may be returned in any order.
//!
//! Implement `Solution::subsets(nums: Vec<i32>) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 3]
//! Output: [[], [1], [1, 2], [1, 2, 3], [1, 3], [2], [2, 3], [3]]
//!
//! Input:  nums = [0]
//! Output: [[], [0]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10`
//! - `-10 <= nums[i] <= 10`, all distinct
//!
//! ## Hints
//!
//! 1. Build subsets in increasing index order: after choosing `nums[i]`, only choose later
//!    elements. Then every subset is generated exactly once.
//! 2. Keep one `current` vector: push an element, recurse, then pop it to undo the choice.
//! 3. Every partial path is itself a subset, so record it at the start of each call.

pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalized(mut sets: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for s in &mut sets {
            s.sort_unstable();
        }
        sets.sort();
        sets
    }

    #[test]
    fn example_three_values() {
        assert_eq!(
            normalized(Solution::subsets(vec![1, 2, 3])),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3]
            ]
        );
    }

    #[test]
    fn example_single_value() {
        assert_eq!(
            normalized(Solution::subsets(vec![0])),
            vec![vec![], vec![0]]
        );
    }

    #[test]
    fn negative_values() {
        assert_eq!(
            normalized(Solution::subsets(vec![-1, 5])),
            vec![vec![], vec![-1], vec![-1, 5], vec![5]]
        );
    }

    #[test]
    fn ten_values_give_1024_distinct_subsets() {
        let result = normalized(Solution::subsets((1..=10).collect()));
        assert_eq!(result.len(), 1024);
        let mut deduped = result.clone();
        deduped.dedup();
        assert_eq!(deduped.len(), 1024);
        assert_eq!(result.iter().filter(|s| s.len() == 5).count(), 252); // C(10, 5)
    }
}
