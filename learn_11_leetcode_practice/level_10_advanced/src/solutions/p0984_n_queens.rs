//! # Problem 0984: N-Queens
//!
//! - **Difficulty:** Hard
//! - **Topics:** backtracking row by row, column and diagonal occupancy with `Vec<bool>`, building boards as `Vec<String>`
//! - **Source:** [LeetCode 51. N-Queens](https://leetcode.com/problems/n-queens/)
//!
//! ## Problem
//!
//! Place `n` queens on an `n x n` chessboard so that no two attack each other (no two share a row,
//! a column or a diagonal). Return **all** distinct solutions. Each solution is a board: `n` strings
//! of length `n`, with `'Q'` for a queen and `'.'` for an empty square. Solutions may be returned in
//! any order.
//!
//! Implement `Solution::solve_n_queens(n: i32) -> Vec<Vec<String>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  n = 4
//! Output: [[".Q..", "...Q", "Q...", "..Q."],
//!          ["..Q.", "Q...", "...Q", ".Q.."]]
//!
//! Input:  n = 1
//! Output: [["Q"]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 9`
//!
//! ## Hints
//!
//! 1. Exactly one queen per row: choose its column row by row.
//! 2. A square `(row, col)` is attacked through its column, its diagonal `row - col` and its
//!    anti-diagonal `row + col`; shift `row - col` by `n - 1` to index a `Vec<bool>`.
//! 3. After placing the last row, turn the chosen columns into strings.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** a recursive search places one queen per row. Three boolean vectors (columns, diagonals,
//! anti-diagonals) make the safety test `O(1)`; placing and removing a queen updates them, which is
//! the "undo" step of backtracking.
//!
//! **Complexity:** `O(n!)` time in the worst case (far less thanks to pruning), `O(n)` extra space
//! besides the output.
//!
//! **Rust concepts:**
//! - A small `struct` holding the search state keeps the recursive method's signature short.
//! - `queens.push(col)` / `queens.pop()` mirror the choose / un-choose steps.
//! - Each board row is built with `(0..n).map(|c| if c == col { 'Q' } else { '.' }).collect()`.
// @solution-end

pub struct Solution;

// @solution-start remove
struct Search {
    n: usize,
    cols: Vec<bool>,
    diagonals: Vec<bool>,
    anti_diagonals: Vec<bool>,
    queens: Vec<usize>,
    boards: Vec<Vec<String>>,
}

impl Search {
    fn place(&mut self, row: usize) {
        if row == self.n {
            let board = self
                .queens
                .iter()
                .map(|&col| {
                    (0..self.n)
                        .map(|c| if c == col { 'Q' } else { '.' })
                        .collect()
                })
                .collect();
            self.boards.push(board);
            return;
        }
        for col in 0..self.n {
            let (d, a) = (row + self.n - 1 - col, row + col);
            if self.cols[col] || self.diagonals[d] || self.anti_diagonals[a] {
                continue;
            }
            self.cols[col] = true;
            self.diagonals[d] = true;
            self.anti_diagonals[a] = true;
            self.queens.push(col);
            self.place(row + 1);
            self.queens.pop();
            self.cols[col] = false;
            self.diagonals[d] = false;
            self.anti_diagonals[a] = false;
        }
    }
}
// @solution-end

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        // @solution-start
        let n = n as usize;
        let mut search = Search {
            n,
            cols: vec![false; n],
            diagonals: vec![false; 2 * n - 1],
            anti_diagonals: vec![false; 2 * n - 1],
            queens: Vec::with_capacity(n),
            boards: Vec::new(),
        };
        search.place(0);
        search.boards
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(board: &[String]) -> bool {
        let queens: Vec<(i32, i32)> = board
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.char_indices()
                    .filter(|&(_, ch)| ch == 'Q')
                    .map(move |(c, _)| (r as i32, c as i32))
            })
            .collect();
        queens.len() == board.len()
            && queens.iter().enumerate().all(|(i, &(r1, c1))| {
                queens[i + 1..]
                    .iter()
                    .all(|&(r2, c2)| r1 != r2 && c1 != c2 && (r1 - r2).abs() != (c1 - c2).abs())
            })
    }

    #[test]
    fn four_queens() {
        let mut boards = Solution::solve_n_queens(4);
        boards.sort();
        assert_eq!(
            boards,
            vec![
                vec!["..Q.", "Q...", "...Q", ".Q.."],
                vec![".Q..", "...Q", "Q...", "..Q."]
            ]
        );
    }

    #[test]
    fn tiny_boards() {
        assert_eq!(Solution::solve_n_queens(1), vec![vec!["Q"]]);
        assert!(Solution::solve_n_queens(2).is_empty());
        assert!(Solution::solve_n_queens(3).is_empty());
    }

    #[test]
    fn counts_and_validity() {
        for (n, expected) in [(5, 10), (6, 4), (7, 40), (8, 92), (9, 352)] {
            let boards = Solution::solve_n_queens(n);
            assert_eq!(boards.len(), expected, "n = {n}");
            assert!(boards.iter().all(|b| is_valid(b)));
            let mut unique = boards.clone();
            unique.sort();
            unique.dedup();
            assert_eq!(unique.len(), expected);
        }
    }
}
