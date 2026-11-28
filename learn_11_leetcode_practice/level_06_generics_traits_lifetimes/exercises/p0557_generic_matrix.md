# Problem 0557: A Generic Matrix

- **Difficulty:** Medium
- **Topics:** generic struct `Matrix<T>`, `Index<(usize, usize)>`/`IndexMut`, `Add`/`Mul` for `&Matrix<T>`, bounds on impl blocks
- **Source:** Original · [std::ops::Index](https://doc.rust-lang.org/std/ops/trait.Index.html)

## Problem

Build a dense matrix that works for any numeric element type. `Matrix<T>` stores `rows`, `cols` and a
row-major `data: Vec<T>` (private fields).

| Impl block | Items |
| --- | --- |
| `impl<T> Matrix<T>` | `from_rows(rows: Vec<Vec<T>>) -> Self` (panics with `"rows must have equal length"` for ragged input), `rows(&self)`, `cols(&self)` |
| `impl<T: Copy + Default> Matrix<T>` | `zeros(rows, cols) -> Self` (every cell `T::default()`), `transpose(&self) -> Self` |
| `impl<T: Copy + Mul<Output = T>> Matrix<T>` | `scale(&self, k: T) -> Self` |
| `impl<T> Index<(usize, usize)>` / `IndexMut<(usize, usize)>` | `m[(r, c)]`, panics if out of range |
| `impl<T: Copy + Add<Output = T>> Add for &Matrix<T>` | element-wise sum, `Output = Matrix<T>`; panics with `"dimension mismatch"` |
| `impl<T: Copy + Default + Add<Output = T> + Mul<Output = T>> Mul for &Matrix<T>` | matrix product, `Output = Matrix<T>`; panics with `"dimension mismatch"` unless `a.cols == b.rows` |

The operators take references, so `&a * &b` does not consume the matrices.

## Examples

```text
let a = Matrix::from_rows(vec![vec![1, 2], vec![3, 4]]);
let b = Matrix::from_rows(vec![vec![5, 6], vec![7, 8]]);
(&a + &b)[(1, 0)]       => 10
(&a * &b)[(0, 1)]       => 22      (1*6 + 2*8)
a.transpose()[(0, 1)]   => 3
a.scale(10)[(1, 1)]     => 40
Matrix::<f64>::zeros(2, 3).cols() => 3
```

## Constraints

- Dimensions up to `100 x 100` in the tests.

## Hints

1. Cell `(r, c)` lives at `data[r * cols + c]`.
2. For the product, `out[(i, j)] = sum over k of a[(i, k)] * b[(k, j)]`, starting from
   `T::default()`.
3. Each impl block lists only the bounds its methods need; `from_rows` needs none at all.

## Starter code

Write your code in [`src/practice/p0557_generic_matrix.rs`](../src/practice/p0557_generic_matrix.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0557
```

```rust
use std::ops::{Add, Index, IndexMut, Mul};

/// A dense row-major matrix.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        todo!("Write your solution here")
    }

    pub fn rows(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn cols(&self) -> usize {
        todo!("Write your solution here")
    }
}

impl<T: Copy + Default> Matrix<T> {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        todo!("Write your solution here")
    }

    pub fn transpose(&self) -> Self {
        todo!("Write your solution here")
    }
}

impl<T: Copy + Mul<Output = T>> Matrix<T> {
    pub fn scale(&self, k: T) -> Self {
        todo!("Write your solution here")
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (r, c): (usize, usize)) -> &T {
        todo!("Write your solution here")
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut T {
        todo!("Write your solution here")
    }
}

impl<T: Copy + Add<Output = T>> Add for &Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<T>) -> Matrix<T> {
        todo!("Write your solution here")
    }
}

impl<T: Copy + Default + Add<Output = T> + Mul<Output = T>> Mul for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Matrix<T> {
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

    fn m(rows: Vec<Vec<i64>>) -> Matrix<i64> {
        Matrix::from_rows(rows)
    }

