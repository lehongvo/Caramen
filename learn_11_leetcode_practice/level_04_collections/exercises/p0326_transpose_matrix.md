# Problem 0326: Transpose Matrix

- **Difficulty:** Easy
- **Topics:** non-square `Vec<Vec<i32>>`, nested `map` + `collect`, row/column indices
- **Source:** [LeetCode 867. Transpose Matrix](https://leetcode.com/problems/transpose-matrix/)

## Problem

The **transpose** of a matrix flips it over its main diagonal: the element at row `r`, column
`c` moves to row `c`, column `r`. An `m x n` matrix becomes an `n x m` matrix.

Given a matrix as a vector of rows (all rows have the same length), return its transpose.

Implement `Solution::transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.

## Examples

```text
Input:  matrix = [[1, 2, 3],
                  [4, 5, 6],
                  [7, 8, 9]]
Output: [[1, 4, 7], [2, 5, 8], [3, 6, 9]]

Input:  matrix = [[1, 2, 3],
                  [4, 5, 6]]
Output: [[1, 4], [2, 5], [3, 6]]
```

## Constraints

- `1 <= m, n <= 1000` and `1 <= m * n <= 10^5`
- `-10^9 <= matrix[r][c] <= 10^9`

## Hints

1. For a non-square matrix the result has a different shape, so it cannot be done by swapping
   in place; build a new matrix.
2. Row `c` of the answer is column `c` of the input: `matrix.iter().map(|row| row[c])`.

## Starter code

Write your code in [`src/practice/p0326_transpose_matrix.rs`](../src/practice/p0326_transpose_matrix.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0326
```

```rust
pub struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
        );
    }

    #[test]
    fn example_rectangle() {
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            vec![vec![1, 4], vec![2, 5], vec![3, 6]]
        );
    }

    #[test]
    fn single_row_and_single_cell() {
        assert_eq!(
            Solution::transpose(vec![vec![1, 2, 3]]),
            vec![vec![1], vec![2], vec![3]]
        );
        assert_eq!(Solution::transpose(vec![vec![-7]]), vec![vec![-7]]);
    }

    #[test]
    fn transposing_twice_gives_back_the_input() {
        let matrix: Vec<Vec<i32>> = (0..100)
            .map(|r| (0..1000).map(|c| r * 1000 + c).collect())
            .collect();
        let t = Solution::transpose(matrix.clone());
        assert_eq!(t.len(), 1000);
        assert_eq!(t[999][99], 99_999);
        assert_eq!(Solution::transpose(t), matrix);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the new row `c` collects `row[c]` from every input row, top to bottom. Doing that for
every column index `c` in `0..n` builds the transpose.

**Complexity:** `O(m * n)` time and space.

**Rust concepts:**
- Nested iterators build nested collections: the inner `collect()` produces a `Vec<i32>`, the
  outer one a `Vec<Vec<i32>>` (both inferred from the return type).
- `matrix[0].len()` is safe because the constraints guarantee at least one row.
- An equivalent imperative version allocates `vec![vec![0; m]; n]` and assigns
  `t[c][r] = matrix[r][c]` in two loops.

```rust
pub struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // The constraints guarantee at least one row.
        let cols = matrix[0].len();
        (0..cols)
            .map(|c| matrix.iter().map(|row| row[c]).collect())
            .collect()
    }
}
```

Full tested solution: [`src/solutions/p0326_transpose_matrix.rs`](../src/solutions/p0326_transpose_matrix.rs)

</details>
