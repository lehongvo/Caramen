//! # Problem 0780: Number of Islands
//!
//! - **Difficulty:** Medium
//! - **Topics:** connected components with DFS, sinking visited land, `Vec<Vec<char>>`
//! - **Source:** [LeetCode 200. Number of Islands](https://leetcode.com/problems/number-of-islands/)
//!
//! ## Problem
//!
//! You are given an `m x n` map `grid` where `'1'` is land and `'0'` is water. An **island** is a
//! group of land cells connected **horizontally or vertically** (not diagonally), surrounded by
//! water. Everything outside the grid counts as water. Return the number of islands.
//!
//! Implement `Solution::num_islands(grid: Vec<Vec<char>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  grid = [['1', '1', '1', '1', '0'],
//!                 ['1', '1', '0', '1', '0'],
//!                 ['1', '1', '0', '0', '0'],
//!                 ['0', '0', '0', '0', '0']]
//! Output: 1
//!
//! Input:  grid = [['1', '1', '0', '0', '0'],
//!                 ['1', '1', '0', '0', '0'],
//!                 ['0', '0', '1', '0', '0'],
//!                 ['0', '0', '0', '1', '1']]
//! Output: 3
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 300`
//! - `grid[i][j]` is `'0'` or `'1'`.
//!
//! ## Hints
//!
//! 1. Scan the grid. The first time you meet a land cell, you have found a new island.
//! 2. Before continuing the scan, visit that whole island (DFS or BFS) and mark it, so its other
//!    cells are not counted again.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** count connected components with DFS: scan every cell. A `'1'` that has not been
//! visited starts a new island: increment the count, then "sink" the entire island by turning each
//! of its cells into `'0'` with an explicit-stack DFS over the 4 neighbours. Sunk cells are never
//! counted again, so each island is counted exactly once. The function owns `grid`, so changing it
//! is allowed and replaces a separate `visited` matrix.
//!
//! **Complexity:** `O(m * n)` time (each cell is sunk at most once), `O(m * n)` space for the
//! stack in the worst case.
//!
//! **Rust concepts:**
//! - The parameter is an owned `Vec<Vec<char>>`, so `let mut grid = grid;` lets the function
//!   modify it without affecting the caller.
//! - `char` literals such as `'1'` compare with `==` directly.
//! - The neighbour loop is the same `checked_add_signed` pattern as in the previous problems.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        // @solution-start
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        let mut islands = 0;
        for r in 0..m {
            for c in 0..n {
                if grid[r][c] != '1' {
                    continue;
                }
                islands += 1;
                // Sink the whole island so it is not counted again.
                grid[r][c] = '0';
                let mut stack = vec![(r, c)];
                while let Some((i, j)) = stack.pop() {
                    for (di, dj) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let (Some(ni), Some(nj)) =
                            (i.checked_add_signed(di), j.checked_add_signed(dj))
                        else {
                            continue;
                        };
                        if ni < m && nj < n && grid[ni][nj] == '1' {
                            grid[ni][nj] = '0';
                            stack.push((ni, nj));
                        }
                    }
                }
            }
        }
        islands
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|row| row.chars().collect()).collect()
    }

    #[test]
    fn example_one_island() {
        let grid = parse(&["11110", "11010", "11000", "00000"]);
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn example_three_islands() {
        let grid = parse(&["11000", "11000", "00100", "00011"]);
        assert_eq!(Solution::num_islands(grid), 3);
    }

    #[test]
    fn only_water_or_only_land() {
        assert_eq!(Solution::num_islands(parse(&["000", "000"])), 0);
        assert_eq!(Solution::num_islands(parse(&["111", "111"])), 1);
    }

    #[test]
    fn diagonal_cells_are_separate_islands() {
        assert_eq!(Solution::num_islands(parse(&["101", "010", "101"])), 5);
    }

    #[test]
    fn island_with_a_lake() {
        assert_eq!(Solution::num_islands(parse(&["111", "101", "111"])), 1);
    }

    #[test]
    fn large_checkerboard() {
        let grid: Vec<Vec<char>> = (0..300)
            .map(|r| {
                (0..300)
                    .map(|c| if (r + c) % 2 == 0 { '1' } else { '0' })
                    .collect()
            })
            .collect();
        assert_eq!(Solution::num_islands(grid), 45_000);
    }

    #[test]
    fn large_single_snake_island() {
        // Every other row is land; the rows are joined alternately at the right and left ends.
        let grid: Vec<Vec<char>> = (0..299)
            .map(|r| {
                (0..300)
                    .map(|c| {
                        let joined = (r % 4 == 1 && c == 299) || (r % 4 == 3 && c == 0);
                        if r % 2 == 0 || joined { '1' } else { '0' }
                    })
                    .collect()
            })
            .collect();
        assert_eq!(Solution::num_islands(grid), 1);
    }
}
