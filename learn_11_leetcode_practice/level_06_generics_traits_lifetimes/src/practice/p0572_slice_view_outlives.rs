// ==================================================================
// PRACTICE 0572: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0572_slice_view_outlives.md
// Run the tests:          cargo test practice::p0572
// ==================================================================

//! # Problem 0572: Slice Views and `T: 'a`
//!
//! - **Difficulty:** Medium
//! - **Topics:** `struct View<'a, T: 'a>`, outlives bounds, returning `View<'a, T>` and `&'a T` from methods, manual `Clone`/`Copy` without `T: Clone`
//! - **Source:** Original · [The Rust Reference: lifetime bounds](https://doc.rust-lang.org/reference/trait-bounds.html#lifetime-bounds)
//!
//! ## Problem
//!
//! `View<'a, T>` is a read-only window into a slice: `struct View<'a, T: 'a> { data: &'a [T] }`. The
//! bound `T: 'a` says "every reference inside `T` lives at least as long as `'a`", which is required for
//! `&'a [T]` to make sense. (Since Rust 2018 the compiler infers this bound, but writing it out shows
//! what is going on.)
//!
//! Implement, in `impl<'a, T> View<'a, T>`:
//!
//! - `new(data: &'a [T]) -> Self`, `len(&self)`, `is_empty(&self)`, `as_slice(&self) -> &'a [T]`.
//! - `get(&self, i: usize) -> Option<&'a T>`.
//! - `split_at(&self, mid: usize) -> (View<'a, T>, View<'a, T>)`: like `slice::split_at`, but `mid` is
//!   clamped to `len` instead of panicking.
//! - `chunks(&self, size: usize) -> Vec<View<'a, T>>`: consecutive views of `size` elements (the last
//!   one may be shorter). Panics with `"chunk size must be positive"` when `size == 0`.
//!
//! In `impl<'a, T: PartialOrd> View<'a, T>`:
//!
//! - `longest_increasing_run(&self) -> View<'a, T>`: the longest run of consecutive, **strictly**
//!   increasing elements (the first one on ties); an empty view for empty data.
//!
//! Also implement `Clone` and `Copy` for `View<'a, T>` **by hand**, for every `T`: a view is just a
//! reference, so copying it never requires copying a `T` (`#[derive]` would demand `T: Clone`).
//!
//! ## Examples
//!
//! ```text
//! let data = [1, 2, 0, 3, 4, 5, 1];
//! let v = View::new(&data);
//! v.get(3)                               => Some(&3)
//! v.split_at(2)                          => ([1, 2], [0, 3, 4, 5, 1])
//! v.split_at(99)                         => ([1, 2, 0, 3, 4, 5, 1], [])
//! v.chunks(3)                            => [[1, 2, 0], [3, 4, 5], [1]]
//! v.longest_increasing_run().as_slice()  => [0, 3, 4, 5]
//! ```
//!
//! ## Constraints
//!
//! - Up to `10^5` elements.
//!
//! ## Hints
//!
//! 1. Every method returns pieces of `self.data`, which has type `&'a [T]`, so the pieces naturally have
//!    lifetime `'a`, independent of the `&self` borrow.
//! 2. `impl<T> Clone for View<'_, T> { fn clone(&self) -> Self { *self } }` plus `impl<T> Copy for
//!    View<'_, T> {}`.

/// A read-only window into a slice.
#[derive(Debug, PartialEq)]
pub struct View<'a, T: 'a> {
    data: &'a [T],
}

impl<T> Clone for View<'_, T> {
    fn clone(&self) -> Self {
        todo!("Write your solution here")
    }
}

impl<T> Copy for View<'_, T> {}

impl<'a, T> View<'a, T> {
    pub fn new(data: &'a [T]) -> Self {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn as_slice(&self) -> &'a [T] {
        todo!("Write your solution here")
    }

    pub fn get(&self, i: usize) -> Option<&'a T> {
        todo!("Write your solution here")
    }

    pub fn split_at(&self, mid: usize) -> (View<'a, T>, View<'a, T>) {
        todo!("Write your solution here")
    }

    pub fn chunks(&self, size: usize) -> Vec<View<'a, T>> {
        todo!("Write your solution here")
    }
}

impl<'a, T: PartialOrd> View<'a, T> {
    pub fn longest_increasing_run(&self) -> View<'a, T> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_accessors() {
        let data = [1, 2, 0, 3, 4, 5, 1];
        let v = View::new(&data);
        assert_eq!(v.len(), 7);
        assert!(!v.is_empty());
        assert_eq!(v.get(3), Some(&3));
        assert_eq!(v.get(7), None);
        assert_eq!(v.as_slice(), &data);
    }

    #[test]
    fn splitting_and_chunking() {
        let data = [1, 2, 0, 3, 4, 5, 1];
        let v = View::new(&data);
        let (l, r) = v.split_at(2);
        assert_eq!(
            (l.as_slice(), r.as_slice()),
            (&[1, 2][..], &[0, 3, 4, 5, 1][..])
        );
        let (all, none) = v.split_at(99);
        assert_eq!(all.len(), 7);
        assert!(none.is_empty());
        let chunks: Vec<&[i32]> = v.chunks(3).iter().map(View::as_slice).collect();
        assert_eq!(chunks, vec![&[1, 2, 0][..], &[3, 4, 5][..], &[1][..]]);
    }

    #[test]
    fn increasing_runs() {
        let data = [1, 2, 0, 3, 4, 5, 1];
        assert_eq!(
            View::new(&data).longest_increasing_run().as_slice(),
            &[0, 3, 4, 5]
        );
        assert_eq!(
            View::new(&[5, 5, 5]).longest_increasing_run().as_slice(),
            &[5]
        );
        let empty: [u8; 0] = [];
        assert!(View::new(&empty).longest_increasing_run().is_empty());
        assert_eq!(
            View::new(&[1, 2, 9, 3, 4])
                .longest_increasing_run()
                .as_slice(),
            &[1, 2, 9]
        );
    }

    #[test]
    fn views_of_non_copy_values_are_copy() {
        let words = vec!["b".to_string(), "c".to_string(), "a".to_string()];
        let v = View::new(&words);
        let w = v; // Copy: `v` is still usable
        assert_eq!(v.len(), w.len());
        let run = v.longest_increasing_run();
        assert_eq!(run.as_slice(), &words[..2]);
        #[allow(clippy::clone_on_copy)]
        let cloned = run.clone();
        assert_eq!(cloned, run);
    }

    #[test]
    fn results_outlive_the_view_and_borrowed_elements() {
        let text = String::from("a b c d");
        let words: Vec<&str> = text.split(' ').collect(); // View<'_, &str>: T itself is a reference
        let first_half;
        {
            let v = View::new(&words);
            first_half = v.split_at(2).0;
        }
        assert_eq!(first_half.as_slice(), &["a", "b"]);
        let big: Vec<u32> = (0..100_000).map(|i| i % 1000).collect();
        assert_eq!(View::new(&big).longest_increasing_run().len(), 1000);
    }

    #[test]
    #[should_panic(expected = "chunk size must be positive")]
    fn zero_chunk_size_panics() {
        let data = [1, 2, 3];
        let _ = View::new(&data).chunks(0);
    }
}
