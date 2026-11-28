//! # Problem 0324: Build a Grid
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Vec<Vec<T>>`, `vec![vec![x; cols]; rows]`, safe neighbour access with `get` and `checked_add_signed`
//! - **Source:** Original · [The Rust Book 8.1](https://doc.rust-lang.org/book/ch08-01-vectors.html) · [`usize::checked_add_signed`](https://doc.rust-lang.org/std/primitive.usize.html#method.checked_add_signed)
//!
//! ## Problem
//!
//! A board game stores its board as a 2-D vector of characters: `grid[r][c]` is the cell in row
//! `r` and column `c`. Implement:
//!
//! 1. `make_grid(rows: usize, cols: usize, fill: char) -> Vec<Vec<char>>` returns a board with
//!    `rows` rows of `cols` cells, all equal to `fill`. The rows must be independent: changing a
//!    cell of one row must not change other rows.
//! 2. `count_neighbors(grid: &[Vec<char>], r: usize, c: usize, target: char) -> usize` returns how
//!    many of the (up to) 8 cells around `(r, c)` (horizontally, vertically and diagonally
//!    adjacent, not the cell itself) are equal to `target`. Cells outside the board do not count,
//!    and the function must not panic for border cells. Rows may have different lengths.
//! 3. `render(grid: &[Vec<char>]) -> String` returns the board as text: each row's characters
//!    concatenated, rows separated by `'\n'` (no trailing newline).
//!
//! ## Examples
//!
//! ```text
//! let mut g = make_grid(2, 3, '.');      // ["...", "..."]
//! g[0][1] = '#';
//! g[1][2] = '#';
//! render(&g)                 => ".#.\n..#"
//! count_neighbors(&g, 0, 2, '#') => 2    (the cells (0, 1) and (1, 2))
//! count_neighbors(&g, 1, 0, '#') => 1    (only (0, 1) is adjacent)
//! count_neighbors(&g, 0, 0, '.') => 2    ((1, 0) and (1, 1); (0, 1) is '#')
//! ```
//!
//! ## Constraints
//!
//! - `0 <= rows, cols <= 100`
//! - In `count_neighbors`, `(r, c)` may be any cell of the board.
//!
//! ## Hints
//!
//! 1. `vec![row; n]` clones `row` `n` times, so `vec![vec![fill; cols]; rows]` creates independent
//!    rows.
//! 2. Neighbour offsets are `-1, 0, 1`. `r.checked_add_signed(-1)` is `None` for `r == 0` instead of
//!    underflowing.
//! 3. `grid.get(nr).and_then(|row| row.get(nc))` is `None` for cells outside the board.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `vec!` builds the grid in one expression. For neighbours, try the eight offsets and
//! let `Option` handle every "outside the board" case: `checked_add_signed` rejects negative
//! coordinates and `get` rejects coordinates past the end of a row or of the grid.
//!
//! **Rust concepts:**
//! - `vec![elem; n]` requires `elem: Clone` and clones it, unlike the "same reference repeated"
//!   pitfall of some other languages.
//! - Taking `&[Vec<char>]` accepts a `&Vec<Vec<char>>` through deref coercion.
//! - `let ... else { continue; }` (let-else) binds the coordinates or skips this offset.
//! - `row.iter().collect::<String>()` turns a row of `char`s into a `String`; collecting the rows
//!   into a `Vec<String>` and calling `join("\n")` adds the separators.
// @solution-end

/// Returns a `rows` x `cols` grid filled with `fill`.
pub fn make_grid(rows: usize, cols: usize, fill: char) -> Vec<Vec<char>> {
    // @solution-start
    vec![vec![fill; cols]; rows]
    // @solution-end
}

/// Counts the cells around `(r, c)` equal to `target`.
pub fn count_neighbors(grid: &[Vec<char>], r: usize, c: usize, target: char) -> usize {
    // @solution-start
    let mut count = 0;
    for dr in -1isize..=1 {
        for dc in -1isize..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc)) else {
                continue;
            };
            if grid.get(nr).and_then(|row| row.get(nc)) == Some(&target) {
                count += 1;
            }
        }
    }
    count
    // @solution-end
}

/// Renders the grid as lines of text separated by `'\n'`.
pub fn render(grid: &[Vec<char>]) -> String {
    // @solution-start
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_board() {
        let mut g = make_grid(2, 3, '.');
        assert_eq!(render(&g), "...\n...");
        g[0][1] = '#';
        g[1][2] = '#';
        assert_eq!(render(&g), ".#.\n..#");
        assert_eq!(count_neighbors(&g, 0, 2, '#'), 2);
        assert_eq!(count_neighbors(&g, 1, 0, '#'), 1);
        assert_eq!(count_neighbors(&g, 0, 0, '.'), 2);
    }

    #[test]
    fn rows_are_independent() {
        let mut g = make_grid(3, 2, 'o');
        g[1][0] = 'x';
        assert_eq!(g[0], vec!['o', 'o']);
        assert_eq!(g[1], vec!['x', 'o']);
        assert_eq!(g[2], vec!['o', 'o']);
    }

    #[test]
    fn empty_grids() {
        assert_eq!(make_grid(0, 5, '.'), Vec::<Vec<char>>::new());
        assert_eq!(make_grid(2, 0, '.'), vec![Vec::<char>::new(), Vec::new()]);
        assert_eq!(render(&[]), "");
    }

    #[test]
    fn center_cell_and_ragged_rows() {
        let g = make_grid(3, 3, '*');
        assert_eq!(count_neighbors(&g, 1, 1, '*'), 8);
        let ragged = vec![vec!['*'], vec!['*', '*', '*'], vec![]];
        assert_eq!(count_neighbors(&ragged, 1, 1, '*'), 3);
        assert_eq!(count_neighbors(&ragged, 1, 2, '*'), 1);
    }

    #[test]
    fn large_board_corners() {
        let g = make_grid(100, 100, '#');
        assert_eq!(count_neighbors(&g, 0, 0, '#'), 3);
        assert_eq!(count_neighbors(&g, 99, 99, '#'), 3);
        assert_eq!(count_neighbors(&g, 0, 50, '#'), 5);
        assert_eq!(render(&g).len(), 100 * 100 + 99);
    }
}
