//! # Problem 0796: Combination Sum
//!
//! - **Difficulty:** Medium
//! - **Topics:** backtracking with unlimited reuse, sorted pruning with `break`, remaining-target recursion
//! - **Source:** [LeetCode 39. Combination Sum](https://leetcode.com/problems/combination-sum/)
//!
//! ## Problem
//!
//! Given a vector of **distinct** positive integers `candidates` and a positive `target`, return
//! every distinct combination of candidates whose sum is `target`. The same candidate may be used
//! **any number of times**. Two combinations are the same when they use each candidate the same
//! number of times. The combinations, and the values inside them, may be returned in any order.
//!
//! Implement `Solution::combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  candidates = [2, 3, 6, 7], target = 7
//! Output: [[2, 2, 3], [7]]
//!
//! Input:  candidates = [2, 3, 5], target = 8
//! Output: [[2, 2, 2, 2], [2, 3, 3], [3, 5]]
//!
//! Input:  candidates = [2], target = 1
//! Output: []
//! ```
//!
//! ## Constraints
//!
//! - `1 <= candidates.len() <= 30`
//! - `2 <= candidates[i] <= 40`, all distinct
//! - `1 <= target <= 40`
//!
//! ## Hints
//!
//! 1. Choose candidates in non-decreasing index order to avoid producing the same multiset twice.
//! 2. Because a candidate can be reused, the recursive call may start again at the **same** index.
//! 3. After sorting, once a candidate is larger than what remains, all later ones are too: stop.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** backtracking on the remaining target: sort the candidates. `backtrack(start,
//! remaining)` tries each candidate `c = candidates[i]` with `i >= start`: if `c > remaining`, this
//! and every larger candidate overshoot, so `break`; otherwise push `c` and recurse with
//! `(i, remaining - c)`. Passing `i` (not `i + 1`) allows reusing `c`; never going back to smaller
//! indices keeps each combination in one canonical order, so there are no duplicates. A remaining
//! target of `0` means `current` is a solution.
//!
//! **Complexity:** exponential in the worst case (bounded by the number of combinations, which the
//! constraints keep small); `O(target / min(candidates))` recursion depth.
//!
//! **Rust concepts:**
//! - `break` inside the `for` loop is the sorted-pruning step: it relies on the slice being sorted.
//! - `remaining - c` never goes negative because of the `break` test, so `i32` is enough.
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
            break; // sorted: every later candidate is larger too
        }
        current.push(c);
        backtrack(candidates, i, remaining - c, current, result); // `i`: c may be reused
        current.pop();
    }
}
// @solution-end

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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
    fn example_target_seven() {
        assert_eq!(
            normalized(Solution::combination_sum(vec![2, 3, 6, 7], 7)),
            vec![vec![2, 2, 3], vec![7]]
        );
    }

    #[test]
    fn example_target_eight() {
        assert_eq!(
            normalized(Solution::combination_sum(vec![2, 3, 5], 8)),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
    }

    #[test]
    fn example_impossible() {
        assert!(Solution::combination_sum(vec![2], 1).is_empty());
    }

    #[test]
    fn unsorted_candidates() {
        assert_eq!(
            normalized(Solution::combination_sum(vec![8, 7, 4, 3], 11)),
            vec![vec![3, 4, 4], vec![3, 8], vec![4, 7]]
        );
    }

    #[test]
    fn many_ways_with_small_candidates() {
        let result = normalized(Solution::combination_sum(vec![2, 3], 12));
        // 2*6, 2*3 + 3*2, 3*4
        assert_eq!(
            result,
            vec![
                vec![2, 2, 2, 2, 2, 2],
                vec![2, 2, 2, 3, 3],
                vec![3, 3, 3, 3]
            ]
        );
        let all = Solution::combination_sum((2..=40).collect(), 40);
        assert!(all.iter().all(|c| c.iter().sum::<i32>() == 40));
        assert!(all.contains(&vec![40]));
    }
}
