# Problem 0985: N-Queens II

- **Difficulty:** Hard
- **Topics:** bitmask backtracking, `u32` masks shifted per row, `trailing_zeros` and `x & x.wrapping_neg()` to pick bits
- **Source:** [LeetCode 52. N-Queens II](https://leetcode.com/problems/n-queens-ii/)

## Problem

Count the distinct ways to place `n` non-attacking queens on an `n x n` board (see problem 0984),
without building the boards.

Implement `Solution::total_n_queens(n: i32) -> i32`.

## Examples

```text
Input:  n = 4      Output: 2
Input:  n = 1      Output: 1
Input:  n = 8      Output: 92
```

## Constraints

- `1 <= n <= 9`

## Hints

1. Represent the attacked columns of the current row as three bitmasks: columns, "left"
   diagonals and "right" diagonals.
2. Moving to the next row shifts the diagonal masks by one bit (left and right).
3. The free squares are `!(cols | left | right) & full`; pop them one at a time with
   `bit = free & free.wrapping_neg()`.

## Starter code

Write your code in [`src/practice/p0985_n_queens_ii.rs`](../src/practice/p0985_n_queens_ii.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0985
```

```rust
pub struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
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
    fn examples() {
        assert_eq!(Solution::total_n_queens(4), 2);
        assert_eq!(Solution::total_n_queens(1), 1);
        assert_eq!(Solution::total_n_queens(8), 92);
    }

    #[test]
    fn no_solutions() {
        assert_eq!(Solution::total_n_queens(2), 0);
        assert_eq!(Solution::total_n_queens(3), 0);
    }

    #[test]
    fn known_counts() {
        let counts: Vec<i32> = (1..=9).map(Solution::total_n_queens).collect();
        assert_eq!(counts, vec![1, 0, 0, 2, 10, 4, 40, 92, 352]);
    }

    #[test]
    fn beyond_the_constraints() {
        assert_eq!(Solution::total_n_queens(10), 724);
        assert_eq!(Solution::total_n_queens(11), 2_680);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the classic bitmask search. For each row, `free` holds the columns not attacked; each
set bit is tried by recursing with the column added to `cols` and to both diagonal masks (shifted
for the next row). When every column is used, one solution was found.

**Complexity:** the same search tree as 0984, but each step is a handful of bit operations.

**Rust concepts:**
- `x & x.wrapping_neg()` isolates the lowest set bit; `free &= free - 1` clears it.
- Masks are kept within `full = (1 << n) - 1` so shifted-out diagonals disappear.

```rust
pub struct Solution;

fn count(full: u32, cols: u32, left: u32, right: u32) -> i32 {
    if cols == full {
        return 1;
    }
    let mut free = !(cols | left | right) & full;
    let mut total = 0;
    while free != 0 {
        let bit = free & free.wrapping_neg();
        free &= free - 1;
        total += count(
            full,
            cols | bit,
            ((left | bit) << 1) & full,
            (right | bit) >> 1,
        );
    }
    total
}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let full = (1_u32 << n) - 1;
        count(full, 0, 0, 0)
    }
}
```

Full tested solution: [`src/solutions/p0985_n_queens_ii.rs`](../src/solutions/p0985_n_queens_ii.rs)

</details>
