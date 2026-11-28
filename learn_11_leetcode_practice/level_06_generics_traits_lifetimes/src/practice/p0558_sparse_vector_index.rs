// ==================================================================
// PRACTICE 0558: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0558_sparse_vector_index.md
// Run the tests:          cargo test practice::p0558
// ==================================================================

//! # Problem 0558: A Sparse Vector with `Index`
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Index` returning a reference to a `static`, `IndexMut` that inserts, `BTreeMap<usize, f64>`, `'static` references
//! - **Source:** Original · [The Rust Reference: static items](https://doc.rust-lang.org/reference/items/static-items.html)
//!
//! ## Problem
//!
//! A sparse vector of length `len` stores only its non-zero entries in a `BTreeMap<usize, f64>`; every
//! other position is `0.0`. Make it indexable like a normal vector:
//!
//! - `impl Index<usize> for SparseVector` (`type Output = f64`): `v[i]` returns the stored value, or a
//!   reference to zero for positions that are not stored. `Index::index` must return a **reference**,
//!   and there is no stored `0.0` to point to, so return a reference to a `static ZERO: f64 = 0.0`.
//! - `impl IndexMut<usize> for SparseVector`: `v[i] = x` and `v[i] += x` must work, which means
//!   `index_mut` inserts a `0.0` entry when the position is not stored yet.
//! - Both panic with `"index {i} out of bounds for length {len}"` when `i >= len`.
//!
//! Also implement:
//!
//! - `SparseVector::new(len: usize) -> Self`, `from_dense(values: &[f64]) -> Self` (stores only the
//!   non-zero values), `len(&self)`, `is_empty(&self)` (length zero).
//! - `stored(&self) -> usize`: how many entries the map holds.
//! - `compact(&mut self)`: removes stored entries equal to `0.0` (they can appear through `IndexMut`).
//! - `dot(&self, other: &SparseVector) -> f64`: the dot product (same length assumed). Only positions
//!   stored in **both** vectors contribute.
//!
//! ## Examples
//!
//! ```text
//! let mut v = SparseVector::new(1_000_000);
//! v[3] = 2.5;
//! v[999_999] += 1.0;
//! v[3]           => 2.5
//! v[4]           => 0.0      (not stored)
//! v.stored()     => 2
//! SparseVector::from_dense(&[0.0, 1.0, 0.0, 3.0]).dot(&SparseVector::from_dense(&[5.0, 2.0, 7.0, 1.0])) => 5.0
//! ```
//!
//! ## Constraints
//!
//! - `0 <= len <= 10^9`; at most `10^5` stored entries in the tests.
//!
//! ## Hints
//!
//! 1. `self.values.get(&i).unwrap_or(&ZERO)`: both sides have type `&f64`, and `&ZERO` is a
//!    `&'static f64`, which is allowed wherever a shorter-lived `&f64` is expected.
//! 2. `self.values.entry(i).or_insert(0.0)` returns the `&mut f64` that `index_mut` needs.
//! 3. For `dot`, iterate over the vector with fewer stored entries and look each position up in the
//!    other one.

use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};

/// The value returned for positions that are not stored.
static ZERO: f64 = 0.0;

/// A vector that stores only its non-zero entries.
#[derive(Debug, Clone, PartialEq)]
pub struct SparseVector {}

impl SparseVector {
    pub fn new(len: usize) -> Self {
        todo!("Write your solution here")
    }

    pub fn from_dense(values: &[f64]) -> Self {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn stored(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn compact(&mut self) {
        todo!("Write your solution here")
    }

    pub fn dot(&self, other: &SparseVector) -> f64 {
        todo!("Write your solution here")
    }
}

impl Index<usize> for SparseVector {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        todo!("Write your solution here")
    }
}

impl IndexMut<usize> for SparseVector {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_and_index_mut() {
        let mut v = SparseVector::new(1_000_000);
        v[3] = 2.5;
        v[999_999] += 1.0;
        assert_eq!(v[3], 2.5);
        assert_eq!(v[4], 0.0);
        assert_eq!(v[999_999], 1.0);
        assert_eq!(v.stored(), 2);
        assert_eq!(v.len(), 1_000_000);
        assert!(!v.is_empty());
    }

    #[test]
    fn reading_does_not_store_but_writing_may_store_zero() {
        let mut v = SparseVector::new(10);
        let _ = v[7];
        assert_eq!(v.stored(), 0);
        v[7] += 0.0; // goes through IndexMut, so an explicit 0.0 entry appears
        assert_eq!(v.stored(), 1);
        v.compact();
        assert_eq!(v.stored(), 0);
        assert_eq!(v[7], 0.0);
    }

    #[test]
    fn dense_conversion_and_dot() {
        let a = SparseVector::from_dense(&[0.0, 1.0, 0.0, 3.0]);
        let b = SparseVector::from_dense(&[5.0, 2.0, 7.0, 1.0]);
        assert_eq!(a.stored(), 2);
        assert_eq!(b.stored(), 4);
        assert_eq!(a.dot(&b), 5.0);
        assert_eq!(b.dot(&a), 5.0);
        assert!(SparseVector::from_dense(&[]).is_empty());
    }

    #[test]
    #[should_panic(expected = "index 10 out of bounds for length 5")]
    fn out_of_bounds_read_panics() {
        let v = SparseVector::new(5);
        let _ = v[10];
    }

    #[test]
    fn large_sparse_dot() {
        let n = 1_000_000_000;
        let mut a = SparseVector::new(n);
        let mut b = SparseVector::new(n);
        for k in 0..100_000 {
            a[k * 10_000] = 1.0;
            if k % 2 == 0 {
                b[k * 10_000] = 2.0;
            }
            b[k * 10_000 + 1] = 5.0;
        }
        assert_eq!(a.dot(&b), 100_000.0);
        assert_eq!(b[n - 1], 0.0);
    }
}
