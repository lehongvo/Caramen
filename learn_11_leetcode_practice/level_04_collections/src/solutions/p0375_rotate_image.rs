//! # Problem 0375: Rotate Image
//!
//! - **Difficulty:** Medium
//! - **Topics:** in-place 2-D `Vec` updates, two mutable borrows with `split_at_mut`, `std::mem::swap`, `reverse`
//! - **Source:** [LeetCode 48. Rotate Image](https://leetcode.com/problems/rotate-image/)
//!
//! ## Problem
//!
//! You are given an `n x n` matrix representing an image. Rotate the image by 90 degrees
//! **clockwise**, **in place**: modify the matrix you are given; do not allocate another `n x n`
//! matrix.
//!
//! Implement `Solution::rotate(matrix: &mut Vec<Vec<i32>>)`.
//!
//! ## Examples
//!
//! ```text
//! Input:  [[1, 2, 3],
//!          [4, 5, 6],
//!          [7, 8, 9]]
//! Output: [[7, 4, 1],
//!          [8, 5, 2],
//!          [9, 6, 3]]
//!
//! Input:  [[5, 1, 9, 11], [2, 4, 8, 10], [13, 3, 6, 7], [15, 14, 12, 16]]
//! Output: [[15, 13, 2, 5], [14, 3, 4, 1], [12, 6, 8, 9], [16, 7, 10, 11]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 20`
//! - `-1000 <= matrix[i][j] <= 1000`
//!
//! ## Hints
//!
//! 1. A clockwise rotation equals a **transpose** (swap `matrix[i][j]` with `matrix[j][i]`)
//!    followed by **reversing every row**.
//! 2. `std::mem::swap(&mut matrix[i][j], &mut matrix[j][i])` does not compile: it borrows
//!    `matrix` mutably twice. `split_at_mut` splits the outer vector into two disjoint halves that
//!    can be borrowed mutably at the same time.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** transpose, then mirror horizontally. For the transpose, only the cells above the
//! diagonal (`j > i`) are swapped with their mirror cells. Then `row.reverse()` on every row
//! finishes the rotation.
//!
//! **Complexity:** `O(n^2)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - The borrow checker cannot see that `matrix[i]` and `matrix[j]` are different rows.
//!   `matrix.split_at_mut(j)` returns `(&mut [Vec<i32>], &mut [Vec<i32>])` where row `i < j` is in the
//!   first part and row `j` is `bottom[0]`, so both cells can be borrowed mutably at once.
//! - `std::mem::swap(&mut a, &mut b)` exchanges two values through mutable references.
//! - LeetCode's signature takes `&mut Vec<Vec<i32>>`; Clippy's `ptr_arg` lint would prefer
//!   `&mut [Vec<i32>]`, so it is allowed here to keep LeetCode's exact signature.
// @solution-end

pub struct Solution;

impl Solution {
    // LeetCode's signature uses `&mut Vec<Vec<i32>>`.
    #[allow(clippy::ptr_arg)]
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // @solution-start
        let n = matrix.len();
        for i in 0..n {
            for j in i + 1..n {
                // Row i lives in `top`, row j is the first row of `bottom`.
                let (top, bottom) = matrix.split_at_mut(j);
                std::mem::swap(&mut top[i][j], &mut bottom[0][i]);
            }
        }
        for row in matrix.iter_mut() {
            row.reverse();
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rotated(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Solution::rotate(&mut matrix);
        matrix
    }

    #[test]
    fn example_three_by_three() {
        assert_eq!(
            rotated(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]
        );
    }

    #[test]
    fn example_four_by_four() {
        assert_eq!(
            rotated(vec![
                vec![5, 1, 9, 11],
                vec![2, 4, 8, 10],
                vec![13, 3, 6, 7],
                vec![15, 14, 12, 16]
            ]),
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }

    #[test]
    fn one_and_two() {
        assert_eq!(rotated(vec![vec![-1000]]), vec![vec![-1000]]);
        assert_eq!(
            rotated(vec![vec![1, 2], vec![3, 4]]),
            vec![vec![3, 1], vec![4, 2]]
        );
    }

    #[test]
    fn four_rotations_are_the_identity() {
        let original: Vec<Vec<i32>> = (0..20)
            .map(|r| (0..20).map(|c| r * 20 + c).collect())
            .collect();
        let mut m = original.clone();
        Solution::rotate(&mut m);
        // The first column, read bottom-up, becomes the first row.
        assert_eq!(m[0], (0..20).rev().map(|r| r * 20).collect::<Vec<i32>>());
        for _ in 0..3 {
            Solution::rotate(&mut m);
        }
        assert_eq!(m, original);
    }
}
