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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** backtracking + skip equal siblings: sort, then in `backtrack(start, remaining)` loop
//! over `i >= start`:
//! - `break` as soon as `candidates[i] > remaining` (sorted, so everything after is too big);
//! - skip `i` when `i > start && candidates[i] == candidates[i - 1]`: that value was already
//!   tried as the element at this position of the combination;
//! - otherwise choose it and recurse with `i + 1`, so each position is used at most once.
//!
//! Without the skip rule, an input like a hundred `1`s with target `30` would generate the same
//! combination an astronomical number of times; with it, each level tries the value `1` once.
//!
//! **Complexity:** exponential in the worst case, but each distinct combination is produced once;
//! `O(target)` recursion depth.
//!
//! **Rust concepts:**
//! - `candidates.iter().enumerate().skip(start)` plus `candidates[i - 1]` for the duplicate check.
//! - The function parameters are grouped as "input, position, state, output", a common shape for
//!   backtracking helpers.
// @solution-end

pub struct Solution;

// @solution-start remove
fn backtrack(
    candidates: &[i32],
    start: usize,
    remaining: i32,
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if remaining == 0 {
        result.push(current.clone());
        return;
    }
    for (i, &c) in candidates.iter().enumerate().skip(start) {
        if c > remaining {
            break;
        }
        if i > start && c == candidates[i - 1] {
            continue; // this value was already tried at this level
        }
        current.push(c);
        backtrack(candidates, i + 1, remaining - c, current, result); // each position once
        current.pop();
    }
}
// @solution-end

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // @solution-start
        let mut candidates = candidates;
        candidates.sort_unstable();
        let mut result = Vec::new();
        backtrack(&candidates, 0, target, &mut Vec::new(), &mut result);
        result
        // @solution-end
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
