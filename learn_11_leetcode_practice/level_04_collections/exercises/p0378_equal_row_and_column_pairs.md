# Problem 0378: Equal Row and Column Pairs

- **Difficulty:** Medium
- **Topics:** `HashMap<&[i32], i32>` keyed by borrowed slices, building columns, `get` with a slice key
- **Source:** [LeetCode 2352. Equal Row and Column Pairs](https://leetcode.com/problems/equal-row-and-column-pairs/)

## Problem

Given an `n x n` integer matrix `grid`, return the number of pairs `(r, c)` such that row `r`
and column `c` are **equal**: they contain the same elements in the same order (row read left to
right, column read top to bottom).

Implement `Solution::equal_pairs(grid: Vec<Vec<i32>>) -> i32`.

## Examples

```text
Input:  grid = [[3, 2, 1],
                [1, 7, 6],
                [2, 7, 7]]
Output: 1      (row 2 = column 1 = [2, 7, 7])

Input:  grid = [[3, 1, 2, 2],
                [1, 4, 4, 5],
                [2, 4, 2, 2],
                [2, 4, 2, 2]]
Output: 3      (row 0 = column 0; rows 2 and 3 both equal column 2)
```

## Constraints

- `1 <= n <= 200`
- `1 <= grid[i][j] <= 10^5`

## Hints

1. Comparing every row with every column element by element is `O(n^3)`. Hashing is faster.
2. Count the rows in a `HashMap` (several rows can be equal). Then build each column and look it
   up.

## Starter code

Write your code in [`src/practice/p0378_equal_row_and_column_pairs.rs`](../src/practice/p0378_equal_row_and_column_pairs.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0378
```

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
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
    fn example_one() {
        assert_eq!(
            Solution::equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]]),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::equal_pairs(vec![
                vec![3, 1, 2, 2],
                vec![1, 4, 4, 5],
                vec![2, 4, 2, 2],
                vec![2, 4, 2, 2]
            ]),
            3
        );
    }

    #[test]
    fn single_cell_and_symmetric() {
        assert_eq!(Solution::equal_pairs(vec![vec![5]]), 1);
        // A symmetric matrix: row i equals column i for every i.
        assert_eq!(Solution::equal_pairs(vec![vec![1, 2], vec![2, 3]]), 2);
    }

    #[test]
    fn all_equal_large_grid() {
        // Every row equals every column: 200 * 200 pairs.
        let grid = vec![vec![7; 200]; 200];
        assert_eq!(Solution::equal_pairs(grid), 40_000);
        let distinct: Vec<Vec<i32>> = (0..200)
            .map(|r| (0..200).map(|c| r * 200 + c + 1).collect())
            .collect();
        assert_eq!(Solution::equal_pairs(distinct), 0);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** count how many times each row occurs, using the rows themselves (as borrowed slices)
as keys. For each column, build it as a `Vec<i32>` and add the number of rows equal to it.

**Complexity:** `O(n^2)` average time (hashing a row costs `O(n)`), `O(n^2)` space.

**Rust concepts:**
- `HashMap<&[i32], i32>` borrows the rows from `grid` instead of cloning them; the map cannot
  outlive `grid`, which the borrow checker enforces.
- `counts.get(column.as_slice())` looks up with a `&[i32]`; slices hash and compare by content.
- `copied().unwrap_or(0)` treats "no equal row" as zero.

```rust
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut counts: HashMap<&[i32], i32> = HashMap::new();
        for row in &grid {
            *counts.entry(row.as_slice()).or_insert(0) += 1;
        }
        let n = grid.len();
        (0..n)
            .map(|c| {
                let column: Vec<i32> = grid.iter().map(|row| row[c]).collect();
                counts.get(column.as_slice()).copied().unwrap_or(0)
            })
            .sum()
    }
}
```

Full tested solution: [`src/solutions/p0378_equal_row_and_column_pairs.rs`](../src/solutions/p0378_equal_row_and_column_pairs.rs)

</details>
