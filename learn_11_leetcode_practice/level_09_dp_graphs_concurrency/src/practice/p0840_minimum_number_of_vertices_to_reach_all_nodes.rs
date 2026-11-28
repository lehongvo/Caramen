// ==================================================================
// PRACTICE 0840: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0840_minimum_number_of_vertices_to_reach_all_nodes.md
// Run the tests:          cargo test practice::p0840
// ==================================================================

//! # Problem 0840: Minimum Number of Vertices to Reach All Nodes
//!
//! - **Difficulty:** Medium
//! - **Topics:** in-degree in a DAG, a one-line graph insight, `filter` + `collect`
//! - **Source:** [LeetCode 1557. Minimum Number of Vertices to Reach All Nodes](https://leetcode.com/problems/minimum-number-of-vertices-to-reach-all-nodes/)
//!
//! ## Problem
//!
//! A **directed acyclic graph** has `n` vertices `0..n` and directed edges `edges[i] = [from,
//! to]`. Find the smallest set of vertices from which every vertex of the graph is reachable
//! (a vertex reaches itself). Such a set is unique; return it in any order.
//!
//! Implement `Solution::find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 6, edges = [[0, 1], [0, 2], [2, 5], [3, 4], [4, 2]]
//! Output: [0, 3]
//! Explanation: 0 reaches {0, 1, 2, 5} and 3 reaches {3, 4, 2, 5}.
//!
//! Input:  n = 5, edges = [[0, 1], [2, 1], [3, 1], [1, 4], [2, 4]]
//! Output: [0, 2, 3]
//! ```
//!
//! ## Constraints
//!
//! - `2 <= n <= 10^5`
//! - `1 <= edges.len() <= min(10^5, n * (n - 1) / 2)`
//! - All edges are distinct; the graph has no cycle.
//!
//! ## Hints
//!
//! 1. A vertex with no incoming edge cannot be reached from any other vertex, so it **must** be in
//!    the set.
//! 2. In a DAG, every vertex with an incoming edge can be traced back (following edges backwards)
//!    to some vertex with no incoming edge. So those vertices alone are enough.

pub struct Solution;

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(n: i32, edges: &[[i32; 2]]) -> Vec<i32> {
        let mut got =
            Solution::find_smallest_set_of_vertices(n, edges.iter().map(|e| e.to_vec()).collect());
        got.sort_unstable();
        got
    }

    #[test]
    fn example_six_vertices() {
        assert_eq!(
            run(6, &[[0, 1], [0, 2], [2, 5], [3, 4], [4, 2]]),
            vec![0, 3]
        );
    }

    #[test]
    fn example_five_vertices() {
        assert_eq!(
            run(5, &[[0, 1], [2, 1], [3, 1], [1, 4], [2, 4]]),
            vec![0, 2, 3]
        );
    }

    #[test]
    fn isolated_vertices_are_included() {
        assert_eq!(run(4, &[[0, 1]]), vec![0, 2, 3]);
    }

    #[test]
    fn long_chain_needs_only_the_head() {
        let n = 100_000;
        let edges: Vec<[i32; 2]> = (0..n - 1).map(|i| [i, i + 1]).collect();
        assert_eq!(run(n, &edges), vec![0]);
    }
}
