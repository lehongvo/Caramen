// ==================================================================
// PRACTICE 0071: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0071_find_a_value_in_a_grid.md
// Run the tests:          cargo test practice::p0071
// ==================================================================

//! # Problem 0071: Find a Value in a Grid
//!
//! - **Difficulty:** Easy
//! - **Topics:** loop labels, `break 'outer`, `continue 'outer`, `[[i32; 4]; 3]`, `enumerate`
//! - **Source:** Original · [The Rust Book 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html#loop-labels-to-disambiguate-between-multiple-loops)
//!
//! ## Problem
//!
//! A warehouse stores item codes in a fixed grid of 3 rows and 4 columns, `[[i32; 4]; 3]`. Write two
//! functions that scan the grid with nested loops:
//!
//! 1. `find_first(grid: &[[i32; 4]; 3], target: i32) -> Option<(usize, usize)>` returns the
//!    `(row, column)` of the **first** cell equal to `target`, scanning row by row from the top and,
//!    inside a row, from left to right. Return `None` if the value does not appear. Stop scanning as
//!    soon as you find it.
//! 2. `rows_containing(grid: &[[i32; 4]; 3], target: i32) -> usize` returns how many rows contain
//!    `target` at least once. As soon as a row is known to contain it, move on to the next row.
//!
//! Use a **labeled** outer loop (`'rows: for ...`) with `break 'rows` in the first function and
//! `continue 'rows` in the second.
//!
//! ## Examples
//!
//! ```text
//! grid = [[1, 2, 3, 4],
//!         [5, 6, 7, 2],
//!         [2, 2, 9, 0]]
//!
//! find_first(&grid, 2)      -> Some((0, 1))
//! find_first(&grid, 9)      -> Some((2, 2))
//! find_first(&grid, 42)     -> None
//! rows_containing(&grid, 2) -> 3
//! rows_containing(&grid, 7) -> 1
//! ```
//!
//! ## Constraints
//!
//! - Any `i32` values.
//!
//! ## Hints
//!
//! 1. A plain `break` inside the inner loop only leaves the inner loop.
//! 2. Labels start with a single quote: `'rows: for (r, row) in grid.iter().enumerate() { ... }`.

pub fn find_first(grid: &[[i32; 4]; 3], target: i32) -> Option<(usize, usize)> {
    todo!("Write your solution here")
}

pub fn rows_containing(grid: &[[i32; 4]; 3], target: i32) -> usize {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRID: [[i32; 4]; 3] = [[1, 2, 3, 4], [5, 6, 7, 2], [2, 2, 9, 0]];

    #[test]
    fn finds_the_first_occurrence() {
        assert_eq!(find_first(&GRID, 2), Some((0, 1)));
        assert_eq!(find_first(&GRID, 9), Some((2, 2)));
        assert_eq!(find_first(&GRID, 1), Some((0, 0)));
        assert_eq!(find_first(&GRID, 0), Some((2, 3)));
    }

    #[test]
    fn missing_value() {
        assert_eq!(find_first(&GRID, 42), None);
        assert_eq!(rows_containing(&GRID, 42), 0);
    }

    #[test]
    fn counts_rows_not_cells() {
        assert_eq!(rows_containing(&GRID, 2), 3);
        assert_eq!(rows_containing(&GRID, 7), 1);
        let same = [[5; 4]; 3];
        assert_eq!(rows_containing(&same, 5), 3);
    }

    #[test]
    fn negative_and_extreme_values() {
        let grid = [[0, 0, 0, i32::MIN], [-1, -1, 0, 0], [0, 0, 0, i32::MAX]];
        assert_eq!(find_first(&grid, i32::MIN), Some((0, 3)));
        assert_eq!(find_first(&grid, -1), Some((1, 0)));
        assert_eq!(find_first(&grid, i32::MAX), Some((2, 3)));
        assert_eq!(rows_containing(&grid, 0), 3);
    }
}
