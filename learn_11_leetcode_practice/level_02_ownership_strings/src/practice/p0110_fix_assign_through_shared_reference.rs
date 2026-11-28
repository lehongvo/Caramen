// ==================================================================
// PRACTICE 0110: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0110_fix_assign_through_shared_reference.md
// Run the tests:          cargo test practice::p0110
// ==================================================================

//! # Problem 0110: Fix: Assigning Through a Shared Reference
//!
//! - **Difficulty:** Easy
//! - **Topics:** fix the borrow checker, error E0594, `&` vs `&mut`, `&mut [i32]`, `iter_mut`, `clamp`
//! - **Source:** Original · [E0594](https://doc.rust-lang.org/error_codes/E0594.html) · [The Rust Book 4.2](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
//!
//! ## Problem
//!
//! This function should force every value into the range `low..=high`, changing the caller's data
//! in place, but it does not compile:
//!
//! ```text
//! pub fn clamp_all(values: &Vec<i32>, low: i32, high: i32) {
//!     for value in values.iter() {
//!         if *value < low {
//!             *value = low;
//!         }
//!         if *value > high {
//!             *value = high;
//!         }
//!     }
//! }
//!
//! error[E0594]: cannot assign to `*value`, which is behind a `&` reference
//!   |             *value = low;
//!   |             ^^^^^^^^^^^^ `value` is a `&` reference, so the data it refers to cannot be written
//! ```
//!
//! A shared reference (`&`) only allows reading. Implement the corrected
//! `clamp_all(values: &mut [i32], low: i32, high: i32)`: every element smaller than `low` becomes
//! `low`, every element greater than `high` becomes `high`, and the others stay the same.
//!
//! ## Examples
//!
//! ```text
//! Input:  values = [-5, 3, 12, 7], low = 0, high = 10
//! Output: values = [0, 3, 10, 7]
//!
//! Input:  values = [1, 2, 3], low = 2, high = 2
//! Output: values = [2, 2, 2]
//!
//! Input:  values = [], low = 0, high = 1
//! Output: values = []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= values.len() <= 10^5`
//! - `low <= high`
//!
//! ## Hints
//!
//! 1. Two things must change: the parameter type (`&mut`) and the iterator (`iter_mut()`).
//! 2. `i32::clamp(self, min, max)` does the comparison for you.

pub fn clamp_all(values: &mut [i32], low: i32, high: i32) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_mixed_values() {
        let mut values = vec![-5, 3, 12, 7];
        clamp_all(&mut values, 0, 10);
        assert_eq!(values, vec![0, 3, 10, 7]);
    }

    #[test]
    fn example_single_point_range() {
        let mut values = vec![1, 2, 3];
        clamp_all(&mut values, 2, 2);
        assert_eq!(values, vec![2, 2, 2]);
    }

    #[test]
    fn example_empty() {
        let mut values: Vec<i32> = Vec::new();
        clamp_all(&mut values, 0, 1);
        assert!(values.is_empty());
    }

    #[test]
    fn extreme_values() {
        let mut values = [i32::MIN, 0, i32::MAX];
        clamp_all(&mut values, -1, 1);
        assert_eq!(values, [-1, 0, 1]);
    }

    #[test]
    fn only_part_of_an_array() {
        let mut values = [50, 50, 50, 50];
        clamp_all(&mut values[1..3], 0, 10);
        assert_eq!(values, [50, 10, 10, 50]);
    }

    #[test]
    fn large_input() {
        let mut values: Vec<i32> = (-50_000..50_000).collect();
        clamp_all(&mut values, -100, 100);
        assert_eq!(values.iter().filter(|&&v| v == -100).count(), 49_901);
        assert_eq!(values.iter().filter(|&&v| v == 100).count(), 49_900);
        assert_eq!(values[50_000], 0);
    }
}
