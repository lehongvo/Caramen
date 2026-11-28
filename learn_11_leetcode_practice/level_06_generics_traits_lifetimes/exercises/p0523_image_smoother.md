# Problem 0523: Image Smoother

- **Difficulty:** Easy
- **Topics:** generic struct `Grid<T>`, `impl<T> Grid<T>`, generic `map<U, F>`, neighbourhood queries
- **Source:** [LeetCode 661. Image Smoother](https://leetcode.com/problems/image-smoother/)

## Problem

A grayscale image is an `m x n` matrix of integers. **Smoothing** replaces every pixel with the
average of itself and its (up to) eight neighbours, i.e. the 3x3 block centred on it, clipped at the
image border. The average is rounded **down** (integer division).

Implement `Solution::image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.

Build it on top of a reusable generic grid stored in row-major order. `Grid<T>` has private
fields `rows`, `cols` and `cells: Vec<T>`; implement in `impl<T> Grid<T>`:

- `from_rows(rows: Vec<Vec<T>>) -> Self`: every inner vector is one row (all rows have the same
  length; an empty input gives a `0 x 0` grid).
- `rows(&self) -> usize`, `cols(&self) -> usize`
- `get(&self, r: usize, c: usize) -> Option<&T>`: `None` outside the grid.
- `block3x3(&self, r: usize, c: usize) -> Vec<&T>`: the cell `(r, c)` and its neighbours that lie
  inside the grid, in row-major order.
- `map<U, F>(&self, f: F) -> Grid<U> where F: Fn(usize, usize, &T) -> U`: a new grid of the same
  shape whose cell `(r, c)` is `f(r, c, &old_cell)`.
- `into_rows(self) -> Vec<Vec<T>>`

## Examples

```text
Input:  img = [[1, 1, 1], [1, 0, 1], [1, 1, 1]]
Output: [[0, 0, 0], [0, 0, 0], [0, 0, 0]]
        corner: 3 / 4 = 0; edge: 5 / 6 = 0; centre: 8 / 9 = 0

Input:  img = [[100, 200, 100], [200, 50, 200], [100, 200, 100]]
Output: [[137, 141, 137], [141, 138, 141], [137, 141, 137]]

Grid::from_rows(vec![vec!['a', 'b'], vec!['c', 'd']]).block3x3(0, 0) => [&'a', &'b', &'c', &'d']
```

## Constraints

- `1 <= m, n <= 200`
- `0 <= img[i][j] <= 255`

## Hints

1. Cell `(r, c)` lives at `cells[r * cols + c]`.
2. For the 3x3 block, loop over `r.saturating_sub(1)..=(r + 1).min(rows - 1)` and the same for
   columns; `saturating_sub` avoids underflow at row `0`.
3. `image_smoother` is then `grid.map(|r, c, _| average of grid.block3x3(r, c))`.

## Starter code

Write your code in [`src/practice/p0523_image_smoother.rs`](../src/practice/p0523_image_smoother.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0523
```

```rust
pub struct Solution;

/// A rectangular grid stored in row-major order.
#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        todo!("Write your solution here")
    }

    pub fn rows(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn cols(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&T> {
        todo!("Write your solution here")
    }

    /// The cell and its in-bounds neighbours, row-major.
    pub fn block3x3(&self, r: usize, c: usize) -> Vec<&T> {
        todo!("Write your solution here")
    }

    pub fn map<U, F>(&self, f: F) -> Grid<U>
    where
        F: Fn(usize, usize, &T) -> U,
    {
        todo!("Write your solution here")
    }

    pub fn into_rows(self) -> Vec<Vec<T>> {
        todo!("Write your solution here")
    }
}

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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
    fn example_dark_center() {
        assert_eq!(
            Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
        );
    }

    #[test]
    fn example_bright_pattern() {
        assert_eq!(
            Solution::image_smoother(vec![
                vec![100, 200, 100],
                vec![200, 50, 200],
                vec![100, 200, 100]
            ]),
            vec![
                vec![137, 141, 137],
                vec![141, 138, 141],
                vec![137, 141, 137]
            ]
        );
    }

    #[test]
    fn single_pixel_and_single_row() {
        assert_eq!(Solution::image_smoother(vec![vec![255]]), vec![vec![255]]);
        assert_eq!(
            Solution::image_smoother(vec![vec![0, 9, 3]]),
            vec![vec![4, 4, 6]]
        );
    }

    #[test]
    fn generic_grid_with_chars() {
        let g = Grid::from_rows(vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);
        assert_eq!((g.rows(), g.cols()), (2, 3));
        assert_eq!(g.get(1, 2), Some(&'f'));
        assert_eq!(g.get(2, 0), None);
        assert_eq!(g.block3x3(0, 0), vec![&'a', &'b', &'d', &'e']);
        assert_eq!(g.block3x3(1, 1).len(), 6);
        let upper = g.map(|r, c, ch| format!("{}{r}{c}", ch.to_ascii_uppercase()));
        assert_eq!(upper.get(1, 0).map(String::as_str), Some("D10"));
        assert_eq!(
            g.into_rows(),
            vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]
        );
    }

    #[test]
    fn large_uniform_image() {
        let img = vec![vec![77; 200]; 200];
        let out = Solution::image_smoother(img.clone());
        assert_eq!(out, img);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** store the image in a generic `Grid<T>`. `map` builds a new grid by calling a closure for
every position; the closure reads the clipped 3x3 block from the **old** grid, so updates never
affect each other.

**Complexity:** `O(m * n)` time (at most 9 reads per pixel), `O(m * n)` space.

**Rust concepts:**
- `Grid<T>` works for pixels (`i32`), characters, or anything else; only `image_smoother` needs
  numbers.
- `map<U, F>` has its own type parameters: `U` is the new element type and `F` the closure type
  with bound `Fn(usize, usize, &T) -> U`.
- `block3x3` returns `Vec<&T>`, references into the grid, so it needs no `Clone` bound.
- Every value is between `0` and `255`, so the sum of nine of them fits easily in `i32`.

```rust
pub struct Solution;

/// A rectangular grid stored in row-major order.
#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        let row_count = rows.len();
        let cols = rows.first().map_or(0, Vec::len);
        let cells: Vec<T> = rows.into_iter().flatten().collect();
        Grid {
            rows: row_count,
            cols,
            cells,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&T> {
        if r < self.rows && c < self.cols {
            self.cells.get(r * self.cols + c)
        } else {
            None
        }
    }

    /// The cell and its in-bounds neighbours, row-major.
    pub fn block3x3(&self, r: usize, c: usize) -> Vec<&T> {
        let mut out = Vec::with_capacity(9);
        if r >= self.rows || c >= self.cols {
            return out;
        }
        for rr in r.saturating_sub(1)..=(r + 1).min(self.rows - 1) {
            for cc in c.saturating_sub(1)..=(c + 1).min(self.cols - 1) {
                out.push(&self.cells[rr * self.cols + cc]);
            }
        }
        out
    }

    pub fn map<U, F>(&self, f: F) -> Grid<U>
    where
        F: Fn(usize, usize, &T) -> U,
    {
        let cells = self
            .cells
            .iter()
            .enumerate()
            .map(|(i, cell)| f(i / self.cols, i % self.cols, cell))
            .collect();
        Grid {
            rows: self.rows,
            cols: self.cols,
            cells,
        }
    }

    pub fn into_rows(self) -> Vec<Vec<T>> {
        let mut rows = Vec::with_capacity(self.rows);
        let mut iter = self.cells.into_iter();
        for _ in 0..self.rows {
            rows.push(iter.by_ref().take(self.cols).collect());
        }
        rows
    }
}

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let grid = Grid::from_rows(img);
        grid.map(|r, c, _| {
            let block = grid.block3x3(r, c);
            let sum: i32 = block.iter().copied().sum();
            sum / block.len() as i32
        })
        .into_rows()
    }
}
```

Full tested solution: [`src/solutions/p0523_image_smoother.rs`](../src/solutions/p0523_image_smoother.rs)

</details>
