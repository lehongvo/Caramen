//! # Problem 0793: Combinations
//!
//! - **Difficulty:** Medium
//! - **Topics:** backtracking with pruning (enough numbers left), inclusive ranges, `Vec::with_capacity`
//! - **Source:** [LeetCode 77. Combinations](https://leetcode.com/problems/combinations/)
//!
//! ## Problem
//!
//! Given two integers `n` and `k`, return all combinations of `k` distinct numbers chosen from
//! `1..=n`. A combination is a set: `[1, 2]` and `[2, 1]` are the same and must appear once. The
//! combinations may be returned in any order.
//!
//! Implement `Solution::combine(n: i32, k: i32) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 4, k = 2
//! Output: [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
//!
//! Input:  n = 1, k = 1
//! Output: [[1]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 20`
//! - `1 <= k <= n`
//!
//! ## Hints
//!
//! 1. Build each combination in increasing order, so every set is produced exactly once.
//! 2. If `current` still needs `need` numbers, the next number can be at most `n - need + 1`;
//!    larger starts cannot be completed. Use that to stop the loop early.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** backtracking with pruning: `backtrack(start)` appends one number `x >= start` to
//! `current` and recurses with `x + 1`, which keeps every combination increasing (so each set
//! appears once). When `current` has `k` numbers, record a copy. If `need` more numbers are
//! required, `x` can go up to `n - need + 1` at most: beyond that there are not enough numbers left
//! to finish, so those branches are never entered. The pruning turns many dead-end calls into none.
//!
//! **Complexity:** `O(k * C(n, k))` time (each of the `C(n, k)` combinations is copied),
//! `O(k)` recursion depth.
//!
//! **Rust concepts:**
//! - `for x in start..=n - need + 1` is an inclusive range whose end is computed once; when the
//!   end is smaller than `start`, the range is simply empty.
//! - `Vec::with_capacity(k)` sizes the shared buffer once.
// @solution-end

pub struct Solution;

// @solution-start remove
fn backtrack(start: i32, n: i32, k: usize, current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if current.len() == k {
        result.push(current.clone());
        return;
    }
    let need = (k - current.len()) as i32;
    for x in start..=n - need + 1 {
        current.push(x);
        backtrack(x + 1, n, k, current, result);
        current.pop();
    }
}
// @solution-end

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        // @solution-start
        let k = k as usize;
        let mut result = Vec::new();
        backtrack(1, n, k, &mut Vec::with_capacity(k), &mut result);
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
    fn example_four_choose_two() {
        assert_eq!(
            normalized(Solution::combine(4, 2)),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ]
        );
    }

    #[test]
    fn example_one_choose_one() {
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
    }

    #[test]
    fn choose_everything() {
        assert_eq!(Solution::combine(5, 5), vec![vec![1, 2, 3, 4, 5]]);
    }

    #[test]
    fn choose_one() {
        assert_eq!(
            normalized(Solution::combine(3, 1)),
            vec![vec![1], vec![2], vec![3]]
        );
    }

    #[test]
    fn counts_match_binomial_coefficients() {
        assert_eq!(Solution::combine(10, 5).len(), 252);
        let big = Solution::combine(16, 8);
        assert_eq!(big.len(), 12_870);
        assert!(
            big.iter()
                .all(|c| c.len() == 8 && c.windows(2).all(|w| w[0] < w[1]))
        );
    }
}
