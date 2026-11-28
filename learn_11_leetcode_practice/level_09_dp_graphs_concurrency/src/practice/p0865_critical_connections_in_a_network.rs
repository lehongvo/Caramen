// ==================================================================
// PRACTICE 0865: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0865_critical_connections_in_a_network.md
// Run the tests:          cargo test practice::p0865
// ==================================================================

//! # Problem 0865: Critical Connections in a Network
//!
//! - **Difficulty:** Hard
//! - **Topics:** bridges with Tarjan's low-link values, iterative DFS with a `(node, parent edge,
//!   next index)` stack, edge ids to handle the parent edge
//! - **Source:** [LeetCode 1192. Critical Connections in a Network](https://leetcode.com/problems/critical-connections-in-a-network/)
//!
//! ## Problem
//!
//! There are `n` servers `0..n` connected by undirected `connections[i] = [a, b]`; the network is
//! connected. A **critical connection** is one whose removal disconnects some servers from others.
//! Return all critical connections, in any order (each as `[a, b]` in either orientation).
//!
//! Implement `Solution::critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 4, connections = [[0, 1], [1, 2], [2, 0], [1, 3]]
//! Output: [[1, 3]]
//! Explanation: 0, 1, 2 form a cycle; only the link to server 3 is critical.
//!
//! Input:  n = 2, connections = [[0, 1]]
//! Output: [[0, 1]]
//! ```
//!
//! ## Constraints
//!
//! - `2 <= n <= 10^5`; `n - 1 <= connections.len() <= 10^5`
//! - `0 <= a, b < n`, `a != b`; no repeated connections; the network is connected.
//!
//! ## Hints
//!
//! 1. Removing every edge one at a time and testing connectivity is `O(E * (V + E))`: too slow.
//! 2. In a DFS, give each node a discovery time `disc`. Let `low[u]` be the smallest discovery time
//!    reachable from `u`'s DFS subtree using at most one edge that is not a tree edge.
//! 3. A tree edge `parent -> child` is a bridge exactly when `low[child] > disc[parent]`: nothing in
//!    the child's subtree can climb back to the parent or above.

pub struct Solution;

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the solution and normalizes the answer: each pair sorted, then the list sorted.
    fn run(n: i32, edges: &[[i32; 2]]) -> Vec<Vec<i32>> {
        let mut got = Solution::critical_connections(n, edges.iter().map(|e| e.to_vec()).collect());
        for pair in &mut got {
            pair.sort_unstable();
        }
        got.sort();
        got
    }

    #[test]
    fn example_triangle_with_tail() {
        assert_eq!(run(4, &[[0, 1], [1, 2], [2, 0], [1, 3]]), vec![vec![1, 3]]);
    }

    #[test]
    fn example_single_link() {
        assert_eq!(run(2, &[[0, 1]]), vec![vec![0, 1]]);
    }

    #[test]
    fn cycle_has_no_bridge() {
        assert!(run(5, &[[0, 1], [1, 2], [2, 3], [3, 4], [4, 0]]).is_empty());
    }

    #[test]
    fn two_triangles_joined_by_a_bridge() {
        let e = [[0, 1], [1, 2], [2, 0], [2, 3], [3, 4], [4, 5], [5, 3]];
        assert_eq!(run(6, &e), vec![vec![2, 3]]);
    }

    #[test]
    fn long_path_is_all_bridges() {
        let n = 100_000;
        let e: Vec<[i32; 2]> = (0..n - 1).map(|i| [i, i + 1]).collect();
        let got = run(n, &e);
        assert_eq!(got.len(), (n - 1) as usize);
        assert_eq!(got[0], vec![0, 1]);
        assert_eq!(got[got.len() - 1], vec![n - 2, n - 1]);
    }
}
