//! # Problem 0783: Surrounded Regions
//!
//! - **Difficulty:** Medium
//! - **Topics:** border flood fill with a temporary marker, in-place `&mut Vec<Vec<char>>`, `iter_mut().flatten()`
//! - **Source:** [LeetCode 130. Surrounded Regions](https://leetcode.com/problems/surrounded-regions/)
//!
//! ## Problem
//!
//! You are given an `m x n` board of `'X'` and `'O'` characters. A **region** is a group of `'O'`
//! cells connected 4-directionally. A region is **surrounded** when none of its cells is on the
//! border of the board. **Capture** every surrounded region by replacing all its `'O'` with `'X'`,
//! in place. Regions touching the border stay unchanged.
//!
//! Implement `Solution::solve(board: &mut Vec<Vec<char>>)`.
//!
//! ## Examples
//!
//! ```text
//! Input:  [['X', 'X', 'X', 'X'],          Output: [['X', 'X', 'X', 'X'],
//!          ['X', 'O', 'O', 'X'],                   ['X', 'X', 'X', 'X'],
//!          ['X', 'X', 'O', 'X'],                   ['X', 'X', 'X', 'X'],
//!          ['X', 'O', 'X', 'X']]                   ['X', 'O', 'X', 'X']]
//!         (the bottom 'O' is on the border, so it is not captured)
//!
//! Input:  [['X']]
//! Output: [['X']]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 200`
//! - `board[i][j]` is `'X'` or `'O'`.
//!
//! ## Hints
//!
//! 1. It is easier to find the regions that are **not** captured: they contain a border cell.
//! 2. Mark everything reachable from a border `'O'` with a third character, then make a final pass
//!    over the board.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** border flood fill + final pass: an `'O'` survives exactly when it is connected to an
//! `'O'` on the border. Flood from every border `'O'` (explicit-stack DFS) and mark what you reach
//! with a temporary `'#'`. Then one pass over the board: remaining `'O'` cells are surrounded and
//! become `'X'`; `'#'` cells go back to `'O'`. Using a marker inside the board avoids a separate
//! `visited` matrix.
//!
//! **Complexity:** `O(m * n)` time and space (the stack, in the worst case).
//!
//! **Rust concepts:**
//! - The board is borrowed mutably (`&mut Vec<Vec<char>>`, LeetCode's signature; Clippy's
//!   `ptr_arg` lint is allowed for it), so all changes are visible to the caller.
//! - `for cell in board.iter_mut().flatten()` visits every `&mut char` of a `Vec<Vec<char>>`, and a
//!   `match *cell` rewrites it.
// @solution-end

pub struct Solution;

impl Solution {
    // LeetCode's signature uses `&mut Vec<Vec<char>>`; Clippy would suggest `&mut [Vec<char>]`.
    #[allow(clippy::ptr_arg)]
    pub fn solve(board: &mut Vec<Vec<char>>) {
        // @solution-start
        let (m, n) = (board.len(), board[0].len());
        let mut stack: Vec<(usize, usize)> = (0..m)
            .flat_map(|r| [(r, 0), (r, n - 1)])
            .chain((0..n).flat_map(|c| [(0, c), (m - 1, c)]))
            .collect();
        while let Some((r, c)) = stack.pop() {
            if board[r][c] != 'O' {
                continue; // 'X', or already marked
            }
            board[r][c] = '#'; // connected to the border: safe
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc))
                else {
                    continue;
                };
                if nr < m && nc < n && board[nr][nc] == 'O' {
                    stack.push((nr, nc));
                }
            }
        }
        for cell in board.iter_mut().flatten() {
            *cell = match *cell {
                'O' => 'X', // surrounded: captured
                '#' => 'O', // safe: restore
                other => other,
            };
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board(rows: &[&str]) -> Vec<Vec<char>> {
        rows.iter().map(|row| row.chars().collect()).collect()
    }

    fn run(rows: &[&str]) -> Vec<Vec<char>> {
        let mut b = board(rows);
        Solution::solve(&mut b);
        b
    }

    #[test]
    fn example_one_region_captured() {
        assert_eq!(
            run(&["XXXX", "XOOX", "XXOX", "XOXX"]),
            board(&["XXXX", "XXXX", "XXXX", "XOXX"])
        );
    }

    #[test]
    fn example_single_cell() {
        assert_eq!(run(&["X"]), board(&["X"]));
        assert_eq!(run(&["O"]), board(&["O"]));
    }

    #[test]
    fn region_touching_the_border_survives() {
        assert_eq!(run(&["XOX", "XOX", "XXX"]), board(&["XOX", "XOX", "XXX"]));
    }

    #[test]
    fn nested_rings() {
        assert_eq!(
            run(&["OOOOO", "OXXXO", "OXOXO", "OXXXO", "OOOOO"]),
            board(&["OOOOO", "OXXXO", "OXXXO", "OXXXO", "OOOOO"])
        );
    }

    #[test]
    fn large_board() {
        // A border of 'X' around a block of 'O': everything inside is captured.
        let mut b = vec![vec!['O'; 200]; 200];
        for i in 0..200 {
            b[0][i] = 'X';
            b[199][i] = 'X';
            b[i][0] = 'X';
            b[i][199] = 'X';
        }
        let mut captured = b.clone();
        Solution::solve(&mut captured);
        assert!(captured.iter().flatten().all(|&ch| ch == 'X'));
        // With an opening in the border, nothing is captured.
        b[100][0] = 'O';
        let mut open = b.clone();
        Solution::solve(&mut open);
        assert_eq!(open, b);
    }
}
