# Problem 0505: Shift 2D Grid

- **Difficulty:** Easy
- **Topics:** generic helper `shift_grid_generic<T: Clone>`, flattening, `rotate_right`, `chunks`
- **Source:** [LeetCode 1260. Shift 2D Grid](https://leetcode.com/problems/shift-2d-grid/)

## Problem

You are given an `m x n` grid of integers and a number `k`. One **shift** moves every element one
step forward in row-major order: the element at `(i, j)` moves to `(i, j + 1)`, the last element of
a row moves to the first column of the next row, and the very last element of the grid moves to
`(0, 0)`. Return the grid after `k` shifts.

Implement `Solution::shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>>`.

The shift does not care about the element type, so write the real work in a **generic** helper
`shift_grid_generic<T: Clone>(grid: &[Vec<T>], k: usize) -> Vec<Vec<T>>` and call it from the
LeetCode method. The helper returns an empty vector for an empty grid.

## Examples

```text
Input:  grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]], k = 1
Output: [[9, 1, 2], [3, 4, 5], [6, 7, 8]]

Input:  grid = [[3, 8], [1, 9], [19, 7]], k = 2
Output: [[19, 7], [3, 8], [1, 9]]       (2 shifts = one whole row of 2 columns)

Input:  grid = [[1, 2, 3], [4, 5, 6]], k = 6
Output: [[1, 2, 3], [4, 5, 6]]          (6 shifts = a full cycle)

shift_grid_generic(&[vec!['a', 'b'], vec!['c', 'd']], 1) => [['d', 'a'], ['b', 'c']]
```

## Constraints

- `1 <= m, n <= 50`
- `-1000 <= grid[i][j] <= 1000`
- `0 <= k <= 100`

## Hints

1. Read the grid row by row into one flat vector: a shift is then a rotation of that vector by one
   position to the right.
2. Only `k % (m * n)` shifts matter.
3. `slice::rotate_right` and `slice::chunks(n)` do the heavy lifting.

## Starter code

Write your code in [`src/practice/p0505_shift_2d_grid.rs`](../src/practice/p0505_shift_2d_grid.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0505
```

```rust
pub struct Solution;

/// Shifts every element `k` positions forward in row-major order.
pub fn shift_grid_generic<T: Clone>(grid: &[Vec<T>], k: usize) -> Vec<Vec<T>> {
    todo!("Write your solution here")
}

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
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
    fn example_one_shift() {
        assert_eq!(
            Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
            vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]
        );
    }

    #[test]
    fn example_whole_row_shift() {
        assert_eq!(
            Solution::shift_grid(vec![vec![3, 8], vec![1, 9], vec![19, 7]], 2),
            vec![vec![19, 7], vec![3, 8], vec![1, 9]]
        );
    }

    #[test]
    fn full_cycles_and_zero() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(Solution::shift_grid(grid.clone(), 6), grid);
        assert_eq!(Solution::shift_grid(grid.clone(), 0), grid);
        assert_eq!(
            Solution::shift_grid(grid, 100),
            Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6]], 4)
        );
    }

    #[test]
    fn generic_helper() {
        assert_eq!(
            shift_grid_generic(&[vec!['a', 'b'], vec!['c', 'd']], 1),
            vec![vec!['d', 'a'], vec!['b', 'c']]
        );
        let words = vec![vec!["x".to_string()], vec!["y".to_string()]];
        assert_eq!(
            shift_grid_generic(&words, 3),
            vec![vec!["y".to_string()], vec!["x".to_string()]]
        );
        let empty: Vec<Vec<u8>> = Vec::new();
        assert!(shift_grid_generic(&empty, 5).is_empty());
    }

    #[test]
    fn single_column_and_large_grid() {
        assert_eq!(
            Solution::shift_grid(vec![vec![1], vec![2], vec![3]], 1),
            vec![vec![3], vec![1], vec![2]]
        );
        let grid: Vec<Vec<i32>> = (0..50)
            .map(|i| (0..50).map(|j| i * 50 + j).collect())
            .collect();
        let shifted = Solution::shift_grid(grid, 100);
        assert_eq!(shifted[0][0], 2400);
        assert_eq!(shifted[2][0], 0);
        assert_eq!(shifted[49][49], 2399);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** flatten the grid in row-major order, rotate the flat vector right by `k % len`, and cut
it back into rows of `n` elements.

**Complexity:** `O(m * n)` time and space.

**Rust concepts:**
- The helper is generic over `T: Clone`; `Solution::shift_grid` instantiates it with `i32`.
- `iter().flatten().cloned()` walks every cell of a `&[Vec<T>]` in row-major order.
- `chunks(n).map(<[T]>::to_vec)` turns the flat vector back into rows; `<[T]>::to_vec` is the
  method `to_vec` of the slice type, used as a function value.

```rust
pub struct Solution;

/// Shifts every element `k` positions forward in row-major order.
pub fn shift_grid_generic<T: Clone>(grid: &[Vec<T>], k: usize) -> Vec<Vec<T>> {
    let cols = grid.first().map_or(0, Vec::len);
    let mut flat: Vec<T> = grid.iter().flatten().cloned().collect();
    if flat.is_empty() || cols == 0 {
        return Vec::new();
    }
    let len = flat.len();
    flat.rotate_right(k % len);
    flat.chunks(cols).map(<[T]>::to_vec).collect()
}

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        shift_grid_generic(&grid, k as usize)
    }
}
```

Full tested solution: [`src/solutions/p0505_shift_2d_grid.rs`](../src/solutions/p0505_shift_2d_grid.rs)

</details>
