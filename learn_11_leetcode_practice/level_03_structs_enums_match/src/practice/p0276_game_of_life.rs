// ==================================================================
// PRACTICE 0276: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0276_game_of_life.md
// Run the tests:          cargo test practice::p0276
// ==================================================================

//! # Problem 0276: Game of Life
//!
//! - **Difficulty:** Medium
//! - **Topics:** a cell-state `enum` with four variants for in-place updates, `match (alive, neighbours)`, `&mut Vec<Vec<i32>>`
//! - **Source:** [LeetCode 289. Game of Life](https://leetcode.com/problems/game-of-life/)
//!
//! ## Problem
//!
//! Conway's Game of Life is played on an `m x n` grid of cells, each **alive** (`1`) or **dead**
//! (`0`). Every cell has up to eight neighbours (horizontal, vertical and diagonal). The next
//! generation is computed from the current one, **all cells at the same time**:
//!
//! 1. a live cell with fewer than two live neighbours dies (under-population);
//! 2. a live cell with two or three live neighbours stays alive;
//! 3. a live cell with more than three live neighbours dies (over-population);
//! 4. a dead cell with exactly three live neighbours becomes alive (reproduction).
//!
//! Update `board` **in place** to the next generation.
//!
//! Implement `Solution::game_of_life(board: &mut Vec<Vec<i32>>)`.
//!
//! The challenge of updating in place: when you decide a cell's fate, its neighbours must still
//! show their *old* state. For this level, use an enum with four states, `Dead`, `Alive`, `Dying`
//! (alive now, dead next) and `Born` (dead now, alive next), stored in the grid as the codes
//! `0`, `1`, `2`, `3`.
//!
//! ## Examples
//!
//! ```text
//! Input:  board = [[0, 1, 0],
//!                  [0, 0, 1],
//!                  [1, 1, 1],
//!                  [0, 0, 0]]
//! Output:         [[0, 0, 0],
//!                  [1, 0, 1],
//!                  [0, 1, 1],
//!                  [0, 1, 0]]
//!
//! Input:  board = [[1, 1],
//!                  [1, 0]]
//! Output:         [[1, 1],
//!                  [1, 1]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= m, n <= 25`; every cell is `0` or `1`.
//!
//! ## Hints
//!
//! 1. "Was alive in the old generation" is true for `Alive` and `Dying`: `matches!(state, Alive | Dying)`.
//! 2. The four rules become one `match (was_alive, live_neighbours)` returning the new state.
//! 3. A second pass turns `Dying` into `0` and `Born` into `1`.

pub struct Solution;

impl Solution {
    // LeetCode's signature uses `&mut Vec<Vec<i32>>`; a slice would be more idiomatic.
    #[allow(clippy::ptr_arg)]
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn step(mut board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Solution::game_of_life(&mut board);
        board
    }

    #[test]
    fn example_glider() {
        assert_eq!(
            step(vec![
                vec![0, 1, 0],
                vec![0, 0, 1],
                vec![1, 1, 1],
                vec![0, 0, 0]
            ]),
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]
        );
    }

    #[test]
    fn example_block_completes() {
        assert_eq!(
            step(vec![vec![1, 1], vec![1, 0]]),
            vec![vec![1, 1], vec![1, 1]]
        );
    }

    #[test]
    fn blinker_oscillates() {
        let horizontal = vec![vec![0, 0, 0], vec![1, 1, 1], vec![0, 0, 0]];
        let vertical = vec![vec![0, 1, 0], vec![0, 1, 0], vec![0, 1, 0]];
        assert_eq!(step(horizontal.clone()), vertical);
        assert_eq!(step(vertical), horizontal);
    }

    #[test]
    fn lonely_and_crowded_cells_die() {
        assert_eq!(step(vec![vec![1]]), vec![vec![0]]);
        assert_eq!(
            step(vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]]),
            vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]
        );
    }

    #[test]
    fn large_empty_board_stays_empty() {
        let board = vec![vec![0; 25]; 25];
        assert_eq!(step(board.clone()), board);
    }
}
