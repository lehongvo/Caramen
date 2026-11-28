# Problem 0226: Determine Color of a Chessboard Square

- **Difficulty:** Easy
- **Topics:** returning an `enum` from a helper, slice pattern with `let ... else`, byte arithmetic
- **Source:** [LeetCode 1812. Determine Color of a Chessboard Square](https://leetcode.com/problems/determine-color-of-a-chessboard-square/)

## Problem

A chessboard square is written as two characters: a file letter `'a'`..`'h'` (the column) and a
rank digit `'1'`..`'8'` (the row), for example `"c7"`. Square `"a1"` (bottom-left) is **black**,
and colors alternate along every row and column.

Given `coordinates`, return `true` if the square is **white** and `false` if it is black.

Implement `Solution::square_is_white(coordinates: String) -> bool`.

For this level, write a helper that returns an `enum SquareColor { Black, White }` and turn it
into the `bool` at the end.

## Examples

```text
Input:  coordinates = "a1"
Output: false     (black)

Input:  coordinates = "h3"
Output: true      (white)

Input:  coordinates = "c7"
Output: false
```

## Constraints

- `coordinates.len() == 2`
- `'a' <= coordinates[0] <= 'h'` and `'1' <= coordinates[1] <= '8'`

## Hints

1. Convert the file to `0..8` with `b - b'a'` and the rank with `b - b'1'`.
2. `"a1"` gives `0 + 0 = 0` and is black: the color depends only on whether the sum is even.

## Starter code

Write your code in [`src/practice/p0226_determine_color_of_a_chessboard_square.rs`](../src/practice/p0226_determine_color_of_a_chessboard_square.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0226
```

```rust
pub struct Solution;

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
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

    #[test]
    fn example_a1_is_black() {
        assert!(!Solution::square_is_white("a1".to_string()));
    }

    #[test]
    fn example_h3_is_white() {
        assert!(Solution::square_is_white("h3".to_string()));
    }

    #[test]
    fn example_c7_is_black() {
        assert!(!Solution::square_is_white("c7".to_string()));
    }

    #[test]
    fn corners() {
        assert!(Solution::square_is_white("h1".to_string()));
        assert!(Solution::square_is_white("a8".to_string()));
        assert!(!Solution::square_is_white("h8".to_string()));
    }

    #[test]
    fn board_has_32_white_squares() {
        let mut white = 0;
        for file in 'a'..='h' {
            for rank in '1'..='8' {
                if Solution::square_is_white(format!("{file}{rank}")) {
                    white += 1;
                }
            }
        }
        assert_eq!(white, 32);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** with zero-based file and rank, a square is black exactly when `file + rank` is even.
The helper returns `SquareColor`, and the solution compares it with `SquareColor::White`.

**Complexity:** `O(1)` time and space.

**Rust concepts:**
- `coordinates.as_bytes()` gives `&[u8]`; `b'a'` is a byte literal, so `bytes[0] - b'a'` is a
  `u8` between 0 and 7.
- A slice pattern `let [file, rank] = bytes else { ... };` checks the length and names both
  bytes at once.
- Returning an enum instead of a bare `bool` makes the helper self-describing: `SquareColor::White`
  cannot be confused with "is black".

```rust
pub struct Solution;

/// The two colors of a chessboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SquareColor {
    Black,
    White,
}

/// The color of a square such as `"c7"`, or `None` if the text is not two bytes.
fn color_of(coordinates: &str) -> Option<SquareColor> {
    let &[file, rank] = coordinates.as_bytes() else {
        return None;
    };
    let sum = (file - b'a') + (rank - b'1');
    if sum % 2 == 0 {
        Some(SquareColor::Black)
    } else {
        Some(SquareColor::White)
    }
}

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        color_of(&coordinates) == Some(SquareColor::White)
    }
}
```

Full tested solution: [`src/solutions/p0226_determine_color_of_a_chessboard_square.rs`](../src/solutions/p0226_determine_color_of_a_chessboard_square.rs)

</details>
