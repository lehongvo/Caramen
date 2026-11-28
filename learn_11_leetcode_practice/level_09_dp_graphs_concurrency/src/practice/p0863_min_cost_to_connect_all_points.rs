// ==================================================================
// PRACTICE 0863: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0863_min_cost_to_connect_all_points.md
// Run the tests:          cargo test practice::p0863
// ==================================================================

//! # Problem 0863: Min Cost to Connect All Points
//!
//! - **Difficulty:** Medium
//! - **Topics:** minimum spanning tree with Prim's algorithm on a dense graph (`O(n^2)`, no heap),
//!   Manhattan distance, `i64` sums
//! - **Source:** [LeetCode 1584. Min Cost to Connect All Points](https://leetcode.com/problems/min-cost-to-connect-all-points/)
//!
//! ## Problem
//!
//! You are given `points[i] = [x, y]` on a plane. Connecting points `a` and `b` costs their
//! **Manhattan distance** `|xa - xb| + |ya - yb|`. Return the minimum total cost to connect all
//! points so that there is exactly one simple path between any two points (that is, the total
//! weight of a minimum spanning tree).
//!
//! Implement `Solution::min_cost_connect_points(points: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  points = [[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]
//! Output: 20
//!
//! Input:  points = [[3, 12], [-2, 5], [-4, 1]]
//! Output: 18
//! ```
//!
//! ## Constraints
//!
//! - `1 <= points.len() <= 1000`
//! - `-10^6 <= x, y <= 10^6`; all points are distinct
//! - The answer fits in an `i32`.
//!
//! ## Hints
//!
//! 1. Every pair of points is a potential edge, so the graph is complete: `n^2 / 2` edges. Sorting
//!    them for Kruskal works but is heavy.
//! 2. Prim's algorithm grows one tree: repeatedly add the outside point that is closest to the tree.
//! 3. Keep `best[v]` = cheapest connection from the tree to `v`; after adding a point, update
//!    `best` for the others. No heap is needed for a dense graph.

pub struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(points: &[[i32; 2]]) -> i32 {
        Solution::min_cost_connect_points(points.iter().map(|p| p.to_vec()).collect())
    }

    #[test]
    fn example_five_points() {
        assert_eq!(run(&[[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]), 20);
    }

    #[test]
    fn example_three_points() {
        assert_eq!(run(&[[3, 12], [-2, 5], [-4, 1]]), 18);
    }

    #[test]
    fn single_point() {
        assert_eq!(run(&[[0, 0]]), 0);
    }

    #[test]
    fn small_square() {
        assert_eq!(run(&[[0, 0], [1, 1], [1, 0], [-1, 1]]), 4);
    }

    #[test]
    fn thousand_points() {
        let pts: Vec<[i32; 2]> = (0..1000_i64)
            .map(|i| {
                let x = (i * 7919) % 2_000_001 - 1_000_000;
                let y = (i * 104_729) % 2_000_001 - 1_000_000;
                [x as i32, y as i32]
            })
            .collect();
        assert_eq!(run(&pts), 64_727_094);
        let line: Vec<[i32; 2]> = (0..1000).map(|i| [i, 0]).collect();
        assert_eq!(run(&line), 999);
    }
}
