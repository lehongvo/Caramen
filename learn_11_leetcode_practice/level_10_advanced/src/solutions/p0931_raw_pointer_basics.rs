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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each unsafe operation sits in its own `unsafe` block with a `// SAFETY:` comment that
//! explains why the contract holds. The safe wrappers establish the contract first: a slice gives a
//! valid pointer and length, `checked_get` compares the index with the length, `distance` checks
//! both positions, and `reverse_raw` only dereferences pointers strictly inside the slice.
//!
//! **Complexity:** `sum_*` and `reverse_raw` are `O(n)`, the rest `O(1)`.
//!
//! **Rust concepts:**
//! - Raw pointers may be null, dangling or unaligned, and are not tracked by the borrow checker;
//!   that is why dereferencing them, `add` and `offset_from` require `unsafe`.
//! - An `unsafe fn` moves the proof obligation to the caller; the `# Safety` section (required by
//!   Clippy's `missing_safety_doc` lint for public functions) says what must be proven.
//! - A safe function containing `unsafe` blocks must be sound for **all** inputs: that is what makes
//!   it a safe abstraction.
// @solution-end

use std::ptr;

pub fn swap_via_raw(a: &mut i32, b: &mut i32) {
    // @solution-start
    let pa: *mut i32 = a;
    let pb: *mut i32 = b;
    // SAFETY: both pointers come from live `&mut` references, so they are valid, aligned and
    // cannot alias each other.
    unsafe {
        let tmp = ptr::read(pa);
        ptr::write(pa, ptr::read(pb));
        ptr::write(pb, tmp);
    }
    // @solution-end
}

/// Sums `len` consecutive `i32`s starting at `ptr`.
///
/// # Safety
///
/// `ptr` must be valid for reading `len` consecutive, initialised `i32` values.
pub unsafe fn sum_raw(ptr: *const i32, len: usize) -> i64 {
    // @solution-start
    let mut total = 0;
    for i in 0..len {
        // SAFETY: the caller guarantees `ptr..ptr + len` is readable, and `i < len`.
        total += i64::from(unsafe { *ptr.add(i) });
    }
    total
    // @solution-end
}

pub fn sum_slice(values: &[i32]) -> i64 {
    // @solution-start
    // SAFETY: a slice's pointer is valid for reading exactly `len` elements.
    unsafe { sum_raw(values.as_ptr(), values.len()) }
    // @solution-end
}

/// Returns `&slice[index]` without a bounds check.
///
/// # Safety
///
/// `index` must be smaller than `slice.len()`.
pub unsafe fn get_unchecked_ref<T>(slice: &[T], index: usize) -> &T {
    // @solution-start
    // SAFETY: the caller guarantees `index < len`, so the pointer stays inside the slice.
    unsafe { &*slice.as_ptr().add(index) }
    // @solution-end
}

pub fn checked_get<T>(slice: &[T], index: usize) -> Option<&T> {
    // @solution-start
    if index < slice.len() {
        // SAFETY: we just checked `index < len`.
        Some(unsafe { get_unchecked_ref(slice, index) })
    } else {
        None
    }
    // @solution-end
}

pub fn distance<T>(slice: &[T], i: usize, j: usize) -> isize {
    // @solution-start
    assert!(
        i <= slice.len() && j <= slice.len(),
        "position out of range"
    );
    let base = slice.as_ptr();
    // SAFETY: `i` and `j` are at most `len`, so both pointers are inside the slice or one past its
    // end, i.e. in the same allocation, as `add` and `offset_from` require.
    unsafe { base.add(j).offset_from(base.add(i)) }
    // @solution-end
}

pub fn reverse_raw(values: &mut [u8]) {
    // @solution-start
    if values.len() < 2 {
        return;
    }
    let mut front = values.as_mut_ptr();
    // SAFETY: `len >= 2`, so `len - 1` is the index of the last element.
    let mut back = unsafe { front.add(values.len() - 1) };
    while front < back {
        // SAFETY: `front < back` and both stay within the slice: `front` only moves forward and
        // `back` only moves backward until they meet.
        unsafe {
            ptr::swap(front, back);
            front = front.add(1);
            back = back.sub(1);
        }
    }
    // @solution-end
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
