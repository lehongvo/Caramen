# Problem 0276: Game of Life

- **Difficulty:** Medium
- **Topics:** a cell-state `enum` with four variants for in-place updates, `match (alive, neighbours)`, `&mut Vec<Vec<i32>>`
- **Source:** [LeetCode 289. Game of Life](https://leetcode.com/problems/game-of-life/)

## Problem

Conway's Game of Life is played on an `m x n` grid of cells, each **alive** (`1`) or **dead**
(`0`). Every cell has up to eight neighbours (horizontal, vertical and diagonal). The next
generation is computed from the current one, **all cells at the same time**:

1. a live cell with fewer than two live neighbours dies (under-population);
2. a live cell with two or three live neighbours stays alive;
3. a live cell with more than three live neighbours dies (over-population);
4. a dead cell with exactly three live neighbours becomes alive (reproduction).

Update `board` **in place** to the next generation.

Implement `Solution::game_of_life(board: &mut Vec<Vec<i32>>)`.

The challenge of updating in place: when you decide a cell's fate, its neighbours must still
show their *old* state. For this level, use an enum with four states, `Dead`, `Alive`, `Dying`
(alive now, dead next) and `Born` (dead now, alive next), stored in the grid as the codes
`0`, `1`, `2`, `3`.

## Examples

```text
Input:  board = [[0, 1, 0],
                 [0, 0, 1],
                 [1, 1, 1],
                 [0, 0, 0]]
Output:         [[0, 0, 0],
                 [1, 0, 1],
                 [0, 1, 1],
                 [0, 1, 0]]

Input:  board = [[1, 1],
                 [1, 0]]
Output:         [[1, 1],
                 [1, 1]]
```

## Constraints

- `1 <= m, n <= 25`; every cell is `0` or `1`.

## Hints

1. "Was alive in the old generation" is true for `Alive` and `Dying`: `matches!(state, Alive | Dying)`.
2. The four rules become one `match (was_alive, live_neighbours)` returning the new state.
3. A second pass turns `Dying` into `0` and `Born` into `1`.

## Starter code

Write your code in [`src/practice/p0276_game_of_life.rs`](../src/practice/p0276_game_of_life.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0276
```

```rust
pub struct Solution;

impl Solution {
    // LeetCode's signature uses `&mut Vec<Vec<i32>>`; a slice would be more idiomatic.
    #[allow(clippy::ptr_arg)]
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        todo!("Write your solution here")
    }
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
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
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** first pass: for each cell, count neighbours that were alive in the old generation
(`Alive` or `Dying`) and mark the cell `Dying` or `Born` if it changes. Second pass: settle the
transitional states into plain `0`/`1`.

**Complexity:** `O(m * n)` time, `O(1)` extra space.

**Rust concepts:**
- `CellState::from_code` / `code` convert between the enum and the `i32` grid; the logic only
  ever sees enum values.
- `match (was_alive, n)` with arms like `(true, 2 | 3)` and `(false, 3)` is a direct
  transcription of the four rules.
- LeetCode's signature takes `&mut Vec<Vec<i32>>`; clippy prefers `&mut [Vec<i32>]`
  (`ptr_arg`), so the lint is allowed on that one function.

```rust
pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    Dead,
    Alive,
    /// Alive in the current generation, dead in the next one.
    Dying,
    /// Dead in the current generation, alive in the next one.
    Born,
}

impl CellState {
    fn from_code(code: i32) -> CellState {
        match code {
            1 => CellState::Alive,
            2 => CellState::Dying,
            3 => CellState::Born,
            _ => CellState::Dead,
        }
    }

    fn code(self) -> i32 {
        match self {
            CellState::Dead => 0,
            CellState::Alive => 1,
            CellState::Dying => 2,
            CellState::Born => 3,
        }
    }

    fn was_alive(self) -> bool {
        matches!(self, CellState::Alive | CellState::Dying)
    }

    fn settled(self) -> CellState {
        match self {
            CellState::Dying => CellState::Dead,
            CellState::Born => CellState::Alive,
            other => other,
        }
    }
}

fn live_neighbours(board: &[Vec<i32>], r: usize, c: usize) -> usize {
    let mut count = 0;
    for dr in -1..=1isize {
        for dc in -1..=1isize {
            if (dr, dc) == (0, 0) {
                continue;
            }
            let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc)) else {
                continue;
            };
            if let Some(&code) = board.get(nr).and_then(|row| row.get(nc)) {
                if CellState::from_code(code).was_alive() {
                    count += 1;
                }
            }
        }
    }
    count
}

impl Solution {
    // LeetCode's signature uses `&mut Vec<Vec<i32>>`; a slice would be more idiomatic.
    #[allow(clippy::ptr_arg)]
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        for r in 0..board.len() {
            for c in 0..board[r].len() {
                let state = CellState::from_code(board[r][c]);
                let next = match (state.was_alive(), live_neighbours(board, r, c)) {
                    (true, 2 | 3) => CellState::Alive,
                    (true, _) => CellState::Dying,
                    (false, 3) => CellState::Born,
                    (false, _) => CellState::Dead,
                };
                board[r][c] = next.code();
            }
        }
        for cell in board.iter_mut().flatten() {
            *cell = CellState::from_code(*cell).settled().code();
        }
    }
}
```

Full tested solution: [`src/solutions/p0276_game_of_life.rs`](../src/solutions/p0276_game_of_life.rs)

</details>
