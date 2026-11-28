// ==================================================================
// PRACTICE 0483: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0483_double_ended_step_range.md
// Run the tests:          cargo test practice::p0483
// ==================================================================

//! # Problem 0483: A Double-Ended Step Range
//!
//! - **Difficulty:** Medium
//! - **Topics:** `DoubleEndedIterator::next_back`, `rev` on a custom iterator, `ExactSizeIterator`, consuming from both ends, `iter::from_fn`
//! - **Source:** Original · [std `DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html) · [std `Iterator::rev`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.rev)
//!
//! ## Problem
//!
//! `rev()` only works on iterators that can also produce items **from the back**: they implement
//! `DoubleEndedIterator`, whose method `next_back` returns the last remaining item. Both ends
//! consume the same sequence: once the front and the back meet, both return `None`.
//!
//! `StepRange::new(start, end, step)` represents `start, start + step, start + 2 * step, ...` for
//! all values strictly below `end` (like `(start..end).step_by(step)`). Implement:
//!
//! - `StepRange::new(start: i64, end: i64, step: i64) -> StepRange`. A step that is not positive is
//!   a bug: panic with `"step must be positive"`. If `end <= start` the range is empty.
//! - `impl Iterator` (`type Item = i64`) with an exact `size_hint`, and
//!   `impl DoubleEndedIterator` with `next_back`. (`ExactSizeIterator` is given.)
//! - `last_k(start, end, step, k) -> Vec<i64>`: the last `k` values, largest first.
//! - `from_both_ends(start, end, step) -> Vec<i64>`: front, back, front, back, ... until the range is
//!   exhausted.
//!
//! ## Examples
//!
//! ```text
//! StepRange::new(0, 10, 3) yields 0, 3, 6, 9;   .rev() yields 9, 6, 3, 0
//! StepRange::new(5, 5, 1)  yields nothing
//! last_k(0, 10, 3, 2)              -> [9, 6]
//! from_both_ends(1, 8, 1)          -> [1, 7, 2, 6, 3, 5, 4]
//! ```
//!
//! ## Constraints
//!
//! - `-10^12 <= start, end <= 10^12`, `step <= 10^12`.
//!
//! ## Hints
//!
//! 1. Store the next front value, the step and the **number of items left**. The last item left is
//!    `front + step * (left - 1)`, so `next_back` needs no extra field.
//! 2. The number of items is `ceil((end - start) / step)` when `end > start`.
//! 3. `from_both_ends` can alternate inside `iter::from_fn` with a `bool` toggle.

use std::iter;

pub struct StepRange {}

impl StepRange {
    pub fn new(start: i64, end: i64, step: i64) -> StepRange {
        todo!("Write your solution here")
    }
}

impl Iterator for StepRange {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        todo!("Write your solution here")
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        todo!("Write your solution here")
    }
}

impl DoubleEndedIterator for StepRange {
    fn next_back(&mut self) -> Option<i64> {
        todo!("Write your solution here")
    }
}

impl ExactSizeIterator for StepRange {}

pub fn last_k(start: i64, end: i64, step: i64, k: usize) -> Vec<i64> {
    todo!("Write your solution here")
}

pub fn from_both_ends(start: i64, end: i64, step: i64) -> Vec<i64> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward_and_backward() {
        assert_eq!(
            StepRange::new(0, 10, 3).collect::<Vec<_>>(),
            vec![0, 3, 6, 9]
        );
        assert_eq!(
            StepRange::new(0, 10, 3).rev().collect::<Vec<_>>(),
            vec![9, 6, 3, 0]
        );
        assert_eq!(StepRange::new(0, 9, 3).collect::<Vec<_>>(), vec![0, 3, 6]);
        assert_eq!(
            StepRange::new(-5, 0, 2).rev().collect::<Vec<_>>(),
            vec![-1, -3, -5]
        );
    }

    #[test]
    fn empty_ranges_and_lengths() {
        assert_eq!(StepRange::new(5, 5, 1).count(), 0);
        assert_eq!(StepRange::new(5, 1, 1).next_back(), None);
        assert_eq!(StepRange::new(0, 10, 3).len(), 4);
        assert_eq!(
            StepRange::new(-1_000_000_000_000, 1_000_000_000_000, 1_000).len(),
            2_000_000_000
        );
    }

    #[test]
    fn both_ends_meet_in_the_middle() {
        let mut r = StepRange::new(0, 5, 1);
        assert_eq!(r.next(), Some(0));
        assert_eq!(r.next_back(), Some(4));
        assert_eq!(r.next_back(), Some(3));
        assert_eq!(r.next(), Some(1));
        assert_eq!(r.next(), Some(2));
        assert_eq!(r.next_back(), None);
        assert_eq!(r.next(), None);
    }

    #[test]
    fn helpers() {
        assert_eq!(last_k(0, 10, 3, 2), vec![9, 6]);
        assert_eq!(last_k(0, 10, 3, 10), vec![9, 6, 3, 0]);
        assert_eq!(from_both_ends(1, 8, 1), vec![1, 7, 2, 6, 3, 5, 4]);
        assert_eq!(from_both_ends(0, 0, 1), Vec::<i64>::new());
        assert_eq!(
            StepRange::new(3, 40, 7).collect::<Vec<_>>(),
            (3..40).step_by(7).collect::<Vec<i64>>()
        );
    }

    #[test]
    #[should_panic(expected = "step must be positive")]
    fn zero_step_panics() {
        let _ = StepRange::new(0, 10, 0);
    }
}
