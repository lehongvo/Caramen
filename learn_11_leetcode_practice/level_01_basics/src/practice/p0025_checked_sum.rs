// ==================================================================
// PRACTICE 0025: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0025_checked_sum.md
// Run the tests:          cargo test practice::p0025
// ==================================================================

//! # Problem 0025: Checked Sum
//!
//! - **Difficulty:** Easy
//! - **Topics:** integer overflow, `checked_add`, `Option`, widening to `i64`, iterating a slice by reference
//! - **Source:** Original · [`i32::checked_add`](https://doc.rust-lang.org/std/primitive.i32.html#method.checked_add)
//!
//! ## Problem
//!
//! A sensor stores its running total in a 32-bit register. Write two functions that add up a slice
//! of readings:
//!
//! 1. `checked_sum(nums: &[i32]) -> Option<i32>` adds the readings **from left to right** in an
//!    `i32`. If any partial sum would leave the `i32` range, it returns `None`; otherwise it returns
//!    `Some(total)`. The sum of an empty slice is `Some(0)`.
//! 2. `exact_sum(nums: &[i32]) -> i64` returns the true mathematical sum, which never overflows
//!    because it is accumulated in an `i64`.
//!
//! ## Examples
//!
//! ```text
//! checked_sum(&[1, 2, 3])                 -> Some(6)
//! checked_sum(&[])                        -> Some(0)
//! checked_sum(&[i32::MAX, 1])             -> None
//! checked_sum(&[i32::MAX, 1, -1])         -> None     the partial sum i32::MAX + 1 already overflows
//! exact_sum(&[i32::MAX, 1, -1])           -> 2147483647
//! exact_sum(&[i32::MIN, i32::MIN])        -> -4294967296
//! ```
//!
//! ## Constraints
//!
//! - `0 <= nums.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `a.checked_add(b)` returns `Some(a + b)` when the result fits and `None` when it overflows.
//! 2. Use `match` on that `Option` inside a `for` loop and `return None` as soon as it fails.
//! 3. For `exact_sum`, convert each element with `i64::from(x)` before adding.

pub fn checked_sum(nums: &[i32]) -> Option<i32> {
    todo!("Write your solution here")
}

pub fn exact_sum(nums: &[i32]) -> i64 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sums() {
        assert_eq!(checked_sum(&[1, 2, 3]), Some(6));
        assert_eq!(checked_sum(&[-5, 5]), Some(0));
        assert_eq!(exact_sum(&[1, 2, 3]), 6);
    }

    #[test]
    fn empty_slice() {
        assert_eq!(checked_sum(&[]), Some(0));
        assert_eq!(exact_sum(&[]), 0);
    }

    #[test]
    fn overflow_is_detected() {
        assert_eq!(checked_sum(&[i32::MAX, 1]), None);
        assert_eq!(checked_sum(&[i32::MIN, -1]), None);
        assert_eq!(checked_sum(&[i32::MAX, 1, -1]), None);
    }

    #[test]
    fn reaching_the_limits_exactly_is_fine() {
        assert_eq!(checked_sum(&[i32::MAX - 1, 1]), Some(i32::MAX));
        assert_eq!(checked_sum(&[i32::MIN + 1, -1]), Some(i32::MIN));
        assert_eq!(checked_sum(&[i32::MAX, i32::MIN]), Some(-1));
    }

    #[test]
    fn exact_sum_handles_huge_totals() {
        assert_eq!(exact_sum(&[i32::MAX, 1, -1]), 2_147_483_647);
        assert_eq!(exact_sum(&[i32::MIN, i32::MIN]), -4_294_967_296);
        let big = vec![i32::MAX; 100_000];
        assert_eq!(exact_sum(&big), 214_748_364_700_000);
        assert_eq!(checked_sum(&big), None);
    }
}
