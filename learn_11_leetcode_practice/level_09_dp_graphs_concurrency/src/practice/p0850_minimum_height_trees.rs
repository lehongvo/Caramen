// ==================================================================
// PRACTICE 0850: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0850_minimum_height_trees.md
// Run the tests:          cargo test practice::p0850
// ==================================================================

//! # Problem 0850: Minimum Height Trees
//!
//! - **Difficulty:** Medium
//! - **Topics:** trees as undirected graphs, peeling leaves layer by layer (topological trimming),
//!   `std::mem::take`
//! - **Source:** [LeetCode 310. Minimum Height Trees](https://leetcode.com/problems/minimum-height-trees/)
//!
//! ## Problem
//!
//! A tree with `n` nodes `0..n` is given by its `n - 1` undirected `edges`. Choosing any node as
//! the root turns it into a rooted tree; its **height** is the number of edges on the longest path
//! from the root down to a leaf. Return every node whose choice as root gives the **minimum**
//! height, in any order.
//!
//! Implement `Solution::find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 4, edges = [[1, 0], [1, 2], [1, 3]]
//! Output: [1]
//! Explanation: rooted at 1 the height is 1; any other root gives 2.
//!
//! Input:  n = 6, edges = [[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]]
//! Output: [3, 4]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 2 * 10^4`
//! - `edges.len() == n - 1`; the edges form a tree (connected, no cycle, no repeated edge).
//!
//! ## Hints
//!
//! 1. Running BFS from every node costs `O(n^2)`. Think about where the best roots must be: in the
//!    "middle" of the longest path.
//! 2. Remove all current leaves at once, then the leaves of what is left, and so on, like peeling
//!    an onion.
//! 3. Stop when at most 2 nodes remain; those are the answer (a tree has one or two centers).

pub struct Solution;

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(n: i32, edges: &[[i32; 2]]) -> Vec<i32> {
        let mut got =
            Solution::find_min_height_trees(n, edges.iter().map(|e| e.to_vec()).collect());
        got.sort_unstable();
        got
    }

    #[test]
    fn example_star() {
        assert_eq!(run(4, &[[1, 0], [1, 2], [1, 3]]), vec![1]);
    }

    #[test]
    fn example_two_centers() {
        assert_eq!(
            run(6, &[[3, 0], [3, 1], [3, 2], [3, 4], [5, 4]]),
            vec![3, 4]
        );
    }

    #[test]
    fn one_and_two_nodes() {
        assert_eq!(run(1, &[]), vec![0]);
        assert_eq!(run(2, &[[0, 1]]), vec![0, 1]);
    }

    #[test]
    fn path_graphs() {
        assert_eq!(run(3, &[[0, 1], [1, 2]]), vec![1]);
        let n = 20_000;
        let edges: Vec<[i32; 2]> = (0..n - 1).map(|i| [i, i + 1]).collect();
        assert_eq!(run(n, &edges), vec![9_999, 10_000]);
    }

    #[test]
    fn uneven_branches() {
        // 0-1-2-3-4-5 with a short branch 2-6: the longest path 0..5 has centers 2 and 3.
        let edges = [[0, 1], [1, 2], [2, 3], [3, 4], [4, 5], [2, 6]];
        assert_eq!(run(7, &edges), vec![2, 3]);
    }
}
