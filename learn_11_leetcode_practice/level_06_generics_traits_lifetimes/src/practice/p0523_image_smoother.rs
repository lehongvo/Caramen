// ==================================================================
// PRACTICE 0523: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0523_image_smoother.md
// Run the tests:          cargo test practice::p0523
// ==================================================================

//! # Problem 0523: Image Smoother
//!
//! - **Difficulty:** Easy
//! - **Topics:** generic struct `Grid<T>`, `impl<T> Grid<T>`, generic `map<U, F>`, neighbourhood queries
//! - **Source:** [LeetCode 661. Image Smoother](https://leetcode.com/problems/image-smoother/)
//!
//! ## Problem
//!
//! A grayscale image is an `m x n` matrix of integers. **Smoothing** replaces every pixel with the
//! average of itself and its (up to) eight neighbours, i.e. the 3x3 block centred on it, clipped at the
//! image border. The average is rounded **down** (integer division).
//!
//! Implement `Solution::image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
//!
//! Build it on top of a reusable generic grid stored in row-major order. `Grid<T>` has private
//! fields `rows`, `cols` and `cells: Vec<T>`; implement in `impl<T> Grid<T>`:
//!
//! - `from_rows(rows: Vec<Vec<T>>) -> Self`: every inner vector is one row (all rows have the same
//!   length; an empty input gives a `0 x 0` grid).
//! - `rows(&self) -> usize`, `cols(&self) -> usize`
//! - `get(&self, r: usize, c: usize) -> Option<&T>`: `None` outside the grid.
//! - `block3x3(&self, r: usize, c: usize) -> Vec<&T>`: the cell `(r, c)` and its neighbours that lie
//!   inside the grid, in row-major order.
//! - `map<U, F>(&self, f: F) -> Grid<U> where F: Fn(usize, usize, &T) -> U`: a new grid of the same
//!   shape whose cell `(r, c)` is `f(r, c, &old_cell)`.
//! - `into_rows(self) -> Vec<Vec<T>>`
//!
//! ## Examples
//!
//! ```text
//! Input:  img = [[1, 1, 1], [1, 0, 1], [1, 1, 1]]
//! Output: [[0, 0, 0], [0, 0, 0], [0, 0, 0]]
//!         corner: 3 / 4 = 0; edge: 5 / 6 = 0; centre: 8 / 9 = 0
//!
//! Input:  img = [[100, 200, 100], [200, 50, 200], [100, 200, 100]]
//! Output: [[137, 141, 137], [141, 138, 141], [137, 141, 137]]
//!
//! Grid::from_rows(vec![vec!['a', 'b'], vec!['c', 'd']]).block3x3(0, 0) => [&'a', &'b', &'c', &'d']
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 200`
//! - `0 <= img[i][j] <= 255`
//!
//! ## Hints
//!
//! 1. Cell `(r, c)` lives at `cells[r * cols + c]`.
//! 2. For the 3x3 block, loop over `r.saturating_sub(1)..=(r + 1).min(rows - 1)` and the same for
//!    columns; `saturating_sub` avoids underflow at row `0`.
//! 3. `image_smoother` is then `grid.map(|r, c, _| average of grid.block3x3(r, c))`.

pub struct Solution;

/// A rectangular grid stored in row-major order.
#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        todo!("Write your solution here")
    }

    pub fn rows(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn cols(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&T> {
        todo!("Write your solution here")
    }

    /// The cell and its in-bounds neighbours, row-major.
    pub fn block3x3(&self, r: usize, c: usize) -> Vec<&T> {
        todo!("Write your solution here")
    }

    pub fn map<U, F>(&self, f: F) -> Grid<U>
    where
        F: Fn(usize, usize, &T) -> U,
    {
        todo!("Write your solution here")
    }

    pub fn into_rows(self) -> Vec<Vec<T>> {
        todo!("Write your solution here")
    }
}

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_dark_center() {
        assert_eq!(
            Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
        );
    }

    #[test]
    fn example_bright_pattern() {
        assert_eq!(
            Solution::image_smoother(vec![
                vec![100, 200, 100],
                vec![200, 50, 200],
                vec![100, 200, 100]
            ]),
            vec![
                vec![137, 141, 137],
                vec![141, 138, 141],
                vec![137, 141, 137]
            ]
        );
    }

    #[test]
    fn single_pixel_and_single_row() {
        assert_eq!(Solution::image_smoother(vec![vec![255]]), vec![vec![255]]);
        assert_eq!(
            Solution::image_smoother(vec![vec![0, 9, 3]]),
            vec![vec![4, 4, 6]]
        );
    }

    #[test]
    fn generic_grid_with_chars() {
        let g = Grid::from_rows(vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);
        assert_eq!((g.rows(), g.cols()), (2, 3));
        assert_eq!(g.get(1, 2), Some(&'f'));
        assert_eq!(g.get(2, 0), None);
        assert_eq!(g.block3x3(0, 0), vec![&'a', &'b', &'d', &'e']);
        assert_eq!(g.block3x3(1, 1).len(), 6);
        let upper = g.map(|r, c, ch| format!("{}{r}{c}", ch.to_ascii_uppercase()));
        assert_eq!(upper.get(1, 0).map(String::as_str), Some("D10"));
        assert_eq!(
            g.into_rows(),
            vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]
        );
    }

    #[test]
    fn large_uniform_image() {
        let img = vec![vec![77; 200]; 200];
        let out = Solution::image_smoother(img.clone());
        assert_eq!(out, img);
    }
}
