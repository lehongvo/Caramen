# Problem 0328: Delete Greatest Value in Each Row

- **Difficulty:** Easy
- **Topics:** `iter_mut` over the rows of a 2-D `Vec`, sorting each row, column maxima
- **Source:** [LeetCode 2500. Delete Greatest Value in Each Row](https://leetcode.com/problems/delete-greatest-value-in-each-row/)

## Problem

You are given an `m x n` matrix `grid` of positive integers. Repeat the following operation
until the matrix is empty:

- delete the largest value from **every** row (if a row has several copies of its largest value,
  delete just one of them);
- add the **largest of the deleted values** to your answer.

Every operation removes one column's worth of elements, so there are exactly `n` operations.
Return the final answer.

Implement `Solution::delete_greatest_value(grid: Vec<Vec<i32>>) -> i32`.

## Examples

```text
Input:  grid = [[1, 2, 4],
                [3, 3, 1]]
Output: 8
Explanation: step 1 deletes 4 and 3, adds 4; step 2 deletes 2 and 3, adds 3;
             step 3 deletes 1 and 1, adds 1. Total 4 + 3 + 1 = 8.

Input:  grid = [[10]]
Output: 10
```

## Constraints

- `1 <= m, n <= 50`
- `1 <= grid[i][j] <= 100`

## Hints

1. If every row is sorted, the values deleted in step `k` are exactly the elements of one column.
2. Sort each row in place with `for row in grid.iter_mut() { row.sort_unstable(); }`.
3. Then add up the maximum of each column.

## Starter code

Write your code in [`src/practice/p0328_delete_greatest_value_in_each_row.rs`](../src/practice/p0328_delete_greatest_value_in_each_row.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0328
```

```rust
pub struct Solution;

impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
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
            Solution::delete_greatest_value(vec![vec![1, 2, 4], vec![3, 3, 1]]),
            8
        );
    }

    #[test]
    fn example_single_cell() {
        assert_eq!(Solution::delete_greatest_value(vec![vec![10]]), 10);
    }

    #[test]
    fn single_column_takes_max() {
        assert_eq!(
            Solution::delete_greatest_value(vec![vec![5], vec![9], vec![2]]),
            9
        );
    }

    #[test]
    fn single_row_sums_everything() {
        assert_eq!(Solution::delete_greatest_value(vec![vec![3, 1, 2, 2]]), 8);
    }

    #[test]
    fn largest_grid() {
        let grid = vec![vec![100; 50]; 50];
        assert_eq!(Solution::delete_greatest_value(grid), 5000);
        let grid: Vec<Vec<i32>> = (0..50)
            .map(|r| (1..=50).map(|c| if r == 0 { c } else { 1 }).collect())
            .collect();
        assert_eq!(Solution::delete_greatest_value(grid), 1275);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** sort every row. The `k`-th operation deletes, from each row, its `k`-th largest value,
which is now in the same column of every row. So the answer is the sum, over all columns, of
the column's maximum.

**Complexity:** `O(m * n log n)` time, `O(1)` extra space.

**Rust concepts:**
- `grid.iter_mut()` yields `&mut Vec<i32>` for each row, so each row can be sorted in place.
- The column maximum is `grid.iter().map(|row| row[c]).max()`, an `Option` because an iterator
  could be empty; `unwrap_or(0)` keeps it panic-free.
- Deleting the largest element of each row literally (with `remove`) would also work, but sorting
  once is simpler and faster.

```rust
pub struct Solution;

impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        for row in grid.iter_mut() {
            row.sort_unstable();
        }
        let cols = grid.first().map_or(0, |row| row.len());
        (0..cols)
            .map(|c| grid.iter().map(|row| row[c]).max().unwrap_or(0))
            .sum()
    }
}
```

Full tested solution: [`src/solutions/p0328_delete_greatest_value_in_each_row.rs`](../src/solutions/p0328_delete_greatest_value_in_each_row.rs)

</details>
