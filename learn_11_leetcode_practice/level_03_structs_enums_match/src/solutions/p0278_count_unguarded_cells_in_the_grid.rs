//! # Problem 0278: Count Unguarded Cells in the Grid
//!
//! - **Difficulty:** Medium
//! - **Topics:** a `Cell` enum grid, `matches!` to stop a ray, `while let` over an `Option` cursor, counting with `filter`
//! - **Source:** [LeetCode 2257. Count Unguarded Cells in the Grid](https://leetcode.com/problems/count-unguarded-cells-in-the-grid/)
//!
//! ## Problem
//!
//! An `m x n` grid contains some **guards** and some **walls**, given as lists of `[row, col]`
//! positions. Every guard watches in the four directions (up, down, left, right) as far as the
//! edge of the grid, but its view is stopped by a wall or by another guard. A cell is **guarded**
//! if at least one guard sees it.
//!
//! Return the number of cells that are **not** guarded and contain neither a guard nor a wall.
//!
//! Implement
//! `Solution::count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32`.
//!
//! For this level, store the grid as `Vec<Vec<Cell>>` with `enum Cell { Free, Guard, Wall, Watched }`.
//!
//! ## Examples
//!
//! ```text
//! Input:  m = 4, n = 6, guards = [[0,0],[1,1],[2,3]], walls = [[0,1],[2,2],[1,4]]
//! Output: 7
//!
//! Input:  m = 3, n = 3, guards = [[1,1]], walls = [[0,1],[1,0],[2,1],[1,2]]
//! Output: 4      (the guard is boxed in by walls; the four corners are unguarded)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 10^5`, `2 <= m * n <= 10^5`
//! - `1 <= guards.len() + walls.len() <= m * n`; all positions are distinct and inside the grid.
//!
//! ## Hints
//!
//! 1. Place all guards and walls first, then cast four rays from every guard.
//! 2. A ray stops at `matches!(cell, Cell::Guard | Cell::Wall)`; free cells along it become `Watched`.
//! 3. At the end, count the cells that are still `Free`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** mark guards and walls, then walk from every guard in each direction, turning `Free`
//! cells into `Watched` until a guard, a wall or the edge. The answer is the number of `Free` cells
//! left.
//!
//! **Complexity:** `O(m * n + g + w)` time: rays stop at guards, so every cell is crossed by at most
//! two horizontal and two vertical rays; `O(m * n)` space.
//!
//! **Rust concepts:**
//! - An enum grid documents what each cell is; `matches!(c, Cell::Guard | Cell::Wall)` reads as the
//!   rule "the view is blocked here".
//! - The ray walk uses `while let Some((r, c)) = next` with a helper `step` that returns `None` at the
//!   edge, so bounds checks live in one place.
//! - Watching an already watched cell is harmless, so rays may overlap.
// @solution-end

pub struct Solution;

// @solution-start remove
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Free,
    Guard,
    Wall,
    Watched,
}

/// The neighbour of `(r, c)` in direction `(dr, dc)`, if it is inside an `m x n` grid.
fn step(
    (r, c): (usize, usize),
    (dr, dc): (isize, isize),
    m: usize,
    n: usize,
) -> Option<(usize, usize)> {
    let nr = r.checked_add_signed(dr)?;
    let nc = c.checked_add_signed(dc)?;
    (nr < m && nc < n).then_some((nr, nc))
}

fn position(p: &[i32]) -> Option<(usize, usize)> {
    match p {
        &[r, c] => Some((r as usize, c as usize)),
        _ => None,
    }
}
// @solution-end

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        // @solution-start
        let (m, n) = (m as usize, n as usize);
        let mut grid = vec![vec![Cell::Free; n]; m];
        let guard_cells: Vec<(usize, usize)> = guards.iter().filter_map(|g| position(g)).collect();
        for &(r, c) in &guard_cells {
            grid[r][c] = Cell::Guard;
        }
        for (r, c) in walls.iter().filter_map(|w| position(w)) {
            grid[r][c] = Cell::Wall;
        }
        for &start in &guard_cells {
            for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let mut next = step(start, dir, m, n);
                while let Some((r, c)) = next {
                    if matches!(grid[r][c], Cell::Guard | Cell::Wall) {
                        break;
                    }
                    grid[r][c] = Cell::Watched;
                    next = step((r, c), dir, m, n);
                }
            }
        }
        let free = grid.iter().flatten().filter(|&&c| c == Cell::Free).count();
        // At most 10^5 cells.
        free as i32
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cells(list: &[[i32; 2]]) -> Vec<Vec<i32>> {
        list.iter().map(|p| p.to_vec()).collect()
    }

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::count_unguarded(
                4,
                6,
                cells(&[[0, 0], [1, 1], [2, 3]]),
                cells(&[[0, 1], [2, 2], [1, 4]])
            ),
            7
        );
    }

    #[test]
    fn example_boxed_in_guard() {
        assert_eq!(
            Solution::count_unguarded(
                3,
                3,
                cells(&[[1, 1]]),
                cells(&[[0, 1], [1, 0], [2, 1], [1, 2]])
            ),
            4
        );
    }

    #[test]
    fn guards_block_each_other() {
        // Row of 5: G G . . .  The second guard sees the rest of the row; nothing is free.
        assert_eq!(
            Solution::count_unguarded(1, 5, cells(&[[0, 0], [0, 1]]), vec![]),
            0
        );
        // 2 x 3 with a wall between: G W .
        //                            . . .
        assert_eq!(
            Solution::count_unguarded(2, 3, cells(&[[0, 0]]), cells(&[[0, 1]])),
            3
        );
    }

    #[test]
    fn long_thin_grid() {
        // 1 x 100_000 with one guard in the middle: everything is watched.
        assert_eq!(
            Solution::count_unguarded(1, 100_000, cells(&[[0, 50_000]]), vec![]),
            0
        );
        // A wall right next to the guard stops the view to the right.
        assert_eq!(
            Solution::count_unguarded(1, 100_000, cells(&[[0, 0]]), cells(&[[0, 1]])),
            99_998
        );
    }
}
