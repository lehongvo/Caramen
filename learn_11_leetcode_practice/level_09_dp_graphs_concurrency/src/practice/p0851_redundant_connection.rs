// ==================================================================
// PRACTICE 0851: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0851_redundant_connection.md
// Run the tests:          cargo test practice::p0851
// ==================================================================

//! # Problem 0851: Redundant Connection
//!
//! - **Difficulty:** Medium
//! - **Topics:** union-find (disjoint set union) as a small struct with methods, path halving,
//!   union by size
//! - **Source:** [LeetCode 684. Redundant Connection](https://leetcode.com/problems/redundant-connection/)
//!
//! ## Problem
//!
//! A tree with nodes `1..=n` had **one extra edge** added, so the graph now has `n` undirected
//! edges and exactly one cycle. `edges[i] = [a, b]` lists them in a given order. Return an edge
//! that can be removed so the remaining edges form a tree again. If several edges qualify (every
//! edge on the cycle does), return the one that appears **last** in `edges`.
//!
//! Implement `Solution::find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  edges = [[1, 2], [1, 3], [2, 3]]
//! Output: [2, 3]
//!
//! Input:  edges = [[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]]
//! Output: [1, 4]
//! Explanation: the cycle is 1-2-3-4-1; [1, 4] is its last edge in the input.
//! ```
//!
//! ## Constraints
//!
//! - `3 <= n == edges.len() <= 1000`
//! - `1 <= a < b <= n`; no repeated edges; the graph is connected.
//!
//! ## Hints
//!
//! 1. Add the edges one by one. The first edge whose two ends are **already** connected closes the
//!    cycle, and it is the last cycle edge in input order.
//! 2. "Are `a` and `b` already connected?" is the question a union-find structure answers in
//!    almost constant time.
//! 3. `find(x)` follows parent links to the representative of `x`'s set; `union(a, b)` links two
//!    representatives and reports whether they were different.

pub struct Solution;

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(edges: &[[i32; 2]]) -> Vec<i32> {
        Solution::find_redundant_connection(edges.iter().map(|e| e.to_vec()).collect())
    }

    #[test]
    fn example_triangle() {
        assert_eq!(run(&[[1, 2], [1, 3], [2, 3]]), vec![2, 3]);
    }

    #[test]
    fn example_square_with_tail() {
        assert_eq!(run(&[[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]]), vec![1, 4]);
    }

    #[test]
    fn cycle_edge_in_the_middle() {
        assert_eq!(run(&[[1, 2], [2, 3], [1, 3], [3, 4]]), vec![1, 3]);
    }

    #[test]
    fn large_ring() {
        let mut edges: Vec<[i32; 2]> = (1..1000).map(|i| [i, i + 1]).collect();
        edges.push([1, 1000]);
        assert_eq!(run(&edges), vec![1, 1000]);
    }

    #[test]
    fn ring_closed_early() {
        // The cycle 1-2-4-3-1 closes with [1, 3] before the tail edges are added.
        let edges = [[3, 4], [1, 2], [2, 4], [1, 3], [4, 5], [5, 6]];
        assert_eq!(run(&edges), vec![1, 3]);
    }
}
