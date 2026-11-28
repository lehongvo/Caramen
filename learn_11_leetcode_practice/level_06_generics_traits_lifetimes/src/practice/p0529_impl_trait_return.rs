// ==================================================================
// PRACTICE 0529: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0529_impl_trait_return.md
// Run the tests:          cargo test practice::p0529
// ==================================================================

//! # Problem 0529: Returning `impl Trait`
//!
//! - **Difficulty:** Easy
//! - **Topics:** `-> impl Iterator<Item = T>`, `-> impl Fn`, `-> impl FnMut`, `move` closures, lifetime capture in edition 2024
//! - **Source:** Original · [The Rust Book 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits)
//!
//! ## Problem
//!
//! Some types cannot be written down (closures) or are very long (iterator adapter chains). Return
//! them as `impl Trait`: the caller only knows "some type that implements this trait".
//!
//! Implement:
//!
//! - `evens_below(n: u32) -> impl Iterator<Item = u32>`: `0, 2, 4, ...` up to but not including `n`.
//! - `squares_of(values: &[i32]) -> impl Iterator<Item = i64>`: the square of each value as `i64`,
//!   lazily, in order. The iterator borrows `values`.
//! - `make_adder(k: i32) -> impl Fn(i32) -> i32`: a closure that adds `k` to its argument.
//! - `make_counter() -> impl FnMut() -> u32`: a closure that returns `1`, then `2`, then `3`, ...
//!   (each counter has its own state).
//! - `compose(f: impl Fn(i32) -> i32, g: impl Fn(i32) -> i32) -> impl Fn(i32) -> i32`: a closure
//!   computing `g(f(x))`.
//!
//! ## Examples
//!
//! ```text
//! evens_below(7).collect::<Vec<_>>()          => [0, 2, 4, 6]
//! squares_of(&[3, -4]).collect::<Vec<_>>()    => [9, 16]
//! make_adder(5)(10)                           => 15
//! let mut c = make_counter(); c(); c(); c()   => 1, 2, 3
//! compose(make_adder(1), |x| x * 10)(4)       => 50     ((4 + 1) * 10)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= 10^9`
//! - `|values[i]| <= 2^31 - 1`, so every square fits in `i64`.
//!
//! ## Hints
//!
//! 1. `(0..n).step_by(2)` is an iterator; its concrete type is `StepBy<Range<u32>>`, which you do not
//!    have to spell out.
//! 2. A closure that uses `k` after the function returns must own it: `move |x| x + k`.
//! 3. A counter closure owns a `mut` variable and changes it, so it is `FnMut`, not `Fn`.

/// `0, 2, 4, ...` below `n`.
pub fn evens_below(n: u32) -> impl Iterator<Item = u32> {
    std::iter::from_fn(|| todo!())
}

/// The squares of `values` as `i64`, lazily.
pub fn squares_of(values: &[i32]) -> impl Iterator<Item = i64> {
    std::iter::from_fn(|| todo!())
}

/// A closure that adds `k`.
pub fn make_adder(k: i32) -> impl Fn(i32) -> i32 {
    |_| todo!()
}

/// A closure returning 1, 2, 3, ...
pub fn make_counter() -> impl FnMut() -> u32 {
    || todo!()
}

/// A closure computing `g(f(x))`.
pub fn compose(f: impl Fn(i32) -> i32, g: impl Fn(i32) -> i32) -> impl Fn(i32) -> i32 {
    |_| todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evens() {
        assert_eq!(evens_below(7).collect::<Vec<_>>(), vec![0, 2, 4, 6]);
        assert_eq!(evens_below(1).collect::<Vec<_>>(), vec![0]);
        assert_eq!(evens_below(0).count(), 0);
        // `nth` on a stepped range jumps directly, so this is fast even for a huge `n`.
        assert_eq!(
            evens_below(1_000_000_000).nth(499_999_999),
            Some(999_999_998)
        );
        assert_eq!(evens_below(1_000_000_000).nth(500_000_000), None);
    }

    #[test]
    fn squares_borrow_the_input() {
        let values = vec![3, -4, 0];
        let squares: Vec<i64> = squares_of(&values).collect();
        assert_eq!(squares, vec![9, 16, 0]);
        assert_eq!(
            squares_of(&[i32::MIN]).next(),
            Some(4_611_686_018_427_387_904)
        );
        assert_eq!(squares_of(&values).sum::<i64>(), 25);
    }

    #[test]
    fn adders_and_composition() {
        let add5 = make_adder(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(-5), 0);
        let f = compose(make_adder(1), |x| x * 10);
        assert_eq!(f(4), 50);
        let g = compose(|x| x * 10, make_adder(1));
        assert_eq!(g(4), 41);
    }

    #[test]
    fn counters_are_independent() {
        let mut a = make_counter();
        let mut b = make_counter();
        assert_eq!(a(), 1);
        assert_eq!(a(), 2);
        assert_eq!(b(), 1);
        assert_eq!(a(), 3);
    }
}
