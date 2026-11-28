# Problem 0273: Available Captures for Rook

- **Difficulty:** Easy
- **Topics:** a `Piece` enum parsed from `char`, walking a grid in four directions, `match` deciding when to stop, `find_map`
- **Source:** [LeetCode 999. Available Captures for Rook](https://leetcode.com/problems/available-captures-for-rook/)

## Problem

An `8 x 8` chessboard contains exactly one white rook `'R'`, some white bishops `'B'`, some
black pawns `'p'`, and empty squares `'.'`. The rook moves any number of squares horizontally
or vertically, but cannot jump over pieces. It **attacks** a pawn if it can reach the pawn's
square in one move (a bishop, or another pawn, in the way blocks it).

Return the number of pawns the rook attacks (at most one per direction, so at most 4).

Implement `Solution::num_rook_captures(board: Vec<Vec<char>>) -> i32`.

## Examples

```text
Input:  board = [". . . . . . . .",
                 ". . . p . . . .",
                 ". . . R . . . p",
                 ". . . . . . . .",
                 ". . . . . . . .",
                 ". . . p . . . .",
                 ". . . . . . . .",
                 ". . . . . . . ."]     (spaces added for readability)
Output: 3        (up, right and down)

Input:  board = [". . . . . . . .",
                 ". p p p p p . .",
                 ". p p B p p . .",
                 ". p B R B p . .",
                 ". p p B p p . .",
                 ". p p p p p . .",
                 ". . . . . . . .",
                 ". . . . . . . ."]
Output: 0        (bishops block all four directions)
```

## Constraints

- `board.len() == 8`, `board[i].len() == 8`, exactly one `'R'`.

## Hints

1. Find the rook first: `find_map` over the rows with `position`.
2. Walk each of the four directions until the edge or a piece; a `match` on the piece says
   whether it counts (`Pawn`), blocks (`Bishop`), or lets you continue (`Empty`).

## Starter code

Write your code in [`src/practice/p0273_available_captures_for_rook.rs`](../src/practice/p0273_available_captures_for_rook.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0273
```

```rust
pub struct Solution;

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
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

    fn board(rows: [&str; 8]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    #[test]
    fn example_three_captures() {
        let b = board([
            "........", "...p....", "...R...p", "........", "........", "...p....", "........",
            "........",
        ]);
        assert_eq!(Solution::num_rook_captures(b), 3);
    }

    #[test]
    fn example_blocked_by_bishops() {
        let b = board([
            "........", ".ppppp..", ".ppBpp..", ".pBRBp..", ".ppBpp..", ".ppppp..", "........",
            "........",
        ]);
        assert_eq!(Solution::num_rook_captures(b), 0);
    }

    #[test]
    fn pawn_blocks_pawn_behind_it() {
        let b = board([
            "........", "...p....", "...p....", "pp.R.B.p", "........", "...B....", "...p....",
            "........",
        ]);
        // Up: the nearer pawn counts. Left: pawn at column 1 counts. Right: bishop blocks. Down: bishop blocks.
        assert_eq!(Solution::num_rook_captures(b), 2);
    }

    #[test]
    fn rook_in_the_corner() {
        let b = board([
            "R......p", "........", "........", "........", "........", "........", "........",
            "p.......",
        ]);
        assert_eq!(Solution::num_rook_captures(b), 2);
    }

    #[test]
    fn four_captures() {
        let b = board([
            "...p....", "........", "........", "p..R...p", "........", "........", "........",
            "...p....",
        ]);
        assert_eq!(Solution::num_rook_captures(b), 4);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** locate the rook, then for each direction step square by square. The first non-empty
square decides that direction: a pawn adds one capture, anything else adds none.

**Complexity:** `O(1)`: the board is always `8 x 8`.

**Rust concepts:**
- `Piece::from_char` turns the board characters into an enum once, so the walking code matches
  on meaningful names instead of raw `char`s.
- `checked_add_signed` (stable since Rust 1.66) moves a `usize` index by a signed delta and
  returns `None` below zero; `board.get(r)?.get(c)` handles the other edge.
- A `loop` with `match` on the piece ends with `break` values: `break 1` for a pawn, `break 0`
  for a blocker or the edge.

```rust
pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Piece {
    Rook,
    Bishop,
    Pawn,
    Empty,
}

impl Piece {
    fn from_char(c: char) -> Piece {
        match c {
            'R' => Piece::Rook,
            'B' => Piece::Bishop,
            'p' => Piece::Pawn,
            _ => Piece::Empty,
        }
    }
}

fn piece_at(board: &[Vec<char>], r: usize, c: usize) -> Option<Piece> {
    board.get(r)?.get(c).map(|&ch| Piece::from_char(ch))
}

/// 1 if a pawn is the first piece seen from `(r, c)` in direction `(dr, dc)`, else 0.
fn capture_in(
    board: &[Vec<char>],
    (mut r, mut c): (usize, usize),
    (dr, dc): (isize, isize),
) -> i32 {
    loop {
        let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc)) else {
            break 0;
        };
        match piece_at(board, nr, nc) {
            Some(Piece::Empty) => (r, c) = (nr, nc),
            Some(Piece::Pawn) => break 1,
            Some(Piece::Bishop | Piece::Rook) | None => break 0,
        }
    }
}

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let rook = board.iter().enumerate().find_map(|(r, row)| {
            row.iter()
                .position(|&ch| Piece::from_char(ch) == Piece::Rook)
                .map(|c| (r, c))
        });
        let Some(start) = rook else {
            return 0;
        };
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|dir| capture_in(&board, start, dir))
            .sum()
    }
}
```

Full tested solution: [`src/solutions/p0273_available_captures_for_rook.rs`](../src/solutions/p0273_available_captures_for_rook.rs)

</details>
