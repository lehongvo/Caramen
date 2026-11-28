// ==================================================================
// PRACTICE 0736: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0736_integer_nth_root.md
// Run the tests:          cargo test practice::p0736
// ==================================================================

//! # Problem 0736: Integer N-th Root
//!
//! - **Difficulty:** Medium
//! - **Topics:** binary search on the answer, monotonic predicate, `checked_pow`, `Option::is_some_and`
//! - **Source:** Original · [`u64::checked_pow`](https://doc.rust-lang.org/std/primitive.u64.html#method.checked_pow)
//!
//! ## Problem
//!
//! Write a function that returns the **integer `n`-th root** of `x`: the largest integer `r` such
//! that `r^n <= x`. For example the integer cube root of `26` is `2` (`2^3 = 8 <= 26` but
//! `3^3 = 27 > 26`).
//!
//! Do not use floating point (`f64` cannot represent every `u64` exactly, so `powf` gives wrong
//! answers for large inputs). Use **binary search on the answer**: search the range of possible
//! roots with a test that says whether a candidate is "small enough". Your function must be correct
//! for every `x` up to `u64::MAX` and must never overflow.
//!
//! Implement `integer_nth_root(x: u64, n: u32) -> u64`.
//!
//! ## Examples
//!
//! ```text
//! integer_nth_root(27, 3)         = 3
//! integer_nth_root(26, 3)         = 2
//! integer_nth_root(0, 5)          = 0
//! integer_nth_root(u64::MAX, 1)   = 18446744073709551615
//! integer_nth_root(u64::MAX, 2)   = 4294967295          (2^32 - 1)
//! integer_nth_root(10^18, 3)      = 1000000
//! ```
//!
//! ## Constraints
//!
//! - `0 <= x <= u64::MAX`
//! - `1 <= n <= 64`
//!
//! ## Hints
//!
//! 1. The test "`r^n <= x`" is `true` for small `r` and `false` from some point on. You are looking
//!    for the last `r` where it is `true`.
//! 2. `r^n` may not fit in a `u64`. `checked_pow` returns `None` in that case, which also means
//!    "too big".
//! 3. For `n >= 2` the root is at most `2^32` (because `(2^32)^2 = 2^64` already overflows). That
//!    bound keeps `hi + 1` from overflowing too.

pub fn integer_nth_root(x: u64, n: u32) -> u64 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_perfect_cube() {
        assert_eq!(integer_nth_root(27, 3), 3);
    }

    #[test]
    fn example_just_below_a_cube() {
        assert_eq!(integer_nth_root(26, 3), 2);
    }

    #[test]
    fn zero_and_one() {
        assert_eq!(integer_nth_root(0, 5), 0);
        assert_eq!(integer_nth_root(1, 64), 1);
        assert_eq!(integer_nth_root(0, 1), 0);
    }

    #[test]
    fn example_largest_inputs() {
        assert_eq!(integer_nth_root(u64::MAX, 1), u64::MAX);
        assert_eq!(integer_nth_root(u64::MAX, 2), 4_294_967_295);
        assert_eq!(integer_nth_root(u64::MAX, 64), 1);
    }

    #[test]
    fn powers_of_two() {
        assert_eq!(integer_nth_root(1 << 63, 63), 2);
        assert_eq!(integer_nth_root(1 << 63, 64), 1);
        assert_eq!(integer_nth_root((1 << 63) - 1, 63), 1);
    }

    #[test]
    fn example_powers_of_ten() {
        assert_eq!(
            integer_nth_root(1_000_000_000_000_000_000, 2),
            1_000_000_000
        );
        assert_eq!(integer_nth_root(1_000_000_000_000_000_000, 3), 1_000_000);
        assert_eq!(integer_nth_root(999_999_999_999_999_999, 3), 999_999);
    }

    #[test]
    fn matches_brute_force_for_small_values() {
        for x in 0..2_000u64 {
            for n in 1..=4u32 {
                let expected = (0..=x).take_while(|r| r.pow(n) <= x).last().unwrap_or(0);
                assert_eq!(integer_nth_root(x, n), expected, "x = {x}, n = {n}");
            }
        }
    }
}
