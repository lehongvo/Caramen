# Problem 0819: Unique Paths

- **Difficulty:** Medium
- **Topics:** 2-D grid DP compressed to one row, `vec![1; n]`, in-place updates
- **Source:** [LeetCode 62. Unique Paths](https://leetcode.com/problems/unique-paths/)

## Problem

A robot stands on the top-left cell of an `m x n` grid and wants to reach the bottom-right cell.
At each step it moves exactly one cell **right** or one cell **down**. Return the number of
different paths it can take.

Implement `Solution::unique_paths(m: i32, n: i32) -> i32`.

## Examples

```text
Input:  m = 3, n = 7
Output: 28

Input:  m = 3, n = 2
Output: 3
Explanation: right-down-down, down-right-down, down-down-right.
```

## Constraints

- `1 <= m, n <= 100`
- The answer is at most `2 * 10^9`.

## Hints

1. The number of paths to a cell is the number of paths to the cell above plus the number of
   paths to the cell on its left.
2. Every cell of the first row and first column has exactly one path.
3. You only need the previous row, and it can be updated in place from left to right.

## Starter code

Write your code in [`src/practice/p0819_unique_paths.rs`](../src/practice/p0819_unique_paths.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0819
```

```rust
pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
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
    fn example_3_by_7() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
    }

    #[test]
    fn example_3_by_2() {
        assert_eq!(Solution::unique_paths(3, 2), 3);
    }

    #[test]
    fn single_row_or_column() {
        assert_eq!(Solution::unique_paths(1, 1), 1);
        assert_eq!(Solution::unique_paths(1, 100), 1);
        assert_eq!(Solution::unique_paths(100, 1), 1);
    }

    #[test]
    fn square_grid() {
        assert_eq!(Solution::unique_paths(10, 10), 48_620);
    }

    #[test]
    fn large_answer_fits_in_i32() {
        // C(33, 16)
        assert_eq!(Solution::unique_paths(17, 18), 1_166_803_110);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** keep one row `row[j]` = number of paths to column `j` of the current row. Start with
all ones (first row). For each next row, `row[j] += row[j - 1]` from left to right: before the
update `row[j]` is the count from above, and `row[j - 1]` was already updated (count from the
left).

**Complexity:** `O(m * n)` time, `O(n)` space. (A closed form `C(m + n - 2, m - 1)` also exists.)

**Rust concepts:**
- `vec![1_i32; n]` creates the first row.
- Path counts only grow toward the bottom-right, so if the answer fits in an `i32`, every
  intermediate value does too.

```rust
pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut row = vec![1_i32; n as usize];
        for _ in 1..m {
            for j in 1..row.len() {
                row[j] += row[j - 1];
            }
        }
        row[row.len() - 1]
    }
}
```

Full tested solution: [`src/solutions/p0819_unique_paths.rs`](../src/solutions/p0819_unique_paths.rs)

</details>