    #[test]
    fn indexing_and_shape() {
        let mut a = m(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!((a.rows(), a.cols()), (2, 3));
        assert_eq!(a[(1, 2)], 6);
        a[(0, 0)] = 10;
        a[(1, 1)] *= 2;
        assert_eq!(a, m(vec![vec![10, 2, 3], vec![4, 10, 6]]));
        let z = Matrix::<f64>::zeros(2, 3);
        assert_eq!(z.cols(), 3);
        assert_eq!(z[(1, 2)], 0.0);
    }

    #[test]
    fn add_scale_transpose() {
        let a = m(vec![vec![1, 2], vec![3, 4]]);
        let b = m(vec![vec![5, 6], vec![7, 8]]);
        assert_eq!(&a + &b, m(vec![vec![6, 8], vec![10, 12]]));
        assert_eq!(a.transpose(), m(vec![vec![1, 3], vec![2, 4]]));
        assert_eq!(a.scale(10)[(1, 1)], 40);
        let rect = m(vec![vec![1, 2, 3]]);
        assert_eq!(rect.transpose(), m(vec![vec![1], vec![2], vec![3]]));
    }

    #[test]
    fn products() {
        let a = m(vec![vec![1, 2], vec![3, 4]]);
        let b = m(vec![vec![5, 6], vec![7, 8]]);
        assert_eq!(&a * &b, m(vec![vec![19, 22], vec![43, 50]]));
        let row = m(vec![vec![1, 2, 3]]);
        let col = m(vec![vec![4], vec![5], vec![6]]);
        assert_eq!(&row * &col, m(vec![vec![32]]));
        assert_eq!((&col * &row).rows(), 3);
        let f = Matrix::from_rows(vec![vec![0.5, 0.0], vec![0.0, 2.0]]);
        assert_eq!(
            &f * &f,
            Matrix::from_rows(vec![vec![0.25, 0.0], vec![0.0, 4.0]])
        );
    }

    #[test]
    #[should_panic(expected = "dimension mismatch")]
    fn product_dimension_mismatch() {
        let a = m(vec![vec![1, 2, 3]]);
        let _ = &a * &a;
    }

    #[test]
    #[should_panic(expected = "rows must have equal length")]
    fn ragged_rows_panic() {
        let _ = m(vec![vec![1, 2], vec![3]]);
    }

    #[test]
    fn large_identity_product() {
        let n = 100;
        let mut id = Matrix::<i64>::zeros(n, n);
        for i in 0..n {
            id[(i, i)] = 1;
        }
        let a = Matrix::from_rows(
            (0..n)
                .map(|i| (0..n).map(|j| (i * n + j) as i64).collect())
                .collect(),
        );
        assert_eq!(&a * &id, a);
        assert_eq!(&id * &a, a);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** one flat vector plus the dimensions. Indexing computes the offset; the operators build a
new data vector and wrap it in a matrix of the right shape.

**Complexity:** addition `O(r * c)`; multiplication `O(r * k * c)`.

**Rust concepts:**
- Tuple indices: `impl Index<(usize, usize)>` makes `m[(r, c)]` work, with `type Output = T`.
- Implementing operators for `&Matrix<T>` avoids moving large values; `impl<T: ...> Mul for
  &Matrix<T>` elides the reference lifetime.
- Generic numeric code needs explicit bounds (`Add<Output = T>`, `Mul<Output = T>`) and an identity
  value; `T::default()` serves as zero for all number types.

```rust
use std::ops::{Add, Index, IndexMut, Mul};

/// A dense row-major matrix.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn from_rows(rows: Vec<Vec<T>>) -> Self {
        let row_count = rows.len();
        let cols = rows.first().map_or(0, Vec::len);
        assert!(
            rows.iter().all(|r| r.len() == cols),
            "rows must have equal length"
        );
        Matrix {
            rows: row_count,
            cols,
            data: rows.into_iter().flatten().collect(),
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }
}

impl<T: Copy + Default> Matrix<T> {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }

    pub fn transpose(&self) -> Self {
        let mut out = Matrix::zeros(self.cols, self.rows);
        for r in 0..self.rows {
            for c in 0..self.cols {
                out[(c, r)] = self[(r, c)];
            }
        }
        out
    }
}

impl<T: Copy + Mul<Output = T>> Matrix<T> {
    pub fn scale(&self, k: T) -> Self {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|&x| x * k).collect(),
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (r, c): (usize, usize)) -> &T {
        assert!(
            r < self.rows && c < self.cols,
            "index ({r}, {c}) out of range"
        );
        &self.data[r * self.cols + c]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut T {
        assert!(
            r < self.rows && c < self.cols,
            "index ({r}, {c}) out of range"
        );
        &mut self.data[r * self.cols + c]
    }
}

impl<T: Copy + Add<Output = T>> Add for &Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<T>) -> Matrix<T> {
        assert!(
            self.rows == rhs.rows && self.cols == rhs.cols,
            "dimension mismatch"
        );
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self
                .data
                .iter()
                .zip(&rhs.data)
                .map(|(&a, &b)| a + b)
                .collect(),
        }
    }
}

impl<T: Copy + Default + Add<Output = T> + Mul<Output = T>> Mul for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Matrix<T> {
        assert!(self.cols == rhs.rows, "dimension mismatch");
        let mut out = Matrix::zeros(self.rows, rhs.cols);
        for i in 0..self.rows {
            for k in 0..self.cols {
                let a = self[(i, k)];
                for j in 0..rhs.cols {
                    out[(i, j)] = out[(i, j)] + a * rhs[(k, j)];
                }
            }
        }
        out
    }
}
```

Full tested solution: [`src/solutions/p0557_generic_matrix.rs`](../src/solutions/p0557_generic_matrix.rs)

</details>
