//! # Problem 0918: Game of Life on a `Grid<W, H>`
//!
//! - **Difficulty:** Medium
//! - **Topics:** two const generic parameters, `[[bool; W]; H]`, arrays as parameters (`[&str; H]`), `Copy` types, `Display`
//! - **Source:** Original · [Reference: Const generics](https://doc.rust-lang.org/reference/items/generics.html#const-generics) · [std: `array::from_fn`](https://doc.rust-lang.org/std/array/fn.from_fn.html)
//!
//! ## Problem
//!
//! Conway's Game of Life on a fixed-size board that **wraps around** (a torus: the column left of `0`
//! is `W - 1`, the row above `0` is `H - 1`). The size is part of the type:
//!
//! ```text
//! pub struct Grid<const W: usize, const H: usize> { cells: [[bool; W]; H] }   // cells[y][x]
//! ```
//!
//! - `empty()`: all cells dead.
//! - `from_rows(rows: [&str; H]) -> Result<Self, GridError>`: one string per row, `'#'` alive and
//!   `'.'` dead. Passing the wrong **number** of rows is a compile error (the array length is `H`).
//!   A row whose length is not `W` gives `GridError::WrongWidth { row, len }`; any other character
//!   gives `GridError::BadCell { row, col, ch }`. Report the first problem in row-major order.
//! - `get(&self, x, y) -> bool` and `set(&mut self, x, y, alive)` (`x` is the column). Out-of-range
//!   coordinates are a bug: indexing panics.
//! - `live_neighbors(&self, x, y) -> usize`: the number of live cells among the 8 neighbours, with
//!   wrap-around.
//! - `step(&self) -> Self`: the next generation. A live cell survives with 2 or 3 live neighbours, a
//!   dead cell becomes alive with exactly 3, every other cell is dead.
//! - `steps(&self, n: usize) -> Self` and `population(&self) -> usize`.
//! - `Display`: the rows in the `from_rows` format, separated by `'\n'` (no trailing newline).
//!
//! ## Examples
//!
//! ```text
//! let blinker = Grid::<5, 5>::from_rows([".....", ".....", ".###.", ".....", "....."])?;
//! blinker.step().to_string()
//!   => ".....\n..#..\n..#..\n..#..\n....."
//! blinker.steps(2) == blinker      => true
//! Grid::<3, 2>::from_rows(["###", "##"])   => Err(WrongWidth { row: 1, len: 2 })
//! ```
//!
//! ## Constraints
//!
//! - `3 <= W, H <= 64` in the tests.
//!
//! ## Hints
//!
//! 1. With wrap-around, the neighbour column is `(x + W + dx - 1) % W` for `dx` in `0..3`
//!    (and the same for rows); skip `dx == 1 && dy == 1`, the cell itself.
//! 2. `Grid` is `Copy`, so `step` can build the next grid from `Self::empty()` while reading `self`.
//! 3. `std::array::from_fn` is handy wherever you need an array computed from its index.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `step` reads the current grid and writes a fresh one, so updates never affect the
//! neighbour counts of the same generation. Neighbours are counted with modular arithmetic; adding
//! `W` (or `H`) before subtracting avoids underflow on `usize`.
//!
//! **Complexity:** `step` is `O(W * H)` time and uses one extra grid.
//!
//! **Rust concepts:**
//! - `Grid<5, 5>` and `Grid<8, 8>` are different types: comparing or mixing them is a type error.
//! - An array parameter `[&str; H]` checks the number of rows at compile time.
//! - `[[bool; W]; H]` is `Copy`, so `#[derive(Clone, Copy)]` works and grids are copied by value.
//! - `W` and `H` can be used as ordinary `usize` values inside the impl.
// @solution-end

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GridError {
    WrongWidth { row: usize, len: usize },
    BadCell { row: usize, col: usize, ch: char },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grid<const W: usize, const H: usize> {
    cells: [[bool; W]; H],
}

impl<const W: usize, const H: usize> Grid<W, H> {
    pub fn empty() -> Self {
        // @solution-start
        Grid {
            cells: [[false; W]; H],
        }
        // @solution-end
    }

    pub fn from_rows(rows: [&str; H]) -> Result<Self, GridError> {
        // @solution-start
        let mut grid = Self::empty();
        for (y, row) in rows.iter().enumerate() {
            let len = row.chars().count();
            if len != W {
                return Err(GridError::WrongWidth { row: y, len });
            }
            for (x, ch) in row.chars().enumerate() {
                grid.cells[y][x] = match ch {
                    '#' => true,
                    '.' => false,
                    ch => return Err(GridError::BadCell { row: y, col: x, ch }),
                };
            }
        }
        Ok(grid)
        // @solution-end
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        // @solution-start
        self.cells[y][x]
        // @solution-end
    }

    pub fn set(&mut self, x: usize, y: usize, alive: bool) {
        // @solution-start
        self.cells[y][x] = alive;
        // @solution-end
    }

    pub fn live_neighbors(&self, x: usize, y: usize) -> usize {
        // @solution-start
        let mut count = 0;
        for dy in 0..3 {
            for dx in 0..3 {
                if dx == 1 && dy == 1 {
                    continue;
                }
                let nx = (x + W + dx - 1) % W;
                let ny = (y + H + dy - 1) % H;
                if self.cells[ny][nx] {
                    count += 1;
                }
            }
        }
        count
        // @solution-end
    }

    pub fn step(&self) -> Self {
        // @solution-start
        let mut next = Self::empty();
        for y in 0..H {
            for x in 0..W {
                let n = self.live_neighbors(x, y);
                next.cells[y][x] = matches!((self.cells[y][x], n), (true, 2) | (_, 3));
            }
        }
        next
        // @solution-end
    }

    pub fn steps(&self, n: usize) -> Self {
        // @solution-start
        (0..n).fold(*self, |grid, _| grid.step())
        // @solution-end
    }

    pub fn population(&self) -> usize {
        // @solution-start
        self.cells.iter().flatten().filter(|&&alive| alive).count()
        // @solution-end
    }
}

impl<const W: usize, const H: usize> fmt::Display for Grid<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // @solution-start
        for (y, row) in self.cells.iter().enumerate() {
            if y > 0 {
                writeln!(f)?;
            }
            for &alive in row {
                f.write_str(if alive { "#" } else { "." })?;
            }
        }
        Ok(())
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blinker_oscillates() {
        let blinker =
            Grid::<5, 5>::from_rows([".....", ".....", ".###.", ".....", "....."]).unwrap();
        let next = blinker.step();
        assert_eq!(next.to_string(), ".....\n..#..\n..#..\n..#..\n.....");
        assert_eq!(blinker.steps(2), blinker);
        assert_eq!(next.population(), 3);
        assert_eq!(blinker.live_neighbors(2, 2), 2);
        assert_eq!(blinker.live_neighbors(2, 1), 3);
    }

    #[test]
    fn parse_errors() {
        assert_eq!(
            Grid::<3, 2>::from_rows(["###", "##"]),
            Err(GridError::WrongWidth { row: 1, len: 2 })
        );
        assert_eq!(
            Grid::<3, 2>::from_rows(["#.#", ".x."]),
            Err(GridError::BadCell {
                row: 1,
                col: 1,
                ch: 'x'
            })
        );
        assert_eq!(
            Grid::<3, 3>::from_rows(["..", "#x#", "..."]),
            Err(GridError::WrongWidth { row: 0, len: 2 })
        );
    }

    #[test]
    fn neighbours_wrap_around() {
        let mut grid = Grid::<4, 3>::empty();
        grid.set(0, 0, true);
        grid.set(3, 2, true);
        assert!(grid.get(3, 2));
        // (3, 2) is diagonally adjacent to (0, 0) on the torus.
        assert_eq!(grid.live_neighbors(0, 0), 1);
        assert_eq!(grid.live_neighbors(3, 0), 2);
        assert_eq!(grid.population(), 2);
        assert_eq!(grid.to_string(), "#...\n....\n...#");
    }

    #[test]
    fn block_is_still_life_and_lonely_cells_die() {
        let block = Grid::<4, 4>::from_rows(["....", ".##.", ".##.", "...."]).unwrap();
        assert_eq!(block.step(), block);
        let lonely = Grid::<3, 3>::from_rows(["#..", "...", "..."]).unwrap();
        assert_eq!(lonely.step().population(), 0);
        assert_eq!(Grid::<3, 3>::empty().steps(5), Grid::<3, 3>::empty());
    }

    #[test]
    fn glider_returns_after_twenty_generations_on_a_5x5_torus() {
        let glider =
            Grid::<5, 5>::from_rows([".#...", "..#..", "###..", ".....", "....."]).unwrap();
        let mut grid = glider;
        for generation in 1..20 {
            grid = grid.step();
            assert_eq!(grid.population(), 5, "generation {generation}");
            assert_ne!(grid, glider, "generation {generation}");
        }
        assert_eq!(grid.step(), glider);
        // A glider moves one cell diagonally every 4 generations.
        let moved = glider.steps(4);
        assert_eq!(moved.to_string(), ".....\n..#..\n...#.\n.###.\n.....");
    }
}
