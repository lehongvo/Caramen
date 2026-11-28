// ==================================================================
// PRACTICE 0575: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0575_step_range_iterator.md
// Run the tests:          cargo test practice::p0575
// ==================================================================

//! # Problem 0575: A Generic Step Range
//!
//! - **Difficulty:** Easy
//! - **Topics:** a generic iterator `StepRange<T>`, `impl<T> Iterator for StepRange<T> where T: Copy + PartialOrd + Default + Add + Sub`, avoiding overflow generically
//! - **Source:** Original · [std::iter::Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
//!
//! ## Problem
//!
//! `(start..end).step_by(k)` only works for integer ranges. Write a generic version that also works for
//! floats: `StepRange<T>` yields `start, start + step, start + 2*step, ...` while the value is `< end`.
//!
//! - `StepRange::new(start: T, end: T, step: T) -> Self` panics with `"step must be positive"` unless
//!   `step > T::default()`. (There is no "zero" trait in std, but every number type's `Default` is 0.)
//! - `impl<T> Iterator for StepRange<T> where T: Copy + PartialOrd + Default + Add<Output = T> +
//!   Sub<Output = T>`
//!   with `type Item = T`.
//!
//! The iterator must **never overflow**: `StepRange::new(250_u8, 255, 3)` yields `250, 253` and stops,
//! even though `253 + 3` does not fit in a `u8`. To achieve that, compute the next value only when it
//! will be `< end`: that is the case exactly when `end - current > step`.
//!
//! The struct stores `next: Option<T>` (the value to yield next, `None` when finished), `end` and
//! `step`.
//!
//! ## Examples
//!
//! ```text
//! StepRange::new(0, 10, 3).collect::<Vec<_>>()        => [0, 3, 6, 9]
//! StepRange::new(0.0, 1.0, 0.25).collect::<Vec<_>>()  => [0.0, 0.25, 0.5, 0.75]
//! StepRange::new(250_u8, 255, 3).collect::<Vec<_>>()  => [250, 253]
//! StepRange::new(5, 5, 1).count()                     => 0
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. In `new`, the first value is `Some(start)` only if `start < end`.
//! 2. In `next`, take the current value out of `self.next` and decide whether there is a following one.

use std::ops::{Add, Sub};

/// Values `start, start + step, ...` below `end`.
#[derive(Debug, Clone)]
pub struct StepRange<T> {
    next: Option<T>,
    end: T,
    step: T,
}

impl<T> StepRange<T>
where
    T: Copy + PartialOrd + Default + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(start: T, end: T, step: T) -> Self {
        todo!("Write your solution here")
    }
}

impl<T> Iterator for StepRange<T>
where
    T: Copy + PartialOrd + Default + Add<Output = T> + Sub<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_ranges() {
        assert_eq!(
            StepRange::new(0, 10, 3).collect::<Vec<_>>(),
            vec![0, 3, 6, 9]
        );
        assert_eq!(StepRange::new(0, 9, 3).collect::<Vec<_>>(), vec![0, 3, 6]);
        assert_eq!(
            StepRange::new(-5_i64, 0, 2).collect::<Vec<_>>(),
            vec![-5, -3, -1]
        );
        assert_eq!(StepRange::new(5, 5, 1).count(), 0);
        assert_eq!(StepRange::new(7, 3, 1).count(), 0);
    }

    #[test]
    fn float_ranges() {
        assert_eq!(
            StepRange::new(0.0, 1.0, 0.25).collect::<Vec<_>>(),
            vec![0.0, 0.25, 0.5, 0.75]
        );
        assert_eq!(
            StepRange::new(1.5_f32, 2.0, 1.0).collect::<Vec<_>>(),
            vec![1.5]
        );
    }

    #[test]
    fn never_overflows() {
        assert_eq!(
            StepRange::new(250_u8, 255, 3).collect::<Vec<_>>(),
            vec![250, 253]
        );
        assert_eq!(
            StepRange::new(0_u8, 255, 100).collect::<Vec<_>>(),
            vec![0, 100, 200]
        );
        assert_eq!(
            StepRange::new(u64::MAX - 1, u64::MAX, u64::MAX / 2).count(),
            1
        );
    }

    #[test]
    #[should_panic(expected = "step must be positive")]
    fn zero_step_panics() {
        let _ = StepRange::new(0, 10, 0);
    }

    #[test]
    fn matches_std_step_by() {
        for (start, end, step) in [(0_u32, 1000, 7), (3, 4, 10), (10, 1_000_000, 999)] {
            let ours: Vec<u32> = StepRange::new(start, end, step).collect();
            let std: Vec<u32> = (start..end).step_by(step as usize).collect();
            assert_eq!(ours, std);
        }
    }
}
