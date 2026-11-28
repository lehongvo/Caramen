// ==================================================================
// PRACTICE 0858: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0858_path_with_maximum_probability.md
// Run the tests:          cargo test practice::p0858
// ==================================================================

//! # Problem 0858: Path with Maximum Probability
//!
//! - **Difficulty:** Medium
//! - **Topics:** Dijkstra on products (maximize), `f64` in a `BinaryHeap` via a newtype that
//!   implements `Ord` with `f64::total_cmp`
//! - **Source:** [LeetCode 1514. Path with Maximum Probability](https://leetcode.com/problems/path-with-maximum-probability/)
//!
//! ## Problem
//!
//! An undirected graph has `n` nodes `0..n`. `edges[i] = [a, b]` is an edge whose traversal
//! succeeds with probability `succ_prob[i]`. The success probability of a path is the product of
//! the probabilities of its edges. Return the maximum success probability of a path from
//! `start_node` to `end_node`, or `0.0` if there is no path.
//!
//! Implement `Solution::max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 3, edges = [[0, 1], [1, 2], [0, 2]], succ_prob = [0.5, 0.5, 0.2],
//!         start_node = 0, end_node = 2
//! Output: 0.25
//! Explanation: 0 -> 1 -> 2 gives 0.5 * 0.5 = 0.25, better than the direct 0.2.
//!
//! Input:  n = 3, edges = [[0, 1], [1, 2], [0, 2]], succ_prob = [0.5, 0.5, 0.3],
//!         start_node = 0, end_node = 2
//! Output: 0.3
//!
//! Input:  n = 3, edges = [[0, 1]], succ_prob = [0.5], start_node = 0, end_node = 2
//! Output: 0.0
//! ```
//!
//! ## Constraints
//!
//! - `2 <= n <= 10^4`; `0 <= start_node, end_node < n`, `start_node != end_node`
//! - `0 <= edges.len() <= 2 * 10^4`; `0 <= succ_prob[i] <= 1`
//! - Answers within `1e-5` of the true value are accepted.
//!
//! ## Hints
//!
//! 1. Multiplying by a probability never increases the product, just as adding a non-negative
//!    weight never decreases a sum. So Dijkstra works with "largest product first".
//! 2. `f64` is only `PartialOrd` (because of `NaN`), so it cannot go into a `BinaryHeap` directly.
//!    Wrap it in a struct and implement `Ord` yourself.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start_node: i32,
        end_node: i32,
    ) -> f64 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(n: i32, edges: &[[i32; 2]], probs: &[f64], start: i32, end: i32) -> f64 {
        let e = edges.iter().map(|x| x.to_vec()).collect();
        Solution::max_probability(n, e, probs.to_vec(), start, end)
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn example_two_hops_win() {
        let p = run(3, &[[0, 1], [1, 2], [0, 2]], &[0.5, 0.5, 0.2], 0, 2);
        assert!(close(p, 0.25));
    }

    #[test]
    fn example_direct_edge_wins() {
        let p = run(3, &[[0, 1], [1, 2], [0, 2]], &[0.5, 0.5, 0.3], 0, 2);
        assert!(close(p, 0.3));
    }

    #[test]
    fn example_no_path() {
        assert_eq!(run(3, &[[0, 1]], &[0.5], 0, 2), 0.0);
    }

    #[test]
    fn zero_probability_edges_and_reverse_direction() {
        assert_eq!(run(2, &[[0, 1]], &[0.0], 0, 1), 0.0);
        assert!(close(run(2, &[[0, 1]], &[0.75], 1, 0), 0.75));
    }

    #[test]
    fn long_chain_versus_shortcut() {
        // A chain of 100 edges with probability 0.99 (about 0.366) beats a shortcut of 0.3.
        let n = 101;
        let mut edges: Vec<[i32; 2]> = (0..100).map(|i| [i, i + 1]).collect();
        let mut probs = vec![0.99; 100];
        edges.push([0, 100]);
        probs.push(0.3);
        let p = run(n, &edges, &probs, 0, 100);
        assert!(close(p, 0.99_f64.powi(100)));
    }
}
