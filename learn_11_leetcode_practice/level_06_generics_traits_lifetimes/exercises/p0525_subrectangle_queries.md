# Problem 0525: Subrectangle Queries

- **Difficulty:** Medium
- **Topics:** design struct, small helper struct with a `contains` method, update log vs eager updates, `iter().rev().find()`
- **Source:** [LeetCode 1476. Subrectangle Queries](https://leetcode.com/problems/subrectangle-queries/)

## Problem

Implement `SubrectangleQueries`, which wraps a `rows x cols` matrix of integers:

- `SubrectangleQueries::new(rectangle: Vec<Vec<i32>>) -> Self`
- `update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32)`:
  sets every cell of the rectangle with upper-left corner `(row1, col1)` and lower-right corner
  `(row2, col2)` (both included) to `new_value`.
- `get_value(&self, row: i32, col: i32) -> i32`: the current value of cell `(row, col)`.

Instead of rewriting cells, record every update in a log and answer `get_value` by finding the
**latest** update that covers the cell (or the original value if none does). Represent each logged
update with a small struct `Update { r1, c1, r2, c2, value }` that has a method
`contains(&self, r: usize, c: usize) -> bool`.

This is a LeetCode *design* problem, so there is no `impl Solution`: paste the structs and their
impl blocks into LeetCode.

## Examples

```text
let mut q = SubrectangleQueries::new(vec![vec![1, 2, 1], vec![4, 3, 4], vec![3, 2, 1], vec![1, 1, 1]]);
q.get_value(0, 2)                    => 1
q.update_subrectangle(0, 0, 3, 2, 5) // every cell becomes 5
q.get_value(0, 2)                    => 5
q.get_value(3, 1)                    => 5
q.update_subrectangle(3, 0, 3, 2, 10)
q.get_value(3, 1)                    => 10
q.get_value(0, 2)                    => 5
```

## Constraints

- `1 <= rows, cols <= 100`
- `0 <= row1 <= row2 < rows`, `0 <= col1 <= col2 < cols`
- `1 <= new_value, rectangle[i][j] <= 10^9`
- At most `500` calls to `update_subrectangle` and `get_value` in total.

## Hints

1. Updates only ever overwrite, so the newest update covering a cell decides its value.
2. `self.updates.iter().rev().find(|u| u.contains(r, c))` finds it.

## Starter code

Write your code in [`src/practice/p0525_subrectangle_queries.rs`](../src/practice/p0525_subrectangle_queries.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0525
```

```rust
/// One logged update.
#[derive(Debug, Clone, Copy)]
pub struct Update {
    pub r1: usize,
    pub c1: usize,
    pub r2: usize,
    pub c2: usize,
    pub value: i32,
}

impl Update {
    /// Whether the cell `(r, c)` lies inside this update's rectangle.
    pub fn contains(&self, r: usize, c: usize) -> bool {
        todo!("Write your solution here")
    }
}

/// A matrix with rectangle overwrites.
pub struct SubrectangleQueries {}

impl SubrectangleQueries {
    pub fn new(rectangle: Vec<Vec<i32>>) -> Self {
        todo!("Write your solution here")
    }

    pub fn update_subrectangle(
        &mut self,
        row1: i32,
        col1: i32,
        row2: i32,
        col2: i32,
        new_value: i32,
    ) {
        todo!("Write your solution here")
    }

    pub fn get_value(&self, row: i32, col: i32) -> i32 {
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
    fn example_sequence() {
        let mut q = SubrectangleQueries::new(vec![
            vec![1, 2, 1],
            vec![4, 3, 4],
            vec![3, 2, 1],
            vec![1, 1, 1],
        ]);
        assert_eq!(q.get_value(0, 2), 1);
        q.update_subrectangle(0, 0, 3, 2, 5);
        assert_eq!(q.get_value(0, 2), 5);
        assert_eq!(q.get_value(3, 1), 5);
        q.update_subrectangle(3, 0, 3, 2, 10);
        assert_eq!(q.get_value(3, 1), 10);
        assert_eq!(q.get_value(0, 2), 5);
    }

    #[test]
    fn update_contains() {
        let u = Update {
            r1: 1,
            c1: 2,
            r2: 3,
            c2: 2,
            value: 7,
        };
        assert!(u.contains(1, 2) && u.contains(3, 2));
        assert!(!u.contains(0, 2) && !u.contains(2, 3) && !u.contains(4, 2));
    }

    #[test]
    fn cells_outside_updates_keep_original_values() {
        let mut q = SubrectangleQueries::new(vec![vec![1, 2], vec![3, 4]]);
        q.update_subrectangle(0, 0, 0, 0, 100);
        assert_eq!(q.get_value(0, 0), 100);
        assert_eq!(q.get_value(0, 1), 2);
        assert_eq!(q.get_value(1, 0), 3);
        assert_eq!(q.get_value(1, 1), 4);
    }

    #[test]
    fn matches_eager_model() {
        let mut model = vec![vec![1; 100]; 100];
        let mut q = SubrectangleQueries::new(model.clone());
        for step in 0..250_usize {
            let (r1, c1) = (step * 7 % 100, step * 13 % 100);
            let (r2, c2) = ((r1 + step % 30).min(99), (c1 + step % 17).min(99));
            let value = step as i32 + 2;
            q.update_subrectangle(r1 as i32, c1 as i32, r2 as i32, c2 as i32, value);
            for row in model.iter_mut().take(r2 + 1).skip(r1) {
                for cell in row.iter_mut().take(c2 + 1).skip(c1) {
                    *cell = value;
                }
            }
            let (r, c) = (step * 31 % 100, step * 17 % 100);
            assert_eq!(q.get_value(r as i32, c as i32), model[r][c]);
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** keep the original matrix untouched and push every update onto a log. A query scans the log
from newest to oldest and returns the value of the first update that contains the cell; if there is
none, it returns the original value.

**Complexity:** `O(1)` per update, `O(u)` per query where `u` is the number of updates so far;
`O(rows * cols + u)` space. (Eager updates would cost `O(rows * cols)` per update and `O(1)` per
query; with at most 500 calls both are fine, but the log is cheaper when rectangles are large.)

**Rust concepts:**
- A tiny struct with a method (`Update::contains`) names the idea "this update covers that cell"
  and keeps the search expression readable.
- `Option::map_or` turns "found an update" / "no update" into a value in one step.
- Converting LeetCode's `i32` arguments to `usize` once at the API boundary keeps the internal
  types honest.

```rust
/// One logged update.
#[derive(Debug, Clone, Copy)]
pub struct Update {
    pub r1: usize,
    pub c1: usize,
    pub r2: usize,
    pub c2: usize,
    pub value: i32,
}

impl Update {
    /// Whether the cell `(r, c)` lies inside this update's rectangle.
    pub fn contains(&self, r: usize, c: usize) -> bool {
        (self.r1..=self.r2).contains(&r) && (self.c1..=self.c2).contains(&c)
    }
}

/// A matrix with rectangle overwrites.
pub struct SubrectangleQueries {
    original: Vec<Vec<i32>>,
    updates: Vec<Update>,
}

impl SubrectangleQueries {
    pub fn new(rectangle: Vec<Vec<i32>>) -> Self {
        SubrectangleQueries {
            original: rectangle,
            updates: Vec::new(),
        }
    }

    pub fn update_subrectangle(
        &mut self,
        row1: i32,
        col1: i32,
        row2: i32,
        col2: i32,
        new_value: i32,
    ) {
        self.updates.push(Update {
            r1: row1 as usize,
            c1: col1 as usize,
            r2: row2 as usize,
            c2: col2 as usize,
            value: new_value,
        });
    }

    pub fn get_value(&self, row: i32, col: i32) -> i32 {
        let (r, c) = (row as usize, col as usize);
        self.updates
            .iter()
            .rev()
            .find(|u| u.contains(r, c))
            .map_or(self.original[r][c], |u| u.value)
    }
}
```

Full tested solution: [`src/solutions/p0525_subrectangle_queries.rs`](../src/solutions/p0525_subrectangle_queries.rs)

</details>
