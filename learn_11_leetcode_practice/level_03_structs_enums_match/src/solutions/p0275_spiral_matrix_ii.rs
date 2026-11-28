//! # Problem 0275: Spiral Matrix II
//!
//! - **Difficulty:** Medium
//! - **Topics:** a direction `enum` with `delta` and `turn`, a cursor `struct`, "turn when blocked" simulation
//! - **Source:** [LeetCode 59. Spiral Matrix II](https://leetcode.com/problems/spiral-matrix-ii/)
//!
//! ## Problem
//!
//! Given a positive integer `n`, build an `n x n` matrix filled with the numbers `1` to `n²` in
//! **spiral order**: `1` in the top-left corner, then increasing to the right along the top row,
//! down the right column, left along the bottom row, up the left column, and inwards clockwise.
//!
//! Implement `Solution::generate_matrix(n: i32) -> Vec<Vec<i32>>`.
//!
//! This time, use a different technique from Problem 0274: a cursor walks one cell at a time and
//! **turns right whenever the next cell is outside the matrix or already filled**.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 3
//! Output: [[1, 2, 3],
//!          [8, 9, 4],
//!          [7, 6, 5]]
//!
//! Input:  n = 1
//! Output: [[1]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 20`
//!
//! ## Hints
//!
//! 1. Start with a matrix of zeros: `0` means "not filled yet".
//! 2. `Dir::delta(self) -> (isize, isize)` and `checked_add_signed` give the next cell, or `None`
//!    when it would leave the matrix through the top or left edge.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** place the numbers `1..=n²` one by one. Before each move, look at the cell ahead; if
//! it is outside or already filled, turn clockwise first. Because the spiral never traps itself,
//! one turn is always enough.
//!
//! **Complexity:** `O(n²)` time; `O(1)` extra space besides the output.
//!
//! **Rust concepts:**
//! - `Dir::turn` and `Dir::delta` are two small exhaustive `match`es; the walking code never
//!   mentions coordinates directly.
//! - `fn next_cell(...) -> Option<(usize, usize)>` combines `checked_add_signed`, a bounds check and
//!   the "still empty" test; the `?` operator ends it early.
//! - `vec![vec![0; n]; n]` allocates the matrix; the inner `vec!` is cloned `n` times.
// @solution-end

pub struct Solution;

// @solution-start remove
#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn turn(self) -> Dir {
        match self {
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Right,
        }
    }

    fn delta(self) -> (isize, isize) {
        match self {
            Dir::Right => (0, 1),
            Dir::Down => (1, 0),
            Dir::Left => (0, -1),
            Dir::Up => (-1, 0),
        }
    }
}

/// The cell one step ahead, if it exists and is still empty.
fn next_cell(grid: &[Vec<i32>], (r, c): (usize, usize), dir: Dir) -> Option<(usize, usize)> {
    let (dr, dc) = dir.delta();
    let nr = r.checked_add_signed(dr)?;
    let nc = c.checked_add_signed(dc)?;
    match grid.get(nr)?.get(nc)? {
        0 => Some((nr, nc)),
        _ => None,
    }
}
// @solution-end

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        // @solution-start
        let size = n as usize;
        let mut grid = vec![vec![0; size]; size];
        let (mut pos, mut dir) = ((0, 0), Dir::Right);
        for value in 1..=n * n {
            grid[pos.0][pos.1] = value;
            if value == n * n {
                break;
            }
            if next_cell(&grid, pos, dir).is_none() {
                dir = dir.turn();
            }
            match next_cell(&grid, pos, dir) {
                Some(next) => pos = next,
                None => break,
            }
        }
        grid
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }

    #[test]
    fn example_one() {
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }

    #[test]
    fn two_and_four() {
        assert_eq!(Solution::generate_matrix(2), vec![vec![1, 2], vec![4, 3]]);
        assert_eq!(
            Solution::generate_matrix(4),
            vec![
                vec![1, 2, 3, 4],
                vec![12, 13, 14, 5],
                vec![11, 16, 15, 6],
                vec![10, 9, 8, 7]
            ]
        );
    }

    #[test]
    fn twenty_contains_every_number_once() {
        let m = Solution::generate_matrix(20);
        assert_eq!(m[0][19], 20);
        assert_eq!(m[19][19], 39);
        let mut all: Vec<i32> = m.into_iter().flatten().collect();
        all.sort_unstable();
        assert_eq!(all, (1..=400).collect::<Vec<_>>());
    }
}
