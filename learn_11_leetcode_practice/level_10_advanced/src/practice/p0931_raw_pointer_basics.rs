// ==================================================================
// PRACTICE 0931: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0931_raw_pointer_basics.md
// Run the tests:          cargo test practice::p0931
// ==================================================================

//! # Problem 0931: Raw Pointers and `unsafe fn`
//!
//! - **Difficulty:** Medium
//! - **Topics:** `*const T` / `*mut T`, `unsafe` blocks, `unsafe fn` with a `# Safety` contract, `ptr.add`, `offset_from`, `ptr::swap`, safe wrappers
//! - **Source:** Original · [The Rust Book 20.1: Unsafe Rust](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html) · [std: `ptr`](https://doc.rust-lang.org/std/ptr/index.html)
//!
//! ## Problem
//!
//! Creating a raw pointer is safe; **dereferencing** it is not, because the compiler can no longer
//! check that it points to valid memory. Unsafe code is written in small pieces whose safety
//! conditions are stated and checked by hand, wrapped in safe functions that guarantee those
//! conditions for every caller.
//!
//! Implement:
//!
//! 1. `swap_via_raw(a: &mut i32, b: &mut i32)`: turn both references into `*mut i32` and swap the
//!    values by reading and writing through the pointers.
//! 2. `unsafe fn sum_raw(ptr: *const i32, len: usize) -> i64`, documented with a `# Safety`
//!    section: the caller promises that `ptr` is valid for reading `len` consecutive `i32`s. Read
//!    them with `ptr.add(i)`.
//! 3. `sum_slice(values: &[i32]) -> i64`: a **safe** wrapper that calls `sum_raw` (a slice always
//!    satisfies the contract).
//! 4. `unsafe fn get_unchecked_ref<T>(slice: &[T], index: usize) -> &T` (`# Safety`:
//!    `index < slice.len()`) and the safe `checked_get<T>(slice: &[T], index: usize) -> Option<&T>`
//!    built on it.
//! 5. `distance<T>(slice: &[T], i: usize, j: usize) -> isize`: the signed number of elements from
//!    position `i` to position `j`, computed with `offset_from` on pointers into the slice. Positions
//!    may be anything in `0..=len` (one past the end is allowed); otherwise panic with
//!    `"position out of range"`.
//! 6. `reverse_raw(values: &mut [u8])`: reverse the slice by walking a front pointer and a back
//!    pointer towards each other and swapping with `std::ptr::swap`.
//!
//! In edition 2024 the body of an `unsafe fn` is **not** automatically an unsafe block
//! (`unsafe_op_in_unsafe_fn` warns): write explicit `unsafe { ... }` blocks inside it too.
//!
//! ## Examples
//!
//! ```text
//! let (mut a, mut b) = (1, 2); swap_via_raw(&mut a, &mut b);   // a == 2, b == 1
//! sum_slice(&[1, 2, 3])                   => 6
//! checked_get(&[10, 20], 1)               => Some(&20)
//! checked_get(&[10, 20], 2)               => None
//! distance(&[0u64; 5], 4, 1)              => -3
//! reverse_raw(b"hello") gives b"olleh"
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `let pa: *mut i32 = a;` coerces a `&mut i32` to a raw pointer without any `unsafe`.
//! 2. `ptr.add(i)` is itself `unsafe`: the result must stay inside (or one past) the same allocation.
//! 3. For `reverse_raw`, loop `while front < back` (raw pointers can be compared).

use std::ptr;

pub fn swap_via_raw(a: &mut i32, b: &mut i32) {
    todo!("Write your solution here")
}

/// Sums `len` consecutive `i32`s starting at `ptr`.
///
/// # Safety
///
/// `ptr` must be valid for reading `len` consecutive, initialised `i32` values.
pub unsafe fn sum_raw(ptr: *const i32, len: usize) -> i64 {
    todo!("Write your solution here")
}

pub fn sum_slice(values: &[i32]) -> i64 {
    todo!("Write your solution here")
}

/// Returns `&slice[index]` without a bounds check.
///
/// # Safety
///
/// `index` must be smaller than `slice.len()`.
pub unsafe fn get_unchecked_ref<T>(slice: &[T], index: usize) -> &T {
    todo!("Write your solution here")
}

pub fn checked_get<T>(slice: &[T], index: usize) -> Option<&T> {
    todo!("Write your solution here")
}

pub fn distance<T>(slice: &[T], i: usize, j: usize) -> isize {
    todo!("Write your solution here")
}

pub fn reverse_raw(values: &mut [u8]) {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_through_pointers() {
        let (mut a, mut b) = (1, 2);
        swap_via_raw(&mut a, &mut b);
        assert_eq!((a, b), (2, 1));
        let mut v = [7, -7];
        let (x, y) = v.split_at_mut(1);
        swap_via_raw(&mut x[0], &mut y[0]);
        assert_eq!(v, [-7, 7]);
    }

    #[test]
    fn sums() {
        assert_eq!(sum_slice(&[1, 2, 3]), 6);
        assert_eq!(sum_slice(&[]), 0);
        assert_eq!(sum_slice(&[i32::MAX, i32::MAX]), 4_294_967_294);
        let data = [5, 6, 7, 8];
        // SAFETY: `data.as_ptr().add(1)` is valid for 3 reads (elements 1, 2, 3).
        let tail = unsafe { sum_raw(data.as_ptr().add(1), 3) };
        assert_eq!(tail, 21);
    }

    #[test]
    fn checked_and_unchecked_access() {
        let words = ["a", "b", "c"];
        assert_eq!(checked_get(&words, 1), Some(&"b"));
        assert_eq!(checked_get(&words, 3), None);
        assert_eq!(checked_get::<u8>(&[], 0), None);
        // SAFETY: 2 < 3.
        assert_eq!(unsafe { get_unchecked_ref(&words, 2) }, &"c");
    }

    #[test]
    fn distances() {
        let data = [0_u64; 5];
        assert_eq!(distance(&data, 4, 1), -3);
        assert_eq!(distance(&data, 0, 5), 5);
        assert_eq!(distance(&data, 2, 2), 0);
        let strings: Vec<String> = (0..3).map(|i| i.to_string()).collect();
        assert_eq!(distance(&strings, 0, 3), 3);
    }

    #[test]
    #[should_panic(expected = "position out of range")]
    fn distance_checks_positions() {
        distance(&[1, 2, 3], 0, 4);
    }

    #[test]
    fn reversing() {
        let mut word = *b"hello";
        reverse_raw(&mut word);
        assert_eq!(&word, b"olleh");
        let mut even = *b"abcd";
        reverse_raw(&mut even);
        assert_eq!(&even, b"dcba");
        let mut one = [9];
        reverse_raw(&mut one);
        assert_eq!(one, [9]);
        let mut empty: [u8; 0] = [];
        reverse_raw(&mut empty);
        let mut big: Vec<u8> = (0..=255).cycle().take(100_001).collect();
        let expected: Vec<u8> = big.iter().rev().copied().collect();
        reverse_raw(&mut big);
        assert_eq!(big, expected);
    }
}
