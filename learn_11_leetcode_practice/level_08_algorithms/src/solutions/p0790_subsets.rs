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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** backtracking: `backtrack(start)` records `current` (every path is a subset, the empty
//! one included), then for each index `i >= start` it **chooses** `nums[i]` (push), **explores**
//! with `backtrack(i + 1)`, and **un-chooses** it (pop). Restricting later choices to larger
//! indices means each subset is built in exactly one order, so there are no duplicates. The tree
//! of calls has one node per subset: `2^n` of them.
//!
//! This choose / explore / un-choose shape is the template for every problem of this group.
//!
//! **Complexity:** `O(n * 2^n)` time (each of the `2^n` subsets is copied, up to `n` values each),
//! `O(n)` recursion depth besides the output.
//!
//! **Rust concepts:**
//! - One `current: &mut Vec<i32>` is shared by the whole recursion; `current.clone()` takes a
//!   snapshot when a subset is recorded. Allocating a fresh vector per call would also work but is
//!   slower.
//! - `nums.iter().enumerate().skip(start)` walks the remaining choices with their indices.
// @solution-end

pub struct Solution;

// @solution-start remove
fn backtrack(nums: &[i32], start: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    result.push(current.clone()); // every path is a subset
    for (i, &x) in nums.iter().enumerate().skip(start) {
        current.push(x); // choose
        backtrack(nums, i + 1, current, result); // explore
        current.pop(); // un-choose
    }
}
// @solution-end

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // @solution-start
        let mut result = Vec::with_capacity(1 << nums.len());
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
