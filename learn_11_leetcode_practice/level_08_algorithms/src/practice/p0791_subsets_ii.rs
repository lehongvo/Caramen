// ==================================================================
// PRACTICE 0791: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0791_subsets_ii.md
// Run the tests:          cargo test practice::p0791
// ==================================================================

//! # Problem 0791: Subsets II
//!
//! - **Difficulty:** Medium
//! - **Topics:** backtracking with duplicates, sort first, "skip equal siblings" rule
//! - **Source:** [LeetCode 90. Subsets II](https://leetcode.com/problems/subsets-ii/)
//!
//! ## Problem
//!
//! Given a vector `nums` that **may contain duplicates**, return all of its distinct subsets.
//! Two subsets are the same if they contain the same values with the same multiplicities; each
//! distinct subset must appear exactly once. The subsets, and the values inside them, may be
//! returned in any order.
//!
//! Implement `Solution::subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 2]
//! Output: [[], [1], [1, 2], [1, 2, 2], [2], [2, 2]]
//!
//! Input:  nums = [0]
//! Output: [[], [0]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10`
//! - `-10 <= nums[i] <= 10`
//!
//! ## Hints
//!
//! 1. Start from the Subsets backtracking. Duplicates appear when two equal values can both be
//!    chosen as "the next element" at the same step.
//! 2. Sort the values. At one level of the recursion, choose each distinct value only once: skip
//!    `nums[i]` when it equals `nums[i - 1]` and `i - 1` was also a choice at this level.

pub struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
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
    fn example_with_duplicate() {
        assert_eq!(
            normalized(Solution::subsets_with_dup(vec![1, 2, 2])),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 2],
                vec![2],
                vec![2, 2]
            ]
        );
    }

    #[test]
    fn example_single() {
        assert_eq!(
            normalized(Solution::subsets_with_dup(vec![0])),
            vec![vec![], vec![0]]
        );
    }

    #[test]
    fn all_equal_values() {
        assert_eq!(
            normalized(Solution::subsets_with_dup(vec![2, 2, 2])),
            vec![vec![], vec![2], vec![2, 2], vec![2, 2, 2]]
        );
    }

    #[test]
    fn unsorted_input_with_duplicates() {
        // One value 1 (taken or not) times zero to four 4s: 2 * 5 = 10 subsets.
        let result = normalized(Solution::subsets_with_dup(vec![4, 4, 4, 1, 4]));
        assert_eq!(result.len(), 10);
        let mut deduped = result.clone();
        deduped.dedup();
        assert_eq!(deduped, result);
    }

    #[test]
    fn ten_distinct_values() {
        assert_eq!(Solution::subsets_with_dup((-5..5).collect()).len(), 1024);
    }
}
