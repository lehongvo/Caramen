//! # Problem 0786: Rotting Oranges
//!
//! - **Difficulty:** Medium
//! - **Topics:** multi-source BFS level by level, `VecDeque`, counting what is left
//! - **Source:** [LeetCode 994. Rotting Oranges](https://leetcode.com/problems/rotting-oranges/)
//!
//! ## Problem
//!
//! In an `m x n` grid, each cell is `0` (empty), `1` (a fresh orange) or `2` (a rotten orange).
//! Every minute, each fresh orange that is 4-directionally adjacent to a rotten orange becomes
//! rotten.
//!
//! Return the minimum number of minutes until no fresh orange is left, or `-1` if some fresh
//! orange can never rot. If there are no fresh oranges at the start, the answer is `0`.
//!
//! Implement `Solution::oranges_rotting(grid: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  grid = [[2, 1, 1],
//!                 [1, 1, 0],
//!                 [0, 1, 1]]
//! Output: 4
//!
//! Input:  grid = [[2, 1, 1],
//!                 [0, 1, 1],
//!                 [1, 0, 1]]
//! Output: -1            (the orange in the bottom-left corner is never reached)
//!
//! Input:  grid = [[0, 2]]
//! Output: 0
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 10`
//! - `grid[i][j]` is `0`, `1` or `2`.
//!
//! ## Hints
//!
//! 1. All rotten oranges spread at the same time: start a breadth-first search from **all** of
//!    them at once.
//! 2. Process the queue one level (one minute) at a time: the oranges added during a minute are the
//!    ones that rot in that minute.
//! 3. Count the fresh oranges up front, and decrease the count as they rot.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** multi-source BFS: put every initially rotten orange in a queue; this is level 0.
//! Each minute, pop exactly the oranges of the current level (the queue length at the start of the
//! minute) and rot their fresh neighbours, pushing them as the next level. BFS visits cells in
//! order of distance from the **nearest** rotten orange, which is exactly when each orange rots.
//! Stop when no fresh orange is left (answer: the minutes elapsed) or the queue runs empty with
//! fresh oranges remaining (answer: `-1`).
//!
//! **Complexity:** `O(m * n)` time and space.
//!
//! **Rust concepts:**
//! - [`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html) is the queue:
//!   `push_back` and `pop_front` are both `O(1)` (a `Vec` would need `remove(0)`, which is `O(n)`).
//! - `for _ in 0..queue.len()` evaluates the length once, so the oranges pushed during this minute
//!   are processed in the next one.
// @solution-end

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        // @solution-start
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut fresh = 0;
        for (r, row) in grid.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                match cell {
                    2 => queue.push_back((r, c)),
                    1 => fresh += 1,
                    _ => {}
                }
            }
        }
        let mut minutes = 0;
        while fresh > 0 && !queue.is_empty() {
            minutes += 1;
            for _ in 0..queue.len() {
                let Some((r, c)) = queue.pop_front() else {
                    break;
                };
                for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc))
                    else {
                        continue;
                    };
                    if nr < m && nc < n && grid[nr][nc] == 1 {
                        grid[nr][nc] = 2;
                        fresh -= 1;
                        queue.push_back((nr, nc));
                    }
                }
            }
        }
        if fresh == 0 { minutes } else { -1 }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_four_minutes() {
        let grid = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(Solution::oranges_rotting(grid), 4);
    }

    #[test]
    fn example_unreachable_orange() {
        let grid = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
        assert_eq!(Solution::oranges_rotting(grid), -1);
    }

    #[test]
    fn example_nothing_fresh() {
        assert_eq!(Solution::oranges_rotting(vec![vec![0, 2]]), 0);
        assert_eq!(Solution::oranges_rotting(vec![vec![0]]), 0);
    }

    #[test]
    fn fresh_orange_without_rotten_ones() {
        assert_eq!(Solution::oranges_rotting(vec![vec![1]]), -1);
    }

    #[test]
    fn several_sources_spread_together() {
        // Rotten oranges at both ends meet in the middle after 2 minutes.
        assert_eq!(Solution::oranges_rotting(vec![vec![2, 1, 1, 1, 2]]), 2);
    }

    #[test]
    fn largest_grid_from_one_corner() {
        let mut grid = vec![vec![1; 10]; 10];
        grid[0][0] = 2;
        assert_eq!(Solution::oranges_rotting(grid), 18);
    }
}
