//! # Problem 0055: Countdown by Step
//!
//! - **Difficulty:** Easy
//! - **Topics:** `for` over ranges, `rev()`, `step_by()`, inclusive ranges `..=`, `div_ceil`
//! - **Source:** Original · [The Rust Book 3.5](https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for)
//!
//! ## Problem
//!
//! Ranges are the backbone of `for` loops in Rust. Write two functions:
//!
//! 1. `countdown(start: u32, step: usize) -> Vec<u32>` returns `start, start - step,
//!    start - 2 * step, ...`, stopping before the numbers would go below 0. The list always begins
//!    with `start`, and it contains `0` only if the countdown lands exactly on it.
//! 2. `multiples_in_range(lo: u32, hi: u32, k: u32) -> Vec<u32>` returns, in increasing order, every
//!    multiple of `k` in the closed interval `[lo, hi]` (empty if there is none, or if `lo > hi`).
//!
//! Try to write both with a range, `rev()` and `step_by()` instead of manual arithmetic in a `while` loop.
//!
//! ## Examples
//!
//! ```text
//! countdown(10, 3)              -> [10, 7, 4, 1]
//! countdown(6, 2)               -> [6, 4, 2, 0]
//! countdown(0, 5)               -> [0]
//!
//! multiples_in_range(10, 30, 7) -> [14, 21, 28]
//! multiples_in_range(0, 6, 3)   -> [0, 3, 6]
//! multiples_in_range(8, 9, 5)   -> []
//! ```
//!
//! ## Constraints
//!
//! - `0 <= start <= 10^6`, `1 <= step <= 10^6`
//! - `0 <= lo, hi <= 10^6`, `1 <= k <= 10^6`
//!
//! ## Hints
//!
//! 1. `(0..=start).rev()` counts down from `start` to `0`. `.step_by(n)` keeps every `n`-th element,
//!    starting with the first.
//! 2. For the multiples, the first multiple of `k` that is `>= lo` is `lo.div_ceil(k) * k`.
//! 3. `.collect()` turns the iterator into a `Vec`, or push inside a `for` loop.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `countdown` is `(0..=start).rev().step_by(step)`: the reversed range yields `start`
//! first, and `step_by` skips `step - 1` elements between the ones it keeps. For the multiples,
//! start at the first multiple not below `lo` and step by `k` up to `hi`.
//!
//! **Rust concepts:**
//! - `a..b` excludes `b`; `a..=b` includes it. An empty range (start after the end) simply yields
//!   nothing, so `multiples_in_range` needs no special case for `lo > hi`.
//! - `rev()` works on ranges because they can be walked from both ends. `step_by` takes a `usize`
//!   and **panics** if the step is 0, so the constraints require `step >= 1`.
//! - `u32::div_ceil` is integer division rounding up.
// @solution-end

pub fn countdown(start: u32, step: usize) -> Vec<u32> {
    // @solution-start
    let mut out = Vec::new();
    for n in (0..=start).rev().step_by(step) {
        out.push(n);
    }
    out
    // @solution-end
}

pub fn multiples_in_range(lo: u32, hi: u32, k: u32) -> Vec<u32> {
    // @solution-start
    let first = lo.div_ceil(k) * k; // at most 2 * 10^6, no overflow
    (first..=hi).step_by(k as usize).collect()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn countdown_examples() {
        assert_eq!(countdown(10, 3), vec![10, 7, 4, 1]);
        assert_eq!(countdown(6, 2), vec![6, 4, 2, 0]);
        assert_eq!(countdown(0, 5), vec![0]);
    }

    #[test]
    fn countdown_step_one_and_large_step() {
        assert_eq!(countdown(4, 1), vec![4, 3, 2, 1, 0]);
        assert_eq!(countdown(4, 100), vec![4]);
    }

    #[test]
    fn countdown_long() {
        let v = countdown(1_000_000, 7);
        assert_eq!(v.len(), 142_858);
        assert_eq!(v[0], 1_000_000);
        assert_eq!(*v.last().expect("non-empty"), 1_000_000 % 7);
    }

    #[test]
    fn multiples_examples() {
        assert_eq!(multiples_in_range(10, 30, 7), vec![14, 21, 28]);
        assert_eq!(multiples_in_range(0, 6, 3), vec![0, 3, 6]);
        assert_eq!(multiples_in_range(8, 9, 5), Vec::<u32>::new());
    }

    #[test]
    fn multiples_edge_cases() {
        assert_eq!(multiples_in_range(5, 5, 5), vec![5]);
        assert_eq!(multiples_in_range(9, 3, 1), Vec::<u32>::new());
        assert_eq!(
            multiples_in_range(999_999, 1_000_000, 1_000_000),
            vec![1_000_000]
        );
        assert_eq!(multiples_in_range(0, 1_000_000, 1).len(), 1_000_001);
    }
}
