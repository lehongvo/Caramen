// ==================================================================
// PRACTICE 0213: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0213_design_neighbor_sum_service.md
// Run the tests:          cargo test practice::p0213
// ==================================================================

//! # Problem 0213: Design Neighbor Sum Service
//!
//! - **Difficulty:** Easy
//! - **Topics:** design struct, private helper methods, lookup table built in `new`, `checked_add_signed`
//! - **Source:** [LeetCode 3242. Design Neighbor Sum Service](https://leetcode.com/problems/design-neighbor-sum-service/)
//!
//! ## Problem
//!
//! You are given an `n x n` grid that contains every integer from `0` to `n² - 1` exactly once.
//! Build a service that answers two kinds of questions about the cell holding a given `value`:
//!
//! - `adjacent_sum(&self, value: i32) -> i32`: the sum of the values directly **above, below, left
//!   and right** of that cell;
//! - `diagonal_sum(&self, value: i32) -> i32`: the sum of the values in the four **diagonal**
//!   neighbours (top-left, top-right, bottom-left, bottom-right).
//!
//! Neighbours outside the grid simply do not count. The constructor is
//! `NeighborSum::new(grid: Vec<Vec<i32>>) -> Self`. This is a LeetCode *design* problem, so there is
//! no `impl Solution`: paste the struct and its `impl NeighborSum` block into LeetCode.
//!
//! ## Examples
//!
//! ```text
//! grid = [[0, 1, 2],
//!         [3, 4, 5],
//!         [6, 7, 8]]
//! adjacent_sum(1) => 6     (0 + 2 + 4; nothing above the top row)
//! adjacent_sum(4) => 16    (1 + 3 + 5 + 7)
//! diagonal_sum(4) => 16    (0 + 2 + 6 + 8)
//! diagonal_sum(8) => 4     (only 4 is a diagonal neighbour of the corner)
//!
//! grid = [[1, 2, 0, 3],
//!         [4, 7, 15, 6],
//!         [8, 9, 10, 11],
//!         [12, 13, 14, 5]]
//! adjacent_sum(15) => 23   (0 + 10 + 7 + 6)
//! diagonal_sum(9)  => 45   (4 + 15 + 12 + 14)
//! ```
//!
//! ## Constraints
//!
//! - `3 <= n <= 10`
//! - The grid holds every value `0..n²` exactly once; every queried `value` is in the grid.
//! - At most `2 * n²` calls in total.
//!
//! ## Hints
//!
//! 1. In `new`, record the `(row, col)` position of every value in a `Vec<(usize, usize)>` indexed
//!    by the value, so a query does not have to search the grid.
//! 2. Write one private helper that sums the neighbours for a list of `(dr, dc)` offsets, and call it
//!    from both public methods with different offsets.

/// Answers neighbour-sum queries on a grid of distinct values.
pub struct NeighborSum {}

impl NeighborSum {
    /// Stores the grid and indexes every value's position.
    pub fn new(grid: Vec<Vec<i32>>) -> Self {
        todo!("Write your solution here")
    }

    /// Sum of the up, down, left and right neighbours of `value`.
    pub fn adjacent_sum(&self, value: i32) -> i32 {
        todo!("Write your solution here")
    }

    /// Sum of the four diagonal neighbours of `value`.
    pub fn diagonal_sum(&self, value: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn three_by_three() -> NeighborSum {
        NeighborSum::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]])
    }

    #[test]
    fn example_one() {
        let s = three_by_three();
        assert_eq!(s.adjacent_sum(1), 6);
        assert_eq!(s.adjacent_sum(4), 16);
        assert_eq!(s.diagonal_sum(4), 16);
        assert_eq!(s.diagonal_sum(8), 4);
    }

    #[test]
    fn example_two() {
        let s = NeighborSum::new(vec![
            vec![1, 2, 0, 3],
            vec![4, 7, 15, 6],
            vec![8, 9, 10, 11],
            vec![12, 13, 14, 5],
        ]);
        assert_eq!(s.adjacent_sum(15), 23);
        assert_eq!(s.diagonal_sum(9), 45);
    }

    #[test]
    fn corners_have_few_neighbours() {
        let s = three_by_three();
        assert_eq!(s.adjacent_sum(0), 1 + 3);
        assert_eq!(s.adjacent_sum(8), 5 + 7);
        assert_eq!(s.diagonal_sum(0), 4);
        assert_eq!(s.diagonal_sum(6), 4);
    }

    #[test]
    fn edge_cells() {
        let s = three_by_three();
        // Value 3 is at (1, 0): neighbours 0 (above), 6 (below) and 4 (right).
        assert_eq!(s.adjacent_sum(3), 10);
        assert_eq!(s.diagonal_sum(3), 1 + 7);
    }

    #[test]
    fn largest_grid_center_and_corner() {
        let grid: Vec<Vec<i32>> = (0..10)
            .map(|r| (0..10).map(|c| r * 10 + c).collect())
            .collect();
        let s = NeighborSum::new(grid);
        // value 55 is at (5, 5): neighbours 45, 65, 54, 56; diagonals 44, 46, 64, 66
        assert_eq!(s.adjacent_sum(55), 45 + 65 + 54 + 56);
        assert_eq!(s.diagonal_sum(55), 44 + 46 + 64 + 66);
        assert_eq!(s.adjacent_sum(99), 89 + 98);
        assert_eq!(s.diagonal_sum(99), 88);
    }
}
