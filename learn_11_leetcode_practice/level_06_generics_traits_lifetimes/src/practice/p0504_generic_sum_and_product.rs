// ==================================================================
// PRACTICE 0504: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0504_generic_sum_and_product.md
// Run the tests:          cargo test practice::p0504
// ==================================================================

//! # Problem 0504: Generic Sum and Product
//!
//! - **Difficulty:** Easy
//! - **Topics:** operator traits as bounds, `Add<Output = T>`, `Mul<Output = T>`, `Default`, `From<u8>`
//! - **Source:** Original · [std::ops](https://doc.rust-lang.org/std/ops/index.html)
//!
//! ## Problem
//!
//! Write numeric helpers that work for every number type (`i32`, `u64`, `f64`, ...) without using
//! the `Iterator::sum`/`Iterator::product` methods:
//!
//! - `sum_all<T>(items: &[T]) -> T` where `T: Copy + Default + Add<Output = T>`. The sum of an empty
//!   slice is `T::default()` (zero for every number type).
//! - `product_all<T>(items: &[T]) -> T` where `T: Copy + From<u8> + Mul<Output = T>`. The product of
//!   an empty slice is one, built with `T::from(1)`.
//! - `mean(items: &[f64]) -> Option<f64>` returns the average, or `None` for an empty slice. Use
//!   `sum_all` inside it.
//!
//! ## Examples
//!
//! ```text
//! sum_all(&[1, 2, 3, 4])        => 10
//! sum_all(&[0.5, 0.25])         => 0.75
//! sum_all::<u64>(&[])           => 0
//! product_all(&[2, 3, 4])       => 24
//! product_all::<i64>(&[])       => 1
//! mean(&[1.0, 2.0, 6.0])        => Some(3.0)
//! mean(&[])                     => None
//! ```
//!
//! ## Constraints
//!
//! - The results fit in `T` (no overflow in the tests).
//!
//! ## Hints
//!
//! 1. `a + b` on a generic `T` needs `T: Add<Output = T>`; the `Output = T` part says the result is
//!    a `T` again.
//! 2. There is no "zero" or "one" trait in std, but every number type implements `Default` (zero)
//!    and `From<u8>` (so `T::from(1)` is one).

use std::ops::{Add, Mul};

/// Adds all elements; the empty sum is `T::default()`.
pub fn sum_all<T>(items: &[T]) -> T
where
    T: Copy + Default + Add<Output = T>,
{
    todo!("Write your solution here")
}

/// Multiplies all elements; the empty product is `T::from(1)`.
pub fn product_all<T>(items: &[T]) -> T
where
    T: Copy + From<u8> + Mul<Output = T>,
{
    todo!("Write your solution here")
}

/// The arithmetic mean, or `None` for an empty slice.
pub fn mean(items: &[f64]) -> Option<f64> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_of_several_types() {
        assert_eq!(sum_all(&[1, 2, 3, 4]), 10);
        assert_eq!(sum_all(&[0.5, 0.25]), 0.75);
        assert_eq!(sum_all(&[-5_i64, 5, -1]), -1);
        assert_eq!(sum_all::<u64>(&[]), 0);
    }

    #[test]
    fn products_of_several_types() {
        assert_eq!(product_all(&[2, 3, 4]), 24);
        assert_eq!(product_all(&[1.5_f32, 2.0]), 3.0);
        assert_eq!(product_all::<i64>(&[]), 1);
        assert_eq!(product_all(&[7_u16, 0, 9]), 0);
    }

    #[test]
    fn mean_of_values() {
        assert_eq!(mean(&[1.0, 2.0, 6.0]), Some(3.0));
        assert_eq!(mean(&[-2.5]), Some(-2.5));
        assert_eq!(mean(&[]), None);
    }

    #[test]
    fn large_sum() {
        let values: Vec<u64> = (1..=100_000).collect();
        assert_eq!(sum_all(&values), 5_000_050_000);
        let twos = vec![2_u128; 100];
        assert_eq!(product_all(&twos), 1_u128 << 100);
    }
}
