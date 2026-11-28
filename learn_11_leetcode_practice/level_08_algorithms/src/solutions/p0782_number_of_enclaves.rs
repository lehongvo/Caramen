//! # Problem 0782: Number of Enclaves
//!
//! - **Difficulty:** Medium
//! - **Topics:** flood fill from the border, iterator `chain` of border cells, counting with `flatten`
//! - **Source:** [LeetCode 1020. Number of Enclaves](https://leetcode.com/problems/number-of-enclaves/)
//!
//! ## Problem
//!
//! You are given an `m x n` binary matrix `grid`: `1` is land and `0` is sea. A move goes from a
//! land cell to a 4-directionally adjacent land cell, or walks off the edge of the grid. Return the
//! number of land cells from which you **cannot** walk off the grid in any number of moves.
//!
//! Implement `Solution::num_enclaves(grid: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  grid = [[0, 0, 0, 0],
//!                 [1, 0, 1, 0],
//!                 [0, 1, 1, 0],
//!                 [0, 0, 0, 0]]
//! Output: 3             (the 1 at (1, 0) touches the border; the other three are enclosed)
//!
//! Input:  grid = [[0, 1, 1, 0],
//!                 [0, 0, 1, 0],
//!                 [0, 0, 1, 0],
//!                 [0, 0, 0, 0]]
//! Output: 0             (every land cell is connected to the top border)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 500`
//! - `grid[i][j]` is `0` or `1`.
//!
//! ## Hints
//!
//! 1. Instead of asking "can this cell escape?" for every cell, start from the cells that can
//!    escape for sure: land on the border.
//! 2. Remove everything reachable from the border land. Whatever land remains is enclosed.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** flood fill from the border: a land cell can walk off the grid exactly when it is
//! connected to some land cell on the border. So push every border land cell on a stack and run
//! a DFS that turns all reachable land into sea. The land that survives is enclosed: count it.
//! Reversing the question (start from the exits, not from each cell) turns many searches into one.
//!
//! **Complexity:** `O(m * n)` time and space.
//!
//! **Rust concepts:**
//! - The border cells are produced by chaining iterators:
//!   `(0..m).flat_map(|r| [(r, 0), (r, n - 1)]).chain((0..n).flat_map(|c| [(0, c), (m - 1, c)]))`.
//!   Corner cells appear twice, which is harmless because a sunk cell is skipped.
//! - `grid.iter().flatten().filter(|&&x| x == 1).count()` counts over all rows at once.
//! - An explicit stack is used because a region can hold `250_000` cells, too deep for recursion.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        // @solution-start
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        let border = (0..m)
            .flat_map(|r| [(r, 0), (r, n - 1)])
            .chain((0..n).flat_map(|c| [(0, c), (m - 1, c)]));
        let mut stack: Vec<(usize, usize)> = Vec::new();
        for (r, c) in border {
            if grid[r][c] == 1 {
                grid[r][c] = 0;
                stack.push((r, c));
            }
        }
        while let Some((r, c)) = stack.pop() {
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc))
                else {
                    continue;
                };
                if nr < m && nc < n && grid[nr][nc] == 1 {
                    grid[nr][nc] = 0;
                    stack.push((nr, nc));
                }
            }
        }
        grid.iter().flatten().filter(|&&x| x == 1).count() as i32
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_enclosed() {
        let grid = vec![
            vec![0, 0, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 1, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(Solution::num_enclaves(grid), 3);
    }

    #[test]
    fn example_all_connected_to_border() {
        let grid = vec![
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(Solution::num_enclaves(grid), 0);
    }

    #[test]
    fn single_cell_is_on_the_border() {
        assert_eq!(Solution::num_enclaves(vec![vec![1]]), 0);
    }

    #[test]
    fn single_enclosed_cell() {
        let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::num_enclaves(grid), 1);
    }

    #[test]
    fn diagonal_contact_with_the_border_does_not_count() {
        let grid = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(Solution::num_enclaves(grid), 1);
    }

    #[test]
    fn largest_grid() {
        // Sea on the border, land inside: 498 * 498 enclosed cells.
        let mut grid = vec![vec![1; 500]; 500];
        for i in 0..500 {
            grid[0][i] = 0;
            grid[499][i] = 0;
            grid[i][0] = 0;
            grid[i][499] = 0;
        }
        assert_eq!(Solution::num_enclaves(grid.clone()), 248_004);
        // Opening one border cell lets the whole inside escape.
        grid[0][1] = 1;
        assert_eq!(Solution::num_enclaves(grid), 0);
    }
}
