//! # Problem 0986: Sudoku Solver
//!
//! - **Difficulty:** Hard
//! - **Topics:** backtracking with bitmasks per row/column/box, choosing the most constrained cell, `&mut Vec<Vec<char>>`, `count_ones`
//! - **Source:** [LeetCode 37. Sudoku Solver](https://leetcode.com/problems/sudoku-solver/)
//!
//! ## Problem
//!
//! Fill the empty cells (`'.'`) of a 9 x 9 Sudoku board **in place** so that every row, every column
//! and each of the nine 3 x 3 boxes contains the digits `'1'` to `'9'` exactly once. The puzzle is
//! guaranteed to have exactly one solution.
//!
//! Implement `Solution::solve_sudoku(board: &mut Vec<Vec<char>>)`.
//!
//! ## Examples
//!
//! ```text
//! Input:                         Output:
//! 5 3 . | . 7 . | . . .          5 3 4 | 6 7 8 | 9 1 2
//! 6 . . | 1 9 5 | . . .          6 7 2 | 1 9 5 | 3 4 8
//! . 9 8 | . . . | . 6 .          1 9 8 | 3 4 2 | 5 6 7
//! ------+-------+------          ------+-------+------
//! 8 . . | . 6 . | . . 3          8 5 9 | 7 6 1 | 4 2 3
//! 4 . . | 8 . 3 | . . 1          4 2 6 | 8 5 3 | 7 9 1
//! 7 . . | . 2 . | . . 6          7 1 3 | 9 2 4 | 8 5 6
//! ------+-------+------          ------+-------+------
//! . 6 . | . . . | 2 8 .          9 6 1 | 5 3 7 | 2 8 4
//! . . . | 4 1 9 | . . 5          2 8 7 | 4 1 9 | 6 3 5
//! . . . | . 8 . | . 7 9          3 4 5 | 2 8 6 | 1 7 9
//! ```
//!
//! ## Constraints
//!
//! - `board` is 9 x 9; cells are digits `'1'..='9'` or `'.'`; exactly one solution exists.
//!
//! ## Hints
//!
//! 1. Keep three arrays of 9-bit masks: the digits already used in each row, column and box
//!    (`box = (r / 3) * 3 + c / 3`).
//! 2. At each step, fill the empty cell with the **fewest** candidates (fewest free bits): it prunes
//!    the search enormously.
//! 3. Undo the masks when a choice leads to a dead end.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** build the masks, collect the empty cells, and recurse: pick the empty cell whose
//! candidate mask `!(row | col | box)` has the fewest bits, try each candidate, and backtrack on
//! failure. With the minimum-remaining-values heuristic, typical puzzles need only a few hundred
//! steps.
//!
//! **Complexity:** exponential in the worst case, fast in practice; `O(81)` extra space.
//!
//! **Rust concepts:**
//! - LeetCode's signature takes `&mut Vec<Vec<char>>`; Clippy's `ptr_arg` lint would prefer
//!   `&mut [Vec<char>]`, so it is allowed on the method.
//! - `char::from(b'0' + d)` and `c.to_digit(10)` convert between digits and characters.
//! - `swap_remove` takes a cell out of the "empty" list in `O(1)`; putting it back with `push` and
//!   `swap` restores the list after backtracking.
// @solution-end

pub struct Solution;

// @solution-start remove
struct Sudoku {
    rows: [u16; 9],
    cols: [u16; 9],
    boxes: [u16; 9],
    empty: Vec<(usize, usize)>,
}

impl Sudoku {
    fn candidates(&self, r: usize, c: usize) -> u16 {
        !(self.rows[r] | self.cols[c] | self.boxes[r / 3 * 3 + c / 3]) & 0x1FF
    }

    fn toggle(&mut self, r: usize, c: usize, bit: u16) {
        self.rows[r] ^= bit;
        self.cols[c] ^= bit;
        self.boxes[r / 3 * 3 + c / 3] ^= bit;
    }

    fn solve(&mut self, board: &mut [Vec<char>]) -> bool {
        let Some(best) = (0..self.empty.len()).min_by_key(|&i| {
            self.candidates(self.empty[i].0, self.empty[i].1)
                .count_ones()
        }) else {
            return true;
        };
        let (r, c) = self.empty.swap_remove(best);
        let mut options = self.candidates(r, c);
        while options != 0 {
            let bit = options & options.wrapping_neg();
            options &= options - 1;
            self.toggle(r, c, bit);
            board[r][c] = char::from(b'1' + bit.trailing_zeros() as u8);
            if self.solve(board) {
                return true;
            }
            self.toggle(r, c, bit);
        }
        board[r][c] = '.';
        // Put the cell back where it was so the caller's indices stay valid.
        self.empty.push((r, c));
        let last = self.empty.len() - 1;
        self.empty.swap(best, last);
        false
    }
}
// @solution-end

impl Solution {
    #[allow(clippy::ptr_arg)] // LeetCode's exact signature.
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        // @solution-start
        let mut sudoku = Sudoku {
            rows: [0; 9],
            cols: [0; 9],
            boxes: [0; 9],
            empty: Vec::new(),
        };
        for (r, row) in board.iter().enumerate() {
            for (c, cell) in row.iter().enumerate() {
                match cell.to_digit(10) {
                    Some(d) => sudoku.toggle(r, c, 1 << (d - 1)),
                    None => sudoku.empty.push((r, c)),
                }
            }
        }
        sudoku.solve(board);
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(rows: [&str; 9]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    fn is_solved(board: &[Vec<char>]) -> bool {
        let full = |cells: Vec<char>| {
            let mut sorted = cells;
            sorted.sort_unstable();
            sorted == ['1', '2', '3', '4', '5', '6', '7', '8', '9']
        };
        (0..9).all(|i| {
            full(board[i].clone())
                && full((0..9).map(|r| board[r][i]).collect())
                && full(
                    (0..9)
                        .map(|k| board[i / 3 * 3 + k / 3][i % 3 * 3 + k % 3])
                        .collect(),
                )
        })
    }

    #[test]
    fn example_puzzle() {
        let mut board = parse([
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        Solution::solve_sudoku(&mut board);
        assert_eq!(
            board,
            parse([
                "534678912",
                "672195348",
                "198342567",
                "859761423",
                "426853791",
                "713924856",
                "961537284",
                "287419635",
                "345286179",
            ])
        );
    }

    #[test]
    fn hard_puzzle() {
        // A puzzle with only 17 clues.
        let mut board = parse([
            ".......1.",
            "4........",
            ".2.......",
            "....5.4.7",
            "..8...3..",
            "..1.9....",
            "3..4..2..",
            ".5.1.....",
            "...8.6...",
        ]);
        let clues: Vec<(usize, usize, char)> = (0..9)
            .flat_map(|r| (0..9).map(move |c| (r, c)))
            .filter(|&(r, c)| board[r][c] != '.')
            .map(|(r, c)| (r, c, board[r][c]))
            .collect();
        Solution::solve_sudoku(&mut board);
        assert!(is_solved(&board));
        assert!(clues.iter().all(|&(r, c, ch)| board[r][c] == ch));
    }

    #[test]
    fn almost_solved_board() {
        let solved = parse([
            "534678912",
            "672195348",
            "198342567",
            "859761423",
            "426853791",
            "713924856",
            "961537284",
            "287419635",
            "345286179",
        ]);
        let mut board = solved.clone();
        board[0][0] = '.';
        board[8][8] = '.';
        board[4][4] = '.';
        Solution::solve_sudoku(&mut board);
        assert_eq!(board, solved);
    }
}
