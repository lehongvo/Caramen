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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** backtracking + skip equal siblings: sort so that equal values are adjacent. In
//! `backtrack(start)`, the loop picks the next element of the subset among `nums[start..]`. Picking
//! the second of two equal values at the **same level** would produce exactly the subsets already
//! produced by picking the first one, so skip `i` when `i > start && nums[i] == nums[i - 1]`.
//! Equal values can still appear together: after choosing `nums[i]`, the deeper call starts at
//! `i + 1`, where `nums[i + 1]` is a new level and is allowed.
//!
//! **Complexity:** `O(n * 2^n)` time in the worst case (all distinct), `O(n)` recursion depth.
//!
//! **Rust concepts:**
//! - `let mut nums = nums; nums.sort_unstable();` sorts the owned input before borrowing it as
//!   `&nums` for the recursion.
//! - The skip test reads `nums[i - 1]` only when `i > start >= 0`, so the index cannot underflow
//!   (`&&` short-circuits).
// @solution-end

pub struct Solution;

// @solution-start remove
fn backtrack(nums: &[i32], start: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    result.push(current.clone());
    for i in start..nums.len() {
        if i > start && nums[i] == nums[i - 1] {
            continue; // the same value was already tried at this level
        }
        current.push(nums[i]);
        backtrack(nums, i + 1, current, result);
        current.pop();
    }
}
// @solution-end

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // @solution-start
        let mut nums = nums;
        nums.sort_unstable();
        let mut result = Vec::new();
        backtrack(&nums, 0, &mut Vec::new(), &mut result);
        result
        // @solution-end
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
