// ==================================================================
// PRACTICE 0856: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0856_evaluate_division.md
// Run the tests:          cargo test practice::p0856
// ==================================================================

//! # Problem 0856: Evaluate Division
//!
//! - **Difficulty:** Medium
//! - **Topics:** weighted directed graph with `HashMap<&str, Vec<(&str, f64)>>`, DFS that multiplies
//!   edge weights, `HashSet` of visited names, `f64`
//! - **Source:** [LeetCode 399. Evaluate Division](https://leetcode.com/problems/evaluate-division/)
//!
//! ## Problem
//!
//! You are given equations `equations[i] = [a, b]` with `values[i] = k`, meaning `a / b = k`
//! (variables are strings, all values are positive). Then for each query `[c, d]`, return the value
//! of `c / d` deduced from the equations, or `-1.0` if it cannot be determined. A variable that
//! never appears in any equation is unknown, so even `x / x` is `-1.0` for it. The input never
//! contains contradictions.
//!
//! Implement `Solution::calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  equations = [["a", "b"], ["b", "c"]], values = [2.0, 3.0],
//!         queries = [["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]]
//! Output: [6.0, 0.5, -1.0, 1.0, -1.0]
//! Explanation: a / c = (a / b) * (b / c) = 2 * 3; b / a = 1 / 2; e and x are unknown.
//!
//! Input:  equations = [["a", "b"], ["b", "c"], ["bc", "cd"]], values = [1.5, 2.5, 5.0],
//!         queries = [["a", "c"], ["c", "b"], ["bc", "cd"], ["cd", "bc"]]
//! Output: [3.75, 0.4, 5.0, 0.2]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= equations.len() == values.len() <= 20`; `0.0 < values[i] <= 20.0`
//! - `1 <= queries.len() <= 20`; variable names have 1 to 5 lowercase letters or digits
//! - Answers are compared with a tolerance of `1e-5`.
//!
//! ## Hints
//!
//! 1. Turn every equation `a / b = k` into two weighted edges: `a -> b` with weight `k` and
//!    `b -> a` with weight `1 / k`.
//! 2. `c / d` is the product of the weights along any path from `c` to `d`.
//! 3. Search from `c` (DFS or BFS), carrying the product so far, and keep a visited set to avoid
//!    looping around cycles.

use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pairs(list: &[[&str; 2]]) -> Vec<Vec<String>> {
        list.iter()
            .map(|p| p.iter().map(|s| s.to_string()).collect())
            .collect()
    }

    fn assert_close(got: &[f64], want: &[f64]) {
        assert_eq!(got.len(), want.len());
        for (g, w) in got.iter().zip(want) {
            assert!((g - w).abs() < 1e-5, "got {got:?}, want {want:?}");
        }
    }

    #[test]
    fn example_chain() {
        let got = Solution::calc_equation(
            pairs(&[["a", "b"], ["b", "c"]]),
            vec![2.0, 3.0],
            pairs(&[["a", "c"], ["b", "a"], ["a", "e"], ["a", "a"], ["x", "x"]]),
        );
        assert_close(&got, &[6.0, 0.5, -1.0, 1.0, -1.0]);
    }

    #[test]
    fn example_multi_letter_names() {
        let got = Solution::calc_equation(
            pairs(&[["a", "b"], ["b", "c"], ["bc", "cd"]]),
            vec![1.5, 2.5, 5.0],
            pairs(&[["a", "c"], ["c", "b"], ["bc", "cd"], ["cd", "bc"]]),
        );
        assert_close(&got, &[3.75, 0.4, 5.0, 0.2]);
    }

    #[test]
    fn separate_components() {
        let got = Solution::calc_equation(
            pairs(&[["a", "b"], ["c", "d"]]),
            vec![0.5, 4.0],
            pairs(&[["a", "b"], ["b", "a"], ["a", "c"], ["x", "y"], ["d", "c"]]),
        );
        assert_close(&got, &[0.5, 2.0, -1.0, -1.0, 0.25]);
    }

    #[test]
    fn cycle_in_the_graph() {
        let got = Solution::calc_equation(
            pairs(&[["a", "b"], ["b", "c"], ["c", "a"]]),
            vec![2.0, 2.0, 0.25],
            pairs(&[["a", "c"], ["c", "b"]]),
        );
        assert_close(&got, &[4.0, 0.5]);
    }
}
