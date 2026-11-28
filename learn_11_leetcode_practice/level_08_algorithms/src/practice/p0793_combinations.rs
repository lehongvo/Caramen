// ==================================================================
// PRACTICE 0793: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0793_combinations.md
// Run the tests:          cargo test practice::p0793
// ==================================================================

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

pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
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
