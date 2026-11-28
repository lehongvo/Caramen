# Problem 0277: Valid Sudoku

- **Difficulty:** Medium
- **Topics:** `match` on `char` with a range pattern and a binding, a `Seen` struct of bit masks, box index arithmetic
- **Source:** [LeetCode 36. Valid Sudoku](https://leetcode.com/problems/valid-sudoku/)

## Problem

A `9 x 9` Sudoku board is partially filled: each cell is a digit `'1'`..`'9'` or `'.'` (empty).
Determine whether the **filled** cells are consistent with the rules:

1. each row contains each digit at most once;
2. each column contains each digit at most once;
3. each of the nine `3 x 3` boxes contains each digit at most once.

The board does not need to be solvable; only the filled cells are checked.

Implement `Solution::is_valid_sudoku(board: Vec<Vec<char>>) -> bool`.

## Examples

```text
Input:  the classic example board
        5 3 . | . 7 . | . . .
        6 . . | 1 9 5 | . . .
        . 9 8 | . . . | . 6 .
        ------+-------+------
        8 . . | . 6 . | . . 3
        4 . . | 8 . 3 | . . 1
        7 . . | . 2 . | . . 6
        ------+-------+------
        . 6 . | . . . | 2 8 .
        . . . | 4 1 9 | . . 5
        . . . | . 8 . | . 7 9
Output: true

Input:  the same board with the top-left 5 replaced by 8
Output: false     (two 8s in the first column and in the top-left box)
```

## Constraints

- `board.len() == 9`, `board[i].len() == 9`, each cell is `'1'`..`'9'` or `'.'`.

## Hints

1. The box of cell `(r, c)` has index `(r / 3) * 3 + c / 3`.
2. `match ch { d @ '1'..='9' => ..., _ => continue }` filters and binds the digit in one arm.
3. A `u16` bit mask per row, column and box is enough to remember which digits were seen.

## Starter code

Write your code in [`src/practice/p0277_valid_sudoku.rs`](../src/practice/p0277_valid_sudoku.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0277
```

```rust
pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
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

    fn board(rows: [&str; 9]) -> Vec<Vec<char>> {
        rows.iter().map(|r| r.chars().collect()).collect()
    }

    fn classic() -> [&'static str; 9] {
        [
            "53..7....",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]
    }

    #[test]
    fn example_valid() {
        assert!(Solution::is_valid_sudoku(board(classic())));
    }

    #[test]
    fn example_duplicate_in_column_and_box() {
        let mut rows = classic();
        rows[0] = "83..7....";
        assert!(!Solution::is_valid_sudoku(board(rows)));
    }

    #[test]
    fn duplicate_in_row_only() {
        let mut rows = classic();
        rows[0] = "53..7...5";
        assert!(!Solution::is_valid_sudoku(board(rows)));
    }

    #[test]
    fn duplicate_in_box_only() {
        let mut rows = [
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
        ];
        rows[3] = "...1.....";
        rows[5] = ".....1...";
        assert!(!Solution::is_valid_sudoku(board(rows)));
        rows[5] = "......1..";
        assert!(Solution::is_valid_sudoku(board(rows)));
    }

    #[test]
    fn empty_and_full_solution() {
        assert!(Solution::is_valid_sudoku(vec![vec!['.'; 9]; 9]));
        let solved = [
            "534678912",
            "672195348",
            "198342567",
            "859761423",
            "426853791",
            "713924856",
            "961537284",
            "287419635",
            "345286179",
        ];
        assert!(Solution::is_valid_sudoku(board(solved)));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** scan every cell once. For a digit `d`, compute its bit `1 << d` and check it against
the masks of its row, column and box; if any already has the bit, the board is invalid.
Otherwise set the bit in all three.

**Complexity:** `O(81)` = `O(1)` time and space.

**Rust concepts:**
- `d @ '1'..='9'` binds the character while checking the range; `d as u32 - '0' as u32` turns
  it into a number.
- The `Seen` struct groups three arrays (`rows`, `cols`, `boxes`) and a method
  `insert(&mut self, r, c, digit) -> bool` that returns `false` on a duplicate, so the main
  loop stays short.
- Bit masks are a compact set: `mask & bit != 0` tests membership, `mask |= bit` inserts.

```rust
pub struct Solution;

#[derive(Default)]
struct Seen {
    rows: [u16; 9],
    cols: [u16; 9],
    boxes: [u16; 9],
}

impl Seen {
    /// Records `digit` at `(r, c)`; `false` if it was already in that row, column or box.
    fn insert(&mut self, r: usize, c: usize, digit: u32) -> bool {
        let bit = 1u16 << digit;
        let b = (r / 3) * 3 + c / 3;
        if (self.rows[r] | self.cols[c] | self.boxes[b]) & bit != 0 {
            return false;
        }
        self.rows[r] |= bit;
        self.cols[c] |= bit;
        self.boxes[b] |= bit;
        true
    }
}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut seen = Seen::default();
        for (r, row) in board.iter().enumerate().take(9) {
            for (c, &ch) in row.iter().enumerate().take(9) {
                let digit = match ch {
                    d @ '1'..='9' => d as u32 - '0' as u32,
                    _ => continue,
                };
                if !seen.insert(r, c, digit) {
                    return false;
                }
            }
        }
        true
    }
}
```

Full tested solution: [`src/solutions/p0277_valid_sudoku.rs`](../src/solutions/p0277_valid_sudoku.rs)

</details>
