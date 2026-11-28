//! # Problem 0054: Tic-Tac-Toe Winner
//!
//! - **Difficulty:** Easy
//! - **Topics:** `[[char; 3]; 3]`, a `const` array of tuples, destructuring tuples in a `for` loop, `Option<char>`
//! - **Source:** Original · [The Rust Book 3.2](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types)
//!
//! ## Problem
//!
//! A tic-tac-toe board is a 3x3 array of characters: `'X'`, `'O'`, or `'.'` for an empty cell.
//! A player wins with three of their marks on one **line**: one of the 3 rows, 3 columns or 2
//! diagonals.
//!
//! Write `winner(board: [[char; 3]; 3]) -> Option<char>` that returns `Some('X')` or `Some('O')` if
//! that player has a complete line, and `None` if nobody has one. You may assume that at most one
//! player has a complete line (a player may have two lines at once).
//!
//! ## Examples
//!
//! ```text
//! [['X', 'X', 'X'],
//!  ['O', 'O', '.'],
//!  ['.', '.', '.']]   -> Some('X')        top row
//!
//! [['X', 'O', '.'],
//!  ['X', 'O', '.'],
//!  ['.', 'O', 'X']]   -> Some('O')        middle column
//!
//! [['X', 'O', 'X'],
//!  ['X', 'O', 'O'],
//!  ['O', 'X', 'X']]   -> None             a draw
//! ```
//!
//! ## Constraints
//!
//! - Every cell is `'X'`, `'O'` or `'.'`.
//!
//! ## Hints
//!
//! 1. List the 8 lines once, as coordinates: `[(0, 0), (0, 1), (0, 2)]` is the top row.
//! 2. A line is won when its first cell is not `'.'` and the three cells are equal.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** store the 8 winning lines in a `const` array of coordinate triples. For each line,
//! read the three cells; if they are equal and not empty, that mark is the winner.
//!
//! **Rust concepts:**
//! - `const LINES: [[(usize, usize); 3]; 8]` is an array of 8 lines, each an array of 3 `(row, col)`
//!   tuples. Data tables like this replace a long chain of `if` statements.
//! - `for [(r0, c0), (r1, c1), (r2, c2)] in LINES` destructures an array **and** the tuples inside
//!   it directly in the loop pattern.
//! - Comparing three values needs two comparisons: `a == b && b == c` (Rust has no `a == b == c`).
// @solution-end

// @solution-start remove
const LINES: [[(usize, usize); 3]; 8] = [
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    [(0, 0), (1, 1), (2, 2)],
    [(0, 2), (1, 1), (2, 0)],
];
// @solution-end

pub fn winner(board: [[char; 3]; 3]) -> Option<char> {
    // @solution-start
    for [(r0, c0), (r1, c1), (r2, c2)] in LINES {
        let a = board[r0][c0];
        if a != '.' && a == board[r1][c1] && a == board[r2][c2] {
            return Some(a);
        }
    }
    None
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_win() {
        let board = [['X', 'X', 'X'], ['O', 'O', '.'], ['.', '.', '.']];
        assert_eq!(winner(board), Some('X'));
    }

    #[test]
    fn column_win() {
        let board = [['X', 'O', '.'], ['X', 'O', '.'], ['.', 'O', 'X']];
        assert_eq!(winner(board), Some('O'));
    }

    #[test]
    fn diagonal_wins() {
        let main = [['O', 'X', 'X'], ['.', 'O', 'X'], ['.', '.', 'O']];
        assert_eq!(winner(main), Some('O'));
        let anti = [['O', 'O', 'X'], ['.', 'X', '.'], ['X', '.', '.']];
        assert_eq!(winner(anti), Some('X'));
    }

    #[test]
    fn draw_and_empty_board() {
        let draw = [['X', 'O', 'X'], ['X', 'O', 'O'], ['O', 'X', 'X']];
        assert_eq!(winner(draw), None);
        assert_eq!(winner([['.'; 3]; 3]), None);
    }

    #[test]
    fn two_lines_for_the_same_player() {
        let board = [['X', 'X', 'X'], ['X', 'O', 'O'], ['X', 'O', 'O']];
        assert_eq!(winner(board), Some('X'));
    }
}
