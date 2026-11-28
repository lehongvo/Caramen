// ==================================================================
// PRACTICE 0789: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0789_word_search.md
// Run the tests:          cargo test practice::p0789
// ==================================================================

//! # Problem 0789: Word Search
//!
//! - **Difficulty:** Medium
//! - **Topics:** DFS with backtracking on a grid, temporary marking and restoring, pruning, `Iterator::any`
//! - **Source:** [LeetCode 79. Word Search](https://leetcode.com/problems/word-search/)
//!
//! ## Problem
//!
//! Given an `m x n` grid of letters `board` and a string `word`, return `true` if `word` can be
//! spelled by a path of cells where each next cell is **horizontally or vertically** adjacent to
//! the previous one. A cell may be used **at most once** in the path.
//!
//! Implement `Solution::exist(board: Vec<Vec<char>>, word: String) -> bool`.
//!
//! ## Examples
//!
//! ```text
//! board = [['A', 'B', 'C', 'E'],
//!          ['S', 'F', 'C', 'S'],
//!          ['A', 'D', 'E', 'E']]
//!
//! Input:  word = "ABCCED"
//! Output: true
//!
//! Input:  word = "SEE"
//! Output: true
//!
//! Input:  word = "ABCB"
//! Output: false         (the path would need the 'B' twice)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 6`
//! - `1 <= word.len() <= 15`
//! - `board` and `word` contain only uppercase and lowercase English letters.
//!
//! ## Hints
//!
//! 1. Try every cell as a start. From a cell that matches the current letter, try the four
//!    neighbours for the next letter.
//! 2. To prevent reusing a cell, overwrite it with a marker before going deeper and **restore** it
//!    afterwards (backtracking).
//! 3. Cheap pruning: if the board does not contain enough copies of some letter, answer `false`
//!    immediately.

pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Vec<char>> {
        ["ABCE", "SFCS", "ADEE"]
            .iter()
            .map(|row| row.chars().collect())
            .collect()
    }

    fn run(board: Vec<Vec<char>>, word: &str) -> bool {
        Solution::exist(board, word.to_string())
    }

    #[test]
    fn example_long_path() {
        assert!(run(sample(), "ABCCED"));
    }

    #[test]
    fn example_short_path() {
        assert!(run(sample(), "SEE"));
    }

    #[test]
    fn example_cell_reuse_is_not_allowed() {
        assert!(!run(sample(), "ABCB"));
    }

    #[test]
    fn single_cell_board() {
        assert!(run(vec![vec!['a']], "a"));
        assert!(!run(vec![vec!['a']], "b"));
        assert!(!run(vec![vec!['a']], "aa"));
    }

    #[test]
    fn letters_are_case_sensitive() {
        assert!(!run(vec![vec!['a', 'B']], "ab"));
        assert!(run(vec![vec!['a', 'B']], "aB"));
    }

    #[test]
    fn marking_is_undone_between_attempts() {
        // From the 'C' the search first goes down to the 'E' at (1, 2) and fails after using the
        // three lower-right 'E's; the real path needs those cells again, so they must be restored.
        let board: Vec<Vec<char>> = ["ABCE", "SFES", "ADEE"]
            .iter()
            .map(|r| r.chars().collect())
            .collect();
        assert!(run(board, "ABCESEEEFS"));
    }

    #[test]
    fn worst_case_board_is_fast_thanks_to_pruning() {
        let mut board = vec![vec!['A'; 6]; 6];
        assert!(!run(board.clone(), "AAAAAAAAAAAAAAB")); // no 'B' at all
        board[5][5] = 'B';
        assert!(run(board.clone(), "AAAAAAAAAAAAAAB")); // searched from the single 'B'
        assert!(!run(board, "ABBBB"));
    }
}
