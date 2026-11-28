// ==================================================================
// PRACTICE 0839: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0839_number_of_provinces.md
// Run the tests:          cargo test practice::p0839
// ==================================================================

//! # Problem 0839: Number of Provinces
//!
//! - **Difficulty:** Medium
//! - **Topics:** connected components, adjacency **matrix**, iterative DFS with a `Vec` stack
//! - **Source:** [LeetCode 547. Number of Provinces](https://leetcode.com/problems/number-of-provinces/)
//!
//! ## Problem
//!
//! There are `n` cities. `is_connected` is an `n x n` matrix where `is_connected[i][j] == 1` means
//! cities `i` and `j` are directly connected by a road (the matrix is symmetric and
//! `is_connected[i][i] == 1`). A **province** is a group of cities that are connected directly or
//! through other cities, and not connected to any city outside the group. Return the number of
//! provinces.
//!
//! Implement `Solution::find_circle_num(is_connected: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  is_connected = [[1, 1, 0],
//!                         [1, 1, 0],
//!                         [0, 0, 1]]
//! Output: 2
//!
//! Input:  is_connected = [[1, 0, 0],
//!                         [0, 1, 0],
//!                         [0, 0, 1]]
//! Output: 3
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 200`
//! - `is_connected[i][j]` is `0` or `1`; `is_connected[i][i] == 1`;
//!   `is_connected[i][j] == is_connected[j][i]`
//!
//! ## Hints
//!
//! 1. A province is a connected component of the graph. Count how many times you have to start a
//!    new search from an unvisited city.
//! 2. With an adjacency matrix, the neighbours of `u` are the `v` with `is_connected[u][v] == 1`.

pub struct Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_provinces() {
        let m = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        assert_eq!(Solution::find_circle_num(m), 2);
    }

    #[test]
    fn example_all_isolated() {
        let m = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        assert_eq!(Solution::find_circle_num(m), 3);
    }

    #[test]
    fn single_city() {
        assert_eq!(Solution::find_circle_num(vec![vec![1]]), 1);
    }

    #[test]
    fn indirect_connection() {
        // 0 - 3 and 3 - 1 put 0 and 1 in the same province without a direct road.
        let m = vec![
            vec![1, 0, 0, 1],
            vec![0, 1, 0, 1],
            vec![0, 0, 1, 0],
            vec![1, 1, 0, 1],
        ];
        assert_eq!(Solution::find_circle_num(m), 2);
    }

    #[test]
    fn large_chain_and_full_matrix() {
        let n = 200;
        let mut chain = vec![vec![0; n]; n];
        for i in 0..n {
            chain[i][i] = 1;
            if i + 1 < n {
                chain[i][i + 1] = 1;
                chain[i + 1][i] = 1;
            }
        }
        assert_eq!(Solution::find_circle_num(chain), 1);
        assert_eq!(Solution::find_circle_num(vec![vec![1; n]; n]), 1);
    }
}
