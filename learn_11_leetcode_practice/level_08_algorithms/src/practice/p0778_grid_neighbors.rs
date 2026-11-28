// ==================================================================
// PRACTICE 0778: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0778_grid_neighbors.md
// Run the tests:          cargo test practice::p0778
// ==================================================================

//! # Problem 0778: Grid Neighbors
//!
//! - **Difficulty:** Easy
//! - **Topics:** returning `impl Iterator`, `usize::checked_add_signed`, `filter_map` with `?`, `bool::then_some`
//! - **Source:** Original · [`usize::checked_add_signed`](https://doc.rust-lang.org/std/primitive.usize.html#method.checked_add_signed) · [`impl Trait`](https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits)
//!
//! ## Problem
//!
//! Every grid search in the next problems needs "the neighbours of cell `(r, c)` that lie inside
//! the grid". With `usize` indices, the naive `r - 1` underflows at `r == 0` (a panic in debug
//! builds). Write two small iterator functions that do this safely, for a grid of `rows x cols`
//! cells and a cell `(r, c)` inside it:
//!
//! - `neighbors4(rows, cols, r, c)` yields the in-bounds cells among up, right, down, left, **in
//!   that order**: `(r - 1, c)`, `(r, c + 1)`, `(r + 1, c)`, `(r, c - 1)`.
//! - `neighbors8(rows, cols, r, c)` yields the in-bounds cells among the 8 surrounding cells
//!   (diagonals included), in **row-major order**: first the row above from left to right, then the
//!   left and right cells, then the row below from left to right.
//!
//! Both return `impl Iterator<Item = (usize, usize)>` and must not allocate a `Vec`.
//!
//! ## Examples
//!
//! ```text
//! neighbors4(3, 3, 1, 1) -> (0, 1), (1, 2), (2, 1), (1, 0)
//! neighbors4(3, 3, 0, 0) -> (0, 1), (1, 0)
//! neighbors4(1, 1, 0, 0) -> (nothing)
//! neighbors8(3, 3, 0, 0) -> (0, 1), (1, 0), (1, 1)
//! neighbors8(3, 3, 1, 1) -> (0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= rows, cols`, and `r < rows`, `c < cols`.
//!
//! ## Hints
//!
//! 1. Store the directions as `(isize, isize)` offsets in a constant array.
//! 2. `r.checked_add_signed(-1)` returns `None` instead of underflowing. Inside a closure that
//!    returns an `Option`, the `?` operator turns that `None` into "skip this direction".
//! 3. The result must outlive the call: the closure has to own `rows`, `cols`, `r` and `c`
//!    (`move`).

pub fn neighbors4(
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
) -> impl Iterator<Item = (usize, usize)> {
    std::iter::from_fn(|| todo!())
}

pub fn neighbors8(
    rows: usize,
    cols: usize,
    r: usize,
    c: usize,
) -> impl Iterator<Item = (usize, usize)> {
    std::iter::from_fn(|| todo!())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_center_four() {
        let cells: Vec<_> = neighbors4(3, 3, 1, 1).collect();
        assert_eq!(cells, vec![(0, 1), (1, 2), (2, 1), (1, 0)]);
    }

    #[test]
    fn example_corner_four() {
        assert_eq!(
            neighbors4(3, 3, 0, 0).collect::<Vec<_>>(),
            vec![(0, 1), (1, 0)]
        );
        assert_eq!(
            neighbors4(3, 3, 2, 2).collect::<Vec<_>>(),
            vec![(1, 2), (2, 1)]
        );
    }

    #[test]
    fn example_single_cell_has_no_neighbors() {
        assert_eq!(neighbors4(1, 1, 0, 0).count(), 0);
        assert_eq!(neighbors8(1, 1, 0, 0).count(), 0);
    }

    #[test]
    fn example_eight_directions() {
        assert_eq!(
            neighbors8(3, 3, 0, 0).collect::<Vec<_>>(),
            vec![(0, 1), (1, 0), (1, 1)]
        );
        assert_eq!(
            neighbors8(3, 3, 1, 1).collect::<Vec<_>>(),
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2)
            ]
        );
    }

    #[test]
    fn single_row_grid() {
        assert_eq!(
            neighbors4(1, 5, 0, 2).collect::<Vec<_>>(),
            vec![(0, 3), (0, 1)]
        );
        assert_eq!(neighbors8(1, 5, 0, 4).collect::<Vec<_>>(), vec![(0, 3)]);
    }

    #[test]
    fn neighbor_counts_over_a_large_grid() {
        let (rows, cols) = (100, 100);
        let mut total4 = 0;
        let mut total8 = 0;
        for r in 0..rows {
            for c in 0..cols {
                total4 += neighbors4(rows, cols, r, c).count();
                total8 += neighbors8(rows, cols, r, c).count();
            }
        }
        // Each horizontal/vertical adjacent pair is counted from both sides: 2 * (2 * 100 * 99).
        assert_eq!(total4, 39_600);
        // Diagonal pairs add 2 * (2 * 99 * 99) more.
        assert_eq!(total8, 39_600 + 39_204);
    }
}
