# Problem 0274: Spiral Matrix

- **Difficulty:** Medium
- **Topics:** a `Bounds` struct that shrinks, a direction `enum` with `turn`, `match` on the direction, simulation
- **Source:** [LeetCode 54. Spiral Matrix](https://leetcode.com/problems/spiral-matrix/)

## Problem

Given an `m x n` matrix, return all of its elements in **spiral order**: start at the top-left
corner, go right along the top row, then down the right column, then left along the bottom row,
then up the left column, and continue inwards in the same clockwise way until every element has
been visited exactly once.

Implement `Solution::spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32>`.

For this level, keep the not-yet-visited rectangle in a `struct Bounds { top, bottom, left, right }`
and the current direction in an `enum Dir { Right, Down, Left, Up }`.

## Examples

```text
Input:  matrix = [[1, 2, 3],
                  [4, 5, 6],
                  [7, 8, 9]]
Output: [1, 2, 3, 6, 9, 8, 7, 4, 5]

Input:  matrix = [[1, 2, 3, 4],
                  [5, 6, 7, 8],
                  [9, 10, 11, 12]]
Output: [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
```

## Constraints

- `1 <= m, n <= 10`, `-100 <= matrix[i][j] <= 100`

## Hints

1. Each pass along one side visits that whole side of the remaining rectangle, then removes it:
   going right consumes the top row (`top += 1`), going down consumes the right column, and so on.
2. Stop as soon as the rectangle is empty (`top > bottom` or `left > right`).

## Starter code

Write your code in [`src/practice/p0274_spiral_matrix.rs`](../src/practice/p0274_spiral_matrix.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0274
```

```rust
pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
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
    fn example_square() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }

    #[test]
    fn example_wide() {
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }

    #[test]
    fn single_row_and_single_column() {
        assert_eq!(Solution::spiral_order(vec![vec![1, 2, 3]]), vec![1, 2, 3]);
        assert_eq!(
            Solution::spiral_order(vec![vec![1], vec![2], vec![3]]),
            vec![1, 2, 3]
        );
        assert_eq!(Solution::spiral_order(vec![vec![-7]]), vec![-7]);
    }

    #[test]
    fn tall_matrix() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
            vec![1, 2, 4, 6, 8, 7, 5, 3]
        );
    }

    #[test]
    fn largest_matrix_visits_everything_once() {
        let m: Vec<Vec<i32>> = (0..10)
            .map(|r| (0..10).map(|c| r * 10 + c).collect())
            .collect();
        let mut out = Solution::spiral_order(m);
        assert_eq!(&out[..11], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 19]);
        out.sort_unstable();
        assert_eq!(out, (0..100).collect::<Vec<_>>());
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** repeat "walk one full side in the current direction, shrink the bounds on that side,
turn clockwise" until the bounds are empty. Each element is pushed exactly once.

**Complexity:** `O(m * n)` time; `O(1)` extra space besides the output.

**Rust concepts:**
- The bounds use `i32` so that `bottom` and `right` may drop to `-1` without `usize` underflow;
  indexes are converted with `as usize` only when they are known to be valid.
- `match dir { Dir::Right => ..., Dir::Down => ..., ... }` keeps the four cases side by side, and
  `dir.turn()` makes the clockwise order explicit.
- `Bounds::is_empty(&self)` names the loop condition.

```rust
pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn turn(self) -> Dir {
        match self {
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Right,
        }
    }
}

/// The rectangle of cells not visited yet (inclusive bounds).
struct Bounds {
    top: i32,
    bottom: i32,
    left: i32,
    right: i32,
}

impl Bounds {
    fn is_empty(&self) -> bool {
        self.top > self.bottom || self.left > self.right
    }
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = matrix.len() as i32;
        let cols = matrix.first().map_or(0, Vec::len) as i32;
        let mut b = Bounds {
            top: 0,
            bottom: rows - 1,
            left: 0,
            right: cols - 1,
        };
        let at = |r: i32, c: i32| matrix[r as usize][c as usize];
        let mut out = Vec::with_capacity((rows * cols) as usize);
        let mut dir = Dir::Right;
        while !b.is_empty() {
            match dir {
                Dir::Right => {
                    out.extend((b.left..=b.right).map(|c| at(b.top, c)));
                    b.top += 1;
                }
                Dir::Down => {
                    out.extend((b.top..=b.bottom).map(|r| at(r, b.right)));
                    b.right -= 1;
                }
                Dir::Left => {
                    out.extend((b.left..=b.right).rev().map(|c| at(b.bottom, c)));
                    b.bottom -= 1;
                }
                Dir::Up => {
                    out.extend((b.top..=b.bottom).rev().map(|r| at(r, b.left)));
                    b.left += 1;
                }
            }
            dir = dir.turn();
        }
        out
    }
}
```

Full tested solution: [`src/solutions/p0274_spiral_matrix.rs`](../src/solutions/p0274_spiral_matrix.rs)

</details>
