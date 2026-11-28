// ==================================================================
// PRACTICE 0779: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0779_flood_fill.md
// Run the tests:          cargo test practice::p0779
// ==================================================================

//! # Problem 0779: Flood Fill
//!
//! - **Difficulty:** Easy
//! - **Topics:** DFS on a grid with an explicit stack, `let ... else`, `checked_add_signed`
//! - **Source:** [LeetCode 733. Flood Fill](https://leetcode.com/problems/flood-fill/)
//!
//! ## Problem
//!
//! An image is an `m x n` grid of integers, `image[i][j]` being the colour of a pixel. Starting
//! from pixel `(sr, sc)`, perform a **flood fill** with colour `color`: change the starting pixel
//! and every pixel connected to it **4-directionally** (up, down, left, right) through pixels of
//! the **same original colour** as the starting pixel, to `color`. Return the modified image.
//!
//! Implement `Solution::flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  image = [[1, 1, 1],
//!                  [1, 1, 0],
//!                  [1, 0, 1]], sr = 1, sc = 1, color = 2
//! Output: [[2, 2, 2],
//!          [2, 2, 0],
//!          [2, 0, 1]]
//!         (the bottom-right 1 only touches the others diagonally, so it stays)
//!
//! Input:  image = [[0, 0, 0], [0, 0, 0]], sr = 0, sc = 0, color = 0
//! Output: [[0, 0, 0], [0, 0, 0]]     (already the right colour)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 50`
//! - `0 <= image[i][j], color < 2^16`
//! - `0 <= sr < m` and `0 <= sc < n`
//!
//! ## Hints
//!
//! 1. Visit the region with depth-first search. A `Vec` used as a stack avoids recursion.
//! 2. Recolour a pixel when you push it, so it is never pushed twice.
//! 3. If the starting pixel already has colour `color`, there is nothing to do, and continuing
//!    would loop forever (recoloured pixels would still look "original").

pub struct Solution;

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_diagonal_pixel_stays() {
        let image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        assert_eq!(
            Solution::flood_fill(image, 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
    }

    #[test]
    fn example_same_colour() {
        let image = vec![vec![0, 0, 0], vec![0, 0, 0]];
        assert_eq!(Solution::flood_fill(image.clone(), 0, 0, 0), image);
    }

    #[test]
    fn single_pixel() {
        assert_eq!(Solution::flood_fill(vec![vec![5]], 0, 0, 7), vec![vec![7]]);
    }

    #[test]
    fn region_bounded_by_other_colours() {
        let image = vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]];
        assert_eq!(
            Solution::flood_fill(image, 1, 1, 9),
            vec![vec![0, 1, 0], vec![1, 9, 1], vec![0, 1, 0]]
        );
    }

    #[test]
    fn largest_image_is_filled_completely() {
        let filled = Solution::flood_fill(vec![vec![3; 50]; 50], 49, 0, 65_535);
        assert_eq!(filled, vec![vec![65_535; 50]; 50]);
    }

    #[test]
    fn snake_shaped_region() {
        // The 0s form one winding path around the two walls of 1s.
        let image = vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 0],
        ];
        let expected = vec![
            vec![4, 1, 4, 4, 4],
            vec![4, 1, 4, 1, 4],
            vec![4, 4, 4, 1, 4],
        ];
        assert_eq!(Solution::flood_fill(image, 0, 0, 4), expected);
    }
}
