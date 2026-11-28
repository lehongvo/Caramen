// ==================================================================
// PRACTICE 0879: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0879_parallel_in_place_update.md
// Run the tests:          cargo test practice::p0879
// ==================================================================

//! # Problem 0879: Parallel In-Place Update
//!
//! - **Difficulty:** Medium
//! - **Topics:** `chunks_mut` to hand out disjoint `&mut` sub-slices, scoped threads, closure bounds
//!   `Fn + Sync`, a read phase followed by a write phase
//! - **Source:** Original · [`thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html) ·
//!   [slices](https://doc.rust-lang.org/std/primitive.slice.html)
//!
//! ## Problem
//!
//! The borrow checker forbids two threads from holding `&mut` to the same data, but it happily
//! accepts **disjoint** mutable borrows. `chunks_mut` splits one `&mut [T]` into non-overlapping
//! `&mut [T]` pieces that can go to different threads.
//!
//! 1. `parallel_apply<F>(data: &mut [i64], parts: usize, f: F) where F: Fn(i64) -> i64 + Sync`:
//!    replace every element `x` with `f(x)`, using one scoped thread per chunk of
//!    `ceil(len / parts)` elements.
//! 2. `normalize_to_percent(data: &mut [u32], parts: usize)`: first find the maximum of `data` in
//!    parallel (phase 1, shared borrows), then replace every `x` with `x * 100 / max` in parallel
//!    (phase 2, mutable borrows). If the slice is empty or the maximum is `0`, leave it unchanged.
//!
//! ## Examples
//!
//! ```text
//! data = [1, 2, 3, 4, 5]; parallel_apply(&mut data, 2, |x| x * 10)  -> data = [10, 20, 30, 40, 50]
//! data = [5, 10, 20];     normalize_to_percent(&mut data, 2)        -> data = [25, 50, 100]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= parts <= 64`; `0 <= data.len() <= 10^6`; in `normalize_to_percent`, `x <= 10^7`.
//!
//! ## Hints
//!
//! 1. `data.chunks_mut(size)` inside `thread::scope`; each spawned closure gets one `&mut [i64]`.
//! 2. All threads call the same `f` through a shared reference `&f`. Sharing a reference between
//!    threads requires `F: Sync`.
//! 3. The two phases of `normalize_to_percent` are two separate scopes: the shared borrows of
//!    phase 1 must end before the mutable borrows of phase 2 begin.

use std::thread;

pub fn parallel_apply<F>(data: &mut [i64], parts: usize, f: F)
where
    F: Fn(i64) -> i64 + Sync,
{
    todo!("Write your solution here")
}

pub fn normalize_to_percent(data: &mut [u32], parts: usize) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_times_ten() {
        let mut data = vec![1, 2, 3, 4, 5];
        parallel_apply(&mut data, 2, |x| x * 10);
        assert_eq!(data, vec![10, 20, 30, 40, 50]);
    }

    #[test]
    fn example_normalize() {
        let mut data = vec![5, 10, 20];
        normalize_to_percent(&mut data, 2);
        assert_eq!(data, vec![25, 50, 100]);
    }

    #[test]
    fn closure_capturing_local_state() {
        let offset = 7;
        let mut data = vec![0, 1, 2];
        parallel_apply(&mut data, 64, |x| x + offset);
        assert_eq!(data, vec![7, 8, 9]);
    }

    #[test]
    fn empty_and_all_zero_inputs() {
        let mut empty: Vec<i64> = Vec::new();
        parallel_apply(&mut empty, 3, |x| x + 1);
        assert!(empty.is_empty());
        let mut zeros = vec![0_u32; 5];
        normalize_to_percent(&mut zeros, 2);
        assert_eq!(zeros, vec![0; 5]);
    }

    #[test]
    fn large_slices() {
        let mut data: Vec<i64> = (0..1_000_000).collect();
        parallel_apply(&mut data, 8, |x| x * x - 1);
        assert!(
            data.iter()
                .enumerate()
                .all(|(i, &x)| x == (i as i64) * (i as i64) - 1)
        );
        let mut values: Vec<u32> = (0..=10_000_000).step_by(10).collect();
        normalize_to_percent(&mut values, 7);
        assert_eq!(values[0], 0);
        assert_eq!(values[values.len() / 2], 50);
        assert_eq!(*values.last().unwrap(), 100);
    }
}
