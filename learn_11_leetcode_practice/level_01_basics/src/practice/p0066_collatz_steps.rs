// ==================================================================
// PRACTICE 0066: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0066_collatz_steps.md
// Run the tests:          cargo test practice::p0066
// ==================================================================

//! # Problem 0066: Collatz Steps
//!
//! - **Difficulty:** Easy
//! - **Topics:** `loop` with a `break` value, `checked_mul`/`checked_add` with `?`, `u64`, tuples
//! - **Source:** Original · [The Rust Book 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html#returning-values-from-loops)
//!
//! ## Problem
//!
//! The **Collatz** process starts from a positive integer and repeats one step until it reaches 1:
//! an even number is halved; an odd number `x` becomes `3x + 1`. Nobody has proved that every start
//! reaches 1, but it has been checked for all numbers you will ever test.
//!
//! Write two functions:
//!
//! 1. `collatz_steps(n: u64) -> Option<u32>` returns how many steps it takes to go from `n` to `1`.
//!    Return `None` if `n == 0`, or if some `3x + 1` would not fit in a `u64`.
//! 2. `longest_chain_below(limit: u64) -> (u64, u32)` looks at every start `n` with `1 <= n < limit`
//!    and returns `(start, steps)` for the start with the **most** steps. If several starts tie, return
//!    the smallest one.
//!
//! ## Examples
//!
//! ```text
//! collatz_steps(1)   -> Some(0)
//! collatz_steps(6)   -> Some(8)      6, 3, 10, 5, 16, 8, 4, 2, 1
//! collatz_steps(27)  -> Some(111)
//! collatz_steps(0)   -> None
//! collatz_steps(u64::MAX) -> None    it is odd, and 3 * u64::MAX + 1 overflows
//!
//! longest_chain_below(10)  -> (9, 19)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= n <= u64::MAX`
//! - `2 <= limit <= 10^5`
//!
//! ## Hints
//!
//! 1. Use `let steps = loop { ... break count; ... };`.
//! 2. `x.checked_mul(3)?.checked_add(1)?` either computes `3x + 1` or returns `None` from the function.
//! 3. For the second function, keep the best `(start, steps)` in `mut` variables and update it only on a strictly larger count.

pub fn collatz_steps(n: u64) -> Option<u32> {
    todo!("Write your solution here")
}

pub fn longest_chain_below(limit: u64) -> (u64, u32) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_starts() {
        assert_eq!(collatz_steps(1), Some(0));
        assert_eq!(collatz_steps(2), Some(1));
        assert_eq!(collatz_steps(6), Some(8));
        assert_eq!(collatz_steps(7), Some(16));
    }

    #[test]
    fn famous_long_chain() {
        assert_eq!(collatz_steps(27), Some(111));
    }

    #[test]
    fn invalid_or_overflowing_starts() {
        assert_eq!(collatz_steps(0), None);
        assert_eq!(collatz_steps(u64::MAX), None);
        // An even start that halves to an odd number which then overflows.
        assert_eq!(collatz_steps(u64::MAX - 1), None);
    }

    #[test]
    fn huge_power_of_two_does_not_overflow() {
        assert_eq!(collatz_steps(1 << 63), Some(63));
    }

    #[test]
    fn longest_chains() {
        assert_eq!(longest_chain_below(2), (1, 0));
        assert_eq!(longest_chain_below(10), (9, 19));
        assert_eq!(longest_chain_below(100), (97, 118));
        assert_eq!(longest_chain_below(100_000), (77_031, 350));
    }
}
