# Problem 0271: Find Winner on a Tic Tac Toe Game

- **Difficulty:** Easy
- **Topics:** a `Cell` enum grid, a `GameResult` enum, `Option<Player>` cells, checking lines with a `const` table
- **Source:** [LeetCode 1275. Find Winner on a Tic Tac Toe Game](https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game/)

## Problem

Two players, **A** and **B**, play tic-tac-toe on a `3 x 3` grid. A always moves first; they
alternate, each move placing the player's mark on an empty cell. The game ends as soon as one
player has three marks in a row, column or diagonal, or when all nine cells are filled.

`moves[i] = [row, col]` is the `i`-th move (so even `i` are A's moves and odd `i` are B's).
All moves are valid and the game stops at the end of `moves`. Return:

- `"A"` or `"B"` if that player has won;
- `"Draw"` if the grid is full and nobody won;
- `"Pending"` otherwise (the game is not finished).

Implement `Solution::tictactoe(moves: Vec<Vec<i32>>) -> String`.

For this level, model the board as `[[Option<Player>; 3]; 3]` with `enum Player { A, B }`, and
the answer as `enum GameResult { Win(Player), Draw, Pending }`.

## Examples

```text
Input:  moves = [[0,0],[2,0],[1,1],[2,1],[2,2]]
Output: "A"        (A holds the main diagonal)

Input:  moves = [[0,0],[1,1],[0,1],[0,2],[1,0],[2,0]]
Output: "B"        (B holds the anti-diagonal)

Input:  moves = [[0,0],[1,1],[2,0],[1,0],[1,2],[2,1],[0,1],[0,2],[2,2]]
Output: "Draw"

Input:  moves = [[0,0],[1,1]]
Output: "Pending"
```

## Constraints

- `1 <= moves.len() <= 9`, `moves[i].len() == 2`, `0 <= row, col <= 2`
- All moves are on distinct cells and follow the rules.

## Hints

1. A `const LINES: [[(usize, usize); 3]; 8]` table lists the 3 rows, 3 columns and 2 diagonals.
2. A line is won by `p` when its three cells are all `Some(p)`.

## Starter code

Write your code in [`src/practice/p0271_find_winner_on_a_tic_tac_toe_game.rs`](../src/practice/p0271_find_winner_on_a_tic_tac_toe_game.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0271
```

```rust
pub struct Solution;

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
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

    fn play(moves: &[[i32; 2]]) -> String {
        Solution::tictactoe(moves.iter().map(|m| m.to_vec()).collect())
    }

    #[test]
    fn example_a_wins_diagonal() {
        assert_eq!(play(&[[0, 0], [2, 0], [1, 1], [2, 1], [2, 2]]), "A");
    }

    #[test]
    fn example_b_wins_anti_diagonal() {
        assert_eq!(play(&[[0, 0], [1, 1], [0, 1], [0, 2], [1, 0], [2, 0]]), "B");
    }

    #[test]
    fn example_draw() {
        assert_eq!(
            play(&[
                [0, 0],
                [1, 1],
                [2, 0],
                [1, 0],
                [1, 2],
                [2, 1],
                [0, 1],
                [0, 2],
                [2, 2]
            ]),
            "Draw"
        );
    }

    #[test]
    fn example_pending() {
        assert_eq!(play(&[[0, 0], [1, 1]]), "Pending");
        assert_eq!(play(&[[1, 1]]), "Pending");
    }

    #[test]
    fn rows_and_columns() {
        assert_eq!(play(&[[0, 0], [1, 0], [0, 1], [1, 1], [0, 2]]), "A");
        assert_eq!(play(&[[0, 0], [0, 2], [1, 0], [1, 2], [2, 1], [2, 2]]), "B");
    }

    #[test]
    fn full_boards() {
        // A B A / A B B / B A A: nobody has a line.
        assert_eq!(
            play(&[
                [0, 0],
                [0, 1],
                [0, 2],
                [1, 1],
                [1, 0],
                [1, 2],
                [2, 1],
                [2, 0],
                [2, 2]
            ]),
            "Draw"
        );
        // A completes the main diagonal with the ninth move: a win, not a draw.
        assert_eq!(
            play(&[
                [0, 0],
                [0, 1],
                [1, 1],
                [0, 2],
                [1, 2],
                [1, 0],
                [2, 0],
                [2, 1],
                [2, 2]
            ]),
            "A"
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** replay the moves onto the board, then check the eight lines. A winner is returned as
`Win(player)`; otherwise the number of moves decides between `Draw` (9) and `Pending`.

**Complexity:** `O(1)` time and space (at most 9 moves and 8 lines).

**Rust concepts:**
- `Option<Player>` is a natural "maybe empty" cell; no extra `Empty` variant is needed.
- Matching a line's three cells as a tuple, `(Some(a), Some(b), Some(c)) if a == b && b == c`,
  finds a winner and binds it.
- `GameResult::label(&self) -> String` keeps the output strings away from the game logic.
- `match m.as_slice() { &[r, c] => ..., _ => continue }` reads one move safely.

```rust
pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Player {
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameResult {
    Win(Player),
    Draw,
    Pending,
}

impl GameResult {
    fn label(&self) -> String {
        match self {
            GameResult::Win(Player::A) => "A".to_string(),
            GameResult::Win(Player::B) => "B".to_string(),
            GameResult::Draw => "Draw".to_string(),
            GameResult::Pending => "Pending".to_string(),
        }
    }
}

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

fn winner(board: &[[Option<Player>; 3]; 3]) -> Option<Player> {
    LINES.iter().find_map(|&[(r1, c1), (r2, c2), (r3, c3)]| {
        match (board[r1][c1], board[r2][c2], board[r3][c3]) {
            (Some(a), Some(b), Some(c)) if a == b && b == c => Some(a),
            _ => None,
        }
    })
}

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut board = [[None; 3]; 3];
        for (i, m) in moves.iter().enumerate() {
            let &[r, c] = m.as_slice() else {
                continue;
            };
            let player = if i % 2 == 0 { Player::A } else { Player::B };
            // Coordinates are in 0..=2 by the constraints.
            board[r as usize][c as usize] = Some(player);
        }
        let result = match winner(&board) {
            Some(p) => GameResult::Win(p),
            None if moves.len() == 9 => GameResult::Draw,
            None => GameResult::Pending,
        };
        result.label()
    }
}
```

Full tested solution: [`src/solutions/p0271_find_winner_on_a_tic_tac_toe_game.rs`](../src/solutions/p0271_find_winner_on_a_tic_tac_toe_game.rs)

</details>
