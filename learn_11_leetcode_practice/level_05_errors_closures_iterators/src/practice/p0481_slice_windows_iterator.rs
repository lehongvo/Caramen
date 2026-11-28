// ==================================================================
// PRACTICE 0481: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0481_slice_windows_iterator.md
// Run the tests:          cargo test practice::p0481
// ==================================================================

//! # Problem 0481: A Sliding Window Iterator over a Slice
//!
//! - **Difficulty:** Medium
//! - **Topics:** iterator struct with a lifetime `Windows<'a, T>`, `type Item = &'a [T]`, `slice::get` with a range, `size_hint`, `ExactSizeIterator`
//! - **Source:** Original · [std `Iterator::size_hint`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.size_hint) · [std `ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html)
//!
//! ## Problem
//!
//! The standard library's `slice::windows(k)` yields every run of `k` consecutive elements. Write
//! your own version to see how an iterator can hand out **borrowed** pieces of data it does not own.
//!
//! `Windows<'a, T>` borrows a slice (`slice: &'a [T]`). Implement:
//!
//! - `Windows::new(slice: &'a [T], size: usize) -> Windows<'a, T>`. A window size of `0` makes no
//!   sense: panic with `"window size must be at least 1"`.
//! - `impl Iterator for Windows<'a, T>` with `type Item = &'a [T]`: the windows from left to right.
//!   A slice shorter than `size` has no windows.
//! - `size_hint` returning the exact number of windows that are still to come, as `(n, Some(n))`.
//!   (`ExactSizeIterator` is then implemented with an empty `impl`, which gives `len()`.)
//! - `moving_averages(values: &[f64], size: usize) -> Vec<f64>`: the average of every window.
//! - `max_window_sum(values: &[i64], size: usize) -> Option<i64>`: the largest window sum, `None` if
//!   there is no window.
//!
//! ## Examples
//!
//! ```text
//! Windows::new(&[1, 2, 3, 4], 2) yields [1, 2], [2, 3], [3, 4]
//! Windows::new(&[1, 2, 3, 4], 2).len()     -> 3
//! Windows::new(&[1, 2], 5) yields nothing
//! moving_averages(&[1.0, 3.0, 5.0, 7.0], 2) -> [2.0, 4.0, 6.0]
//! max_window_sum(&[4, -1, 2, 7, -5], 3)     -> Some(8)     (-1 + 2 + 7)
//! ```
//!
//! ## Constraints
//!
//! - Slices have at most `10^5` elements.
//!
//! ## Hints
//!
//! 1. Store the start position of the next window. `self.slice.get(start..start + size)` returns
//!    `None` once the window would run past the end, so `?` ends the iteration.
//! 2. The number of windows left is `len + 1 - (start + size)`, but never negative:
//!    `saturating_sub` handles that.

pub struct Windows<'a, T> {
    slice: &'a [T],
}

impl<'a, T> Windows<'a, T> {
    pub fn new(slice: &'a [T], size: usize) -> Windows<'a, T> {
        todo!("Write your solution here")
    }
}

impl<'a, T> Iterator for Windows<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Write your solution here")
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        todo!("Write your solution here")
    }
}

impl<T> ExactSizeIterator for Windows<'_, T> {}

pub fn moving_averages(values: &[f64], size: usize) -> Vec<f64> {
    todo!("Write your solution here")
}

pub fn max_window_sum(values: &[i64], size: usize) -> Option<i64> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yields_every_window() {
        let data = [1, 2, 3, 4];
        let windows: Vec<&[i32]> = Windows::new(&data, 2).collect();
        assert_eq!(windows, vec![&[1, 2][..], &[2, 3], &[3, 4]]);
        assert_eq!(Windows::new(&data, 4).count(), 1);
        assert_eq!(Windows::new(&data, 5).count(), 0);
        assert_eq!(Windows::new(&data, 1).count(), 4);
    }

    #[test]
    fn exact_size() {
        let data: Vec<u8> = (0..10).collect();
        let mut w = Windows::new(&data, 3);
        assert_eq!(w.len(), 8);
        w.next();
        assert_eq!(w.size_hint(), (7, Some(7)));
        assert_eq!(Windows::new(&data, 20).len(), 0);
        let empty: [u8; 0] = [];
        assert_eq!(Windows::new(&empty, 1).len(), 0);
    }

    #[test]
    fn windows_outlive_the_iterator() {
        let data = vec!["a", "b", "c"];
        let first = {
            let mut it = Windows::new(&data, 2);
            it.next()
        };
        assert_eq!(first, Some(&["a", "b"][..]));
    }

    #[test]
    #[should_panic(expected = "window size must be at least 1")]
    fn zero_size_panics() {
        let _ = Windows::new(&[1, 2, 3], 0);
    }

    #[test]
    fn helpers() {
        assert_eq!(
            moving_averages(&[1.0, 3.0, 5.0, 7.0], 2),
            vec![2.0, 4.0, 6.0]
        );
        assert_eq!(max_window_sum(&[4, -1, 2, 7, -5], 3), Some(8));
        assert_eq!(max_window_sum(&[1, 2], 3), None);
        let big: Vec<i64> = (1..=100_000).collect();
        assert_eq!(max_window_sum(&big, 2), Some(199_999));
        assert_eq!(
            Windows::new(&big, 2)
                .zip(big.windows(2))
                .filter(|(a, b)| a == b)
                .count(),
            99_999
        );
    }
}
