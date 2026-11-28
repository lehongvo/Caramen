// ==================================================================
// PRACTICE 0600: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0600_const_generic_matrix.md
// Run the tests:          cargo test practice::p0600
// ==================================================================

//! # Problem 0600: A Const-Generic Matrix
//!
//! - **Difficulty:** Hard
//! - **Topics:** const generics (`Matrix<const R: usize, const C: usize>`), dimension-checked `Mul<Matrix<K, C>> for Matrix<R, K>`, impl blocks for square matrices only (`impl<const N: usize> Matrix<N, N>`), fast exponentiation
//! - **Source:** Original · [The Rust Reference: const generics](https://doc.rust-lang.org/reference/items/generics.html#const-generics)
//!
//! ## Problem
//!
//! Problem 0557 checked matrix dimensions at **run time**. With const generics the dimensions become
//! part of the type, and the compiler rejects a product of incompatible matrices.
//!
//! `Matrix<const R: usize, const C: usize>` stores `data: [[i64; C]; R]` (a private field) and derives
//! `Debug, Clone, Copy, PartialEq, Eq`.
//!
//! For every `R` and `C`:
//!
//! - `new(data: [[i64; C]; R]) -> Self`, `zero() -> Self`, `get(&self, r: usize, c: usize) -> i64`.
//! - `rows() -> usize` and `cols() -> usize` (associated functions returning `R` and `C`).
//! - `transpose(&self) -> Matrix<C, R>`: note the swapped type.
//! - `Add` (only between matrices of the same shape).
//! - `Mul<Matrix<K, C>> for Matrix<R, K>` with `Output = Matrix<R, C>`, for any `K`: an `R x K` matrix
//!   times a `K x C` matrix is `R x C`. Multiplying a `2 x 3` by a `2 x 3` does not compile.
//!
//! Only for square matrices (`impl<const N: usize> Matrix<N, N>`):
//!
//! - `identity() -> Self`, `trace(&self) -> i64`.
//! - `pow(self, e: u32) -> Self` by repeated squaring (`pow(0)` is the identity). Do not square the base
//!   after the last needed bit, to avoid needless overflow.
//!
//! Finally `fibonacci(n: u32) -> i64` using `[[1, 1], [1, 0]]` raised to a power: `F(0) = 0`,
//! `F(1) = 1`, ... up to `F(92)`, the largest that fits in `i64`.
//!
//! ## Examples
//!
//! ```text
//! let a = Matrix::new([[1, 2, 3], [4, 5, 6]]);        // Matrix<2, 3>
//! a.transpose().get(2, 1)                             => 6        (a Matrix<3, 2>)
//! (a * a.transpose()).get(0, 0)                       => 14       (a Matrix<2, 2>)
//! Matrix::<3, 3>::identity().trace()                  => 3
//! Matrix::new([[1, 1], [1, 0]]).pow(10).get(0, 1)     => 55
//! fibonacci(92)                                       => 7540113804746346429
//! ```
//!
//! ## Constraints
//!
//! - Results fit in `i64` in the tests.
//!
//! ## Hints
//!
//! 1. Const parameters are used like values inside the impl: `for r in 0..R`.
//! 2. `[[0; C]; R]` builds the zero array for any `R` and `C` (`i64` is `Copy`).
//! 3. `M^n = [[F(n + 1), F(n)], [F(n), F(n - 1)]]`, so `F(n)` is the top-left entry of `M^(n - 1)`;
//!    computing `M^(n - 1)` instead of `M^n` keeps every entry within `i64` up to `n = 92`.

use std::ops::{Add, Mul};

/// An `R x C` matrix of `i64` with dimensions checked at compile time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matrix<const R: usize, const C: usize> {
    data: [[i64; C]; R],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    pub fn new(data: [[i64; C]; R]) -> Self {
        todo!("Write your solution here")
    }

    pub fn zero() -> Self {
        todo!("Write your solution here")
    }

    pub fn get(&self, r: usize, c: usize) -> i64 {
        todo!("Write your solution here")
    }

    pub fn rows() -> usize {
        todo!("Write your solution here")
    }

    pub fn cols() -> usize {
        todo!("Write your solution here")
    }

    pub fn transpose(&self) -> Matrix<C, R> {
        todo!("Write your solution here")
    }
}

impl<const N: usize> Matrix<N, N> {
    pub fn identity() -> Self {
        todo!("Write your solution here")
    }

    pub fn trace(&self) -> i64 {
        todo!("Write your solution here")
    }

    pub fn pow(self, mut e: u32) -> Self {
        todo!("Write your solution here")
    }
}

impl<const R: usize, const C: usize> Add for Matrix<R, C> {
    type Output = Matrix<R, C>;

    fn add(self, rhs: Matrix<R, C>) -> Matrix<R, C> {
        todo!("Write your solution here")
    }
}

impl<const R: usize, const K: usize, const C: usize> Mul<Matrix<K, C>> for Matrix<R, K> {
    type Output = Matrix<R, C>;

    fn mul(self, rhs: Matrix<K, C>) -> Matrix<R, C> {
        todo!("Write your solution here")
    }
}

/// `F(n)` computed with matrix exponentiation.
pub fn fibonacci(n: u32) -> i64 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shapes_and_transpose() {
        let a = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        assert_eq!((Matrix::<2, 3>::rows(), Matrix::<2, 3>::cols()), (2, 3));
        let t: Matrix<3, 2> = a.transpose();
        assert_eq!(t.get(2, 1), 6);
        assert_eq!(t, Matrix::new([[1, 4], [2, 5], [3, 6]]));
        assert_eq!(Matrix::<2, 2>::zero(), Matrix::new([[0, 0], [0, 0]]));
    }

    #[test]
    fn dimension_checked_products() {
        let a = Matrix::new([[1, 2, 3], [4, 5, 6]]);
        let square: Matrix<2, 2> = a * a.transpose();
        assert_eq!(square, Matrix::new([[14, 32], [32, 77]]));
        let big: Matrix<3, 3> = a.transpose() * a;
        assert_eq!(big.trace(), 17 + 29 + 45); // squared column norms
        let v = Matrix::new([[1], [1], [1]]); // a 3 x 1 column
        assert_eq!(a * v, Matrix::new([[6], [15]]));
        assert_eq!(a + a, Matrix::new([[2, 4, 6], [8, 10, 12]]));
    }

    #[test]
    fn square_only_methods() {
        assert_eq!(Matrix::<3, 3>::identity().trace(), 3);
        let m = Matrix::new([[2, 0], [0, 3]]);
        assert_eq!(m.pow(0), Matrix::identity());
        assert_eq!(m.pow(5), Matrix::new([[32, 0], [0, 243]]));
        assert_eq!(Matrix::new([[1, 1], [1, 0]]).pow(10).get(0, 1), 55);
        let id: Matrix<4, 4> = Matrix::identity();
        assert_eq!(id * id, id);
    }

    #[test]
    fn fibonacci_numbers() {
        let first: Vec<i64> = (0..10).map(fibonacci).collect();
        assert_eq!(first, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
        assert_eq!(fibonacci(50), 12_586_269_025);
        assert_eq!(fibonacci(92), 7_540_113_804_746_346_429);
    }
}
