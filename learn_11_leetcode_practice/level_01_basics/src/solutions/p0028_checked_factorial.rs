//! # Problem 0028: Checked Factorial
//!
//! - **Difficulty:** Easy
//! - **Topics:** `u64`, `checked_mul`, the `?` operator on `Option`, `for` over `1..=n`
//! - **Source:** Original · [`u64::checked_mul`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_mul)
//!
//! ## Problem
//!
//! The factorial `n!` is `1 * 2 * ... * n` (and `0! = 1`). Factorials grow extremely fast: `20!`
//! still fits in a `u64`, but `21!` does not.
//!
//! Write `checked_factorial(n: u32) -> Option<u64>` that returns `Some(n!)` when the result fits in
//! a `u64`, and `None` otherwise. Your function must not panic for any `n`.
//!
//! ## Examples
//!
//! ```text
//! checked_factorial(0)   -> Some(1)
//! checked_factorial(5)   -> Some(120)
//! checked_factorial(20)  -> Some(2432902008176640000)
//! checked_factorial(21)  -> None
//! checked_factorial(1000) -> None
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= u32::MAX`
//!
//! ## Hints
//!
//! 1. Multiply step by step with `checked_mul`, which returns `None` on overflow.
//! 2. Once the product overflowed there is no need to keep looping: return `None` right away.
//!    That also keeps `n = u32::MAX` fast.
//! 3. `some_option?` inside a function returning `Option` returns `None` early for you.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** start with `result = 1u64` and, for `k` in `1..=n`, replace it with
//! `result.checked_mul(k)`. If that returns `None`, the factorial does not fit and we return `None`
//! immediately; the loop therefore runs at most 21 times.
//!
//! **Rust concepts:**
//! - `u64::from(k)` widens the `u32` loop variable so both operands have the same type: Rust never
//!   mixes integer types in arithmetic.
//! - The `?` operator applied to an `Option` inside a function that returns `Option` means: "if
//!   this is `Some(v)`, continue with `v`; if it is `None`, return `None` from the function now".
//!   It is shorthand for `match x { Some(v) => v, None => return None }` (compare problem 0025).
//! - The empty range `1..=0` runs zero times, which makes `0! = 1` work without a special case.
// @solution-end

pub fn checked_factorial(n: u32) -> Option<u64> {
    // @solution-start
    let mut result: u64 = 1;
    for k in 1..=n {
        // `?` stops the loop and returns None at the first overflow (at k = 21).
        result = result.checked_mul(u64::from(k))?;
    }
    Some(result)
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_values() {
        assert_eq!(checked_factorial(0), Some(1));
        assert_eq!(checked_factorial(1), Some(1));
        assert_eq!(checked_factorial(5), Some(120));
        assert_eq!(checked_factorial(10), Some(3_628_800));
    }

    #[test]
    fn largest_that_fits() {
        assert_eq!(checked_factorial(20), Some(2_432_902_008_176_640_000));
    }

    #[test]
    fn first_that_overflows() {
        assert_eq!(checked_factorial(21), None);
    }

    #[test]
    fn huge_inputs_return_quickly() {
        assert_eq!(checked_factorial(1000), None);
        assert_eq!(checked_factorial(u32::MAX), None);
    }
}
