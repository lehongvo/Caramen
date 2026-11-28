// ==================================================================
// PRACTICE 0524: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0524_range_sum_query_2d_immutable.md
// Run the tests:          cargo test practice::p0524
// ==================================================================

//! # Problem 0524: Range Sum Query 2D - Immutable
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct wrapping a generic `PrefixSum2D<T>`, bounds `Copy + Default + Add + Sub`, 2D prefix sums
//! - **Source:** [LeetCode 304. Range Sum Query 2D - Immutable](https://leetcode.com/problems/range-sum-query-2d-immutable/)
//!
//! ## Problem
//!
//! You receive a matrix once and then answer many questions "what is the sum of the rectangle with
//! upper-left corner `(row1, col1)` and lower-right corner `(row2, col2)`, both corners included?".
//! The matrix never changes, and each query must take `O(1)` time.
//!
//! Implement `NumMatrix` (the LeetCode API):
//!
//! - `NumMatrix::new(matrix: Vec<Vec<i32>>) -> Self`
//! - `sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32`
//!
//! Prefix sums work for any number type, so put the logic in a **generic** struct first:
//!
//! - `PrefixSum2D::<T>::new(matrix: &[Vec<T>]) -> Self` precomputes the table.
//! - `PrefixSum2D::<T>::sum(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> T` answers a query
//!   (inclusive corners).
//!
//! with the bounds `T: Copy + Default + Add<Output = T> + Sub<Output = T>` on its impl block, and
//! make `NumMatrix` a thin wrapper around a `PrefixSum2D<i32>`. This is a LeetCode *design* problem,
//! so there is no `impl Solution`: paste both structs and their impl blocks into LeetCode.
//!
//! ## Examples
//!
//! ```text
//! matrix = [[3, 0, 1, 4, 2],
//!           [5, 6, 3, 2, 1],
//!           [1, 2, 0, 1, 5],
//!           [4, 1, 0, 1, 7],
//!           [1, 0, 3, 0, 5]]
//! sum_region(2, 1, 4, 3) => 8     (2+0+1 + 1+0+1 + 0+3+0)
//! sum_region(1, 1, 2, 2) => 11    (6+3 + 2+0)
//! sum_region(1, 2, 2, 4) => 12    (3+2+1 + 0+1+5)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 200`
//! - `-10^4 <= matrix[i][j] <= 10^4`
//! - `0 <= row1 <= row2 < m`, `0 <= col1 <= col2 < n`
//! - At most `10^4` calls to `sum_region`.
//!
//! ## Hints
//!
//! 1. Let `p[i][j]` be the sum of the rectangle of the first `i` rows and first `j` columns, with an
//!    extra row and column of zeros (`T::default()`).
//! 2. `p[i + 1][j + 1] = p[i][j + 1] + p[i + 1][j] - p[i][j] + matrix[i][j]`.
//! 3. A query is `p[r2 + 1][c2 + 1] - p[r1][c2 + 1] - p[r2 + 1][c1] + p[r1][c1]` (inclusion-exclusion).

use std::ops::{Add, Sub};

/// 2D prefix sums over any additive type.
pub struct PrefixSum2D<T> {
    /// `table[i][j]` is the sum of the first `i` rows and `j` columns (padded with zeros).
    table: Vec<Vec<T>>,
}

impl<T> PrefixSum2D<T>
where
    T: Copy + Default + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(matrix: &[Vec<T>]) -> Self {
        todo!("Write your solution here")
    }

    /// Sum of the rectangle `(r1, c1)..=(r2, c2)`.
    pub fn sum(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> T {
        todo!("Write your solution here")
    }
}

/// LeetCode's API, backed by `PrefixSum2D<i32>`.
pub struct NumMatrix {}

impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        todo!("Write your solution here")
    }

    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_matrix() -> Vec<Vec<i32>> {
        vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]
    }

    #[test]
    fn example_queries() {
        let m = NumMatrix::new(example_matrix());
        assert_eq!(m.sum_region(2, 1, 4, 3), 8);
        assert_eq!(m.sum_region(1, 1, 2, 2), 11);
        assert_eq!(m.sum_region(1, 2, 2, 4), 12);
    }

    #[test]
    fn single_cells_and_whole_matrix() {
        let m = NumMatrix::new(example_matrix());
        assert_eq!(m.sum_region(0, 0, 0, 0), 3);
        assert_eq!(m.sum_region(4, 4, 4, 4), 5);
        let total: i32 = example_matrix().iter().flatten().sum();
        assert_eq!(m.sum_region(0, 0, 4, 4), total);
    }

    #[test]
    fn generic_with_floats_and_i64() {
        let f = PrefixSum2D::new(&[vec![0.5, 1.5], vec![2.0, 4.0]]);
        assert_eq!(f.sum(0, 0, 1, 1), 8.0);
        assert_eq!(f.sum(1, 0, 1, 1), 6.0);
        let big = PrefixSum2D::new(&[vec![4_000_000_000_i64, 1], vec![1, 4_000_000_000]]);
        assert_eq!(big.sum(0, 0, 1, 1), 8_000_000_002);
    }

    #[test]
    fn negative_values_large_matrix() {
        let matrix: Vec<Vec<i32>> = (0..200)
            .map(|i| {
                (0..200)
                    .map(|j| if (i + j) % 2 == 0 { 10_000 } else { -10_000 })
                    .collect()
            })
            .collect();
        let m = NumMatrix::new(matrix);
        assert_eq!(m.sum_region(0, 0, 199, 199), 0);
        assert_eq!(m.sum_region(0, 0, 0, 0), 10_000);
        assert_eq!(m.sum_region(0, 0, 2, 2), 10_000);
        assert_eq!(m.sum_region(5, 7, 5, 8), 0);
    }
}
