//! # Problem 0478: Sequences with successors and from_fn
//!
//! - **Difficulty:** Easy
//! - **Topics:** `iter::successors`, `iter::from_fn`, `bool::then`, `checked_mul`, `by_ref`
//! - **Source:** Original · [std `iter::successors`](https://doc.rust-lang.org/std/iter/fn.successors.html) · [std `iter::from_fn`](https://doc.rust-lang.org/std/iter/fn.from_fn.html)
//!
//! ## Problem
//!
//! Writing a struct and an `impl Iterator` is not always necessary. Two standard functions build an
//! iterator from a closure:
//!
//! - `iter::successors(first, |prev| next)`: each item is computed from the previous one; the
//!   sequence ends when the closure returns `None`.
//! - `iter::from_fn(|| ...)`: the closure is the `next` method; it may capture mutable state.
//!
//! Implement:
//!
//! - `collatz(n: u64) -> Vec<u64>`: starting at `n >= 1`, repeatedly apply "halve if even, else
//!   `3x + 1`" until reaching `1` (included).
//! - `powers_of(base: u64, limit: u64) -> Vec<u64>`: `1, base, base^2, ...` while `<= limit`
//!   (`base >= 2`); must not overflow.
//! - `digits_reversed(n: u64) -> Vec<u8>`: the decimal digits of `n` from the last to the first
//!   (`0` -> `[0]`).
//! - `countdown(from: u32) -> Vec<u32>`: `from, from - 1, ..., 0`, written with `from_fn`.
//! - `chunks_of(text: &str, size: usize) -> Vec<String>`: the text cut into pieces of `size`
//!   characters (the last piece may be shorter), written with `from_fn`. `size >= 1`.
//!
//! ## Examples
//!
//! ```text
//! collatz(6)               -> [6, 3, 10, 5, 16, 8, 4, 2, 1]
//! powers_of(3, 100)        -> [1, 3, 9, 27, 81]
//! digits_reversed(1203)    -> [3, 0, 2, 1]
//! countdown(3)             -> [3, 2, 1, 0]
//! chunks_of("abcdefg", 3)  -> ["abc", "def", "g"]
//! ```
//!
//! ## Constraints
//!
//! - `collatz` inputs stay below `10^6` in the tests.
//!
//! ## Hints
//!
//! 1. `successors(Some(n), |&x| (x != 1).then(|| ...))` stops after `1`.
//! 2. For the powers, `x.checked_mul(base)` returns `None` on overflow, and `.filter(|&p| p <= limit)`
//!    turns "too big" into `None` as well.
//! 3. In `chunks_of`, `chars.by_ref().take(size)` takes the next `size` characters without consuming
//!    the iterator `chars` itself.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each sequence is either "next from previous" (`successors`) or "next from some
//! state" (`from_fn`). The closures decide when the sequence ends by returning `None`.
//!
//! **Rust concepts:**
//! - `successors` passes a reference to the previous item; the first item is given as an `Option`
//!   (so an empty sequence is possible with `None`).
//! - `from_fn` closures are `FnMut`: they can own and update state (moved in with `move`).
//! - `Option` combinators keep the stopping rules short: `then`, `then_some`, `checked_*`, `filter`.
//! - `Iterator::by_ref` borrows an iterator so that an adapter like `take` does not consume it.
// @solution-end

use std::iter;

pub fn collatz(n: u64) -> Vec<u64> {
    // @solution-start
    iter::successors(Some(n), |&x| {
        (x != 1).then(|| if x % 2 == 0 { x / 2 } else { 3 * x + 1 })
    })
    .collect()
    // @solution-end
}

pub fn powers_of(base: u64, limit: u64) -> Vec<u64> {
    // @solution-start
    iter::successors(Some(1u64), |&p| p.checked_mul(base))
        .take_while(|&p| p <= limit)
        .collect()
    // @solution-end
}

pub fn digits_reversed(n: u64) -> Vec<u8> {
    // @solution-start
    iter::successors(Some(n), |&x| (x >= 10).then_some(x / 10))
        // x % 10 is a single digit, so the cast is lossless.
        .map(|x| (x % 10) as u8)
        .collect()
    // @solution-end
}

pub fn countdown(from: u32) -> Vec<u32> {
    // @solution-start
    let mut next = Some(from);
    iter::from_fn(move || {
        let current = next?;
        next = current.checked_sub(1);
        Some(current)
    })
    .collect()
    // @solution-end
}

pub fn chunks_of(text: &str, size: usize) -> Vec<String> {
    // @solution-start
    let mut chars = text.chars();
    iter::from_fn(|| {
        let chunk: String = chars.by_ref().take(size).collect();
        (!chunk.is_empty()).then_some(chunk)
    })
    .collect()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collatz_sequences() {
        assert_eq!(collatz(6), vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
        assert_eq!(collatz(1), vec![1]);
        assert_eq!(collatz(27).len(), 112);
        assert_eq!(collatz(837_799).len(), 525);
    }

    #[test]
    fn powers() {
        assert_eq!(powers_of(3, 100), vec![1, 3, 9, 27, 81]);
        assert_eq!(powers_of(2, 1), vec![1]);
        assert_eq!(powers_of(10, 1000), vec![1, 10, 100, 1000]);
        assert_eq!(powers_of(2, u64::MAX).len(), 64);
    }

    #[test]
    fn digits() {
        assert_eq!(digits_reversed(1203), vec![3, 0, 2, 1]);
        assert_eq!(digits_reversed(0), vec![0]);
        assert_eq!(digits_reversed(u64::MAX).len(), 20);
    }

    #[test]
    fn countdowns() {
        assert_eq!(countdown(3), vec![3, 2, 1, 0]);
        assert_eq!(countdown(0), vec![0]);
        assert_eq!(countdown(100).len(), 101);
    }

    #[test]
    fn chunks() {
        assert_eq!(chunks_of("abcdefg", 3), vec!["abc", "def", "g"]);
        assert_eq!(chunks_of("abc", 3), vec!["abc"]);
        assert_eq!(chunks_of("", 2), Vec::<String>::new());
        assert_eq!(chunks_of("üüüü", 2), vec!["üü", "üü"]);
    }
}
