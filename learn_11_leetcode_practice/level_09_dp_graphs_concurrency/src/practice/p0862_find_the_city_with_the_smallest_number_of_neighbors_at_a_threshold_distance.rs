// ==================================================================
// PRACTICE 0862: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0862_find_the_city_with_the_smallest_number_of_neighbors_at_a_threshold_distance.md
// Run the tests:          cargo test practice::p0862
// ==================================================================

//! # Problem 0862: Find the City With the Smallest Number of Neighbors at a Threshold Distance
//!
//! - **Difficulty:** Medium
//! - **Topics:** all-pairs shortest paths with Floyd-Warshall, `Vec<Vec<u32>>` matrix, tie-breaking
//!   with `min_by_key` on a tuple
//! - **Source:** [LeetCode 1334. Find the City With the Smallest Number of Neighbors at a Threshold Distance](https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/)
//!
//! ## Problem
//!
//! There are `n` cities `0..n` and bidirectional weighted roads `edges[i] = [from, to, weight]`.
//! For each city, count the **other** cities whose shortest-path distance from it is at most
//! `distance_threshold`. Return the city with the smallest count; if several cities tie, return the
//! one with the **greatest** number.
//!
//! Implement `Solution::find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 4, edges = [[0, 1, 3], [1, 2, 1], [1, 3, 4], [2, 3, 1]], distance_threshold = 4
//! Output: 3
//! Explanation: within distance 4, city 0 reaches {1, 2}, city 1 {0, 2, 3}, city 2 {0, 1, 3} and
//! city 3 {1, 2}. Cities 0 and 3 tie with 2; return the greater one.
//!
//! Input:  n = 5, edges = [[0, 1, 2], [0, 4, 8], [1, 2, 3], [1, 4, 2], [2, 3, 1], [3, 4, 1]],
//!         distance_threshold = 2
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `2 <= n <= 100`; `1 <= edges.len() <= n * (n - 1) / 2`
//! - `0 <= from < to < n`; `1 <= weight, distance_threshold <= 10^4`; all pairs are distinct.
//!
//! ## Hints
//!
//! 1. You need the distance between **every** pair of cities. With `n <= 100`, Floyd-Warshall in
//!    `O(n^3)` is simple and fast enough.
//! 2. Floyd-Warshall: for each intermediate city `m`, try to improve every `dist[i][j]` with
//!    `dist[i][m] + dist[m][j]`. The loop over `m` must be the outermost one.

use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(n: i32, edges: &[[i32; 3]], threshold: i32) -> i32 {
        Solution::find_the_city(n, edges.iter().map(|e| e.to_vec()).collect(), threshold)
    }

    #[test]
    fn example_tie_goes_to_greater_city() {
        assert_eq!(run(4, &[[0, 1, 3], [1, 2, 1], [1, 3, 4], [2, 3, 1]], 4), 3);
    }

    #[test]
    fn example_five_cities() {
        let e = [
            [0, 1, 2],
            [0, 4, 8],
            [1, 2, 3],
            [1, 4, 2],
            [2, 3, 1],
            [3, 4, 1],
        ];
        assert_eq!(run(5, &e, 2), 0);
    }

    #[test]
    fn isolated_city_wins() {
        assert_eq!(run(3, &[[0, 1, 1]], 1), 2);
    }

    #[test]
    fn shortest_path_uses_many_roads() {
        // The direct road 0-1 costs 10, but 0-2-3-4-5-1 costs 5.
        let e = [
            [0, 1, 10],
            [0, 2, 1],
            [2, 3, 1],
            [3, 4, 1],
            [4, 5, 1],
            [1, 5, 1],
        ];
        assert_eq!(run(6, &e, 3), 1);
    }

    #[test]
    fn largest_complete_graph() {
        // All roads have weight 1 except those touching city 99, which weigh 10^4.
        let mut e = Vec::new();
        for a in 0..100 {
            for b in a + 1..100 {
                e.push([a, b, if b == 99 { 10_000 } else { 1 }]);
            }
        }
        assert_eq!(run(100, &e, 5), 99);
    }
}
