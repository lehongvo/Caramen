# Problem 0821: Minimum Path Sum

- **Difficulty:** Medium
- **Topics:** grid DP done in place, taking ownership of the input with `let mut grid = grid`
- **Source:** [LeetCode 64. Minimum Path Sum](https://leetcode.com/problems/minimum-path-sum/)

## Problem

`grid` is an `m x n` grid of non-negative numbers. Walk from the top-left cell to the
bottom-right cell, moving only **right** or **down**. The cost of a path is the sum of all the
cells it visits (including both ends). Return the minimum cost.

Implement `Solution::min_path_sum(grid: Vec<Vec<i32>>) -> i32`.

## Examples

```text
Input:  grid = [[1, 3, 1],
                [1, 5, 1],
                [4, 2, 1]]
Output: 7
Explanation: 1 -> 3 -> 1 -> 1 -> 1.

Input:  grid = [[1, 2, 3],
                [4, 5, 6]]
Output: 12
```

## Constraints

- `1 <= m, n <= 200`
- `0 <= grid[i][j] <= 200`

## Hints

1. The cheapest way to reach a cell comes from the cheaper of its top and left neighbours.
2. The function owns `grid`, so you may overwrite each cell with its best cost: no extra memory.

## Starter code

Write your code in [`src/practice/p0821_minimum_path_sum.rs`](../src/practice/p0821_minimum_path_sum.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0821
```

```rust
pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
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
    fn example_three_by_three() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
        assert_eq!(Solution::min_path_sum(grid), 7);
    }

    #[test]
    fn example_two_by_three() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(Solution::min_path_sum(grid), 12);
    }

    #[test]
    fn single_cell_row_and_column() {
        assert_eq!(Solution::min_path_sum(vec![vec![5]]), 5);
        assert_eq!(Solution::min_path_sum(vec![vec![1, 2, 3]]), 6);
        assert_eq!(Solution::min_path_sum(vec![vec![1], vec![2], vec![3]]), 6);
    }

    #[test]
    fn detour_is_cheaper() {
        let grid = vec![vec![1, 9, 1, 1], vec![1, 9, 1, 9], vec![1, 1, 1, 1]];
        assert_eq!(Solution::min_path_sum(grid), 6);
    }

    #[test]
    fn largest_uniform_grid() {
        // Any path visits 200 + 200 - 1 = 399 cells.
        assert_eq!(Solution::min_path_sum(vec![vec![1; 200]; 200]), 399);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** replace every cell by the cheapest cost to reach it:
`grid[i][j] += min(grid[i - 1][j], grid[i][j - 1])`, using only the neighbours that exist. Walk
rows top to bottom and columns left to right so both neighbours are already final.

**Complexity:** `O(m * n)` time, `O(1)` extra space.

**Rust concepts:**
- The parameter is an owned `Vec<Vec<i32>>`; `let mut grid = grid;` rebinds it as mutable
  without copying (the signature stays exactly LeetCode's).
- `match (i, j)` on a tuple of indices separates the corner, first row, first column and inner
  cells clearly.

```rust
pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let (m, n) = (grid.len(), grid[0].len());
        for i in 0..m {
            for j in 0..n {
                grid[i][j] += match (i, j) {
                    (0, 0) => 0,
                    (0, _) => grid[0][j - 1],
                    (_, 0) => grid[i - 1][0],
                    _ => grid[i - 1][j].min(grid[i][j - 1]),
                };
            }
        }
        grid[m - 1][n - 1]
    }
}
```

Full tested solution: [`src/solutions/p0821_minimum_path_sum.rs`](../src/solutions/p0821_minimum_path_sum.rs)

</details>
