//! # Problem 0934: Unsafe Traits: `Zeroable`
//!
//! - **Difficulty:** Hard
//! - **Topics:** `unsafe trait` and `unsafe impl`, `std::mem::zeroed`, `#[repr(C)]`, invariants that generic safe code relies on, `Option<Box<T>>` niche
//! - **Source:** Original · [The Rust Book 20.1: Implementing an Unsafe Trait](https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#implementing-an-unsafe-trait) · [std: `mem::zeroed`](https://doc.rust-lang.org/std/mem/fn.zeroed.html)
//!
//! ## Problem
//!
//! Some types are valid when all their bytes are zero (`u32` is `0`, `bool` is `false`, an
//! `Option<Box<T>>` is `None`); others are not (a `&T` or a `Box<T>` must never be null, a
//! `NonZeroU8` is never zero). A **safe** generic function cannot check this, so the property is
//! promised by an `unsafe trait`: implementing it is `unsafe`, and code may then rely on it.
//!
//! ```text
//! /// # Safety
//! /// The all-zero bit pattern must be a valid value of `Self`.
//! pub unsafe trait Zeroable: Sized {
//!     fn zeroed() -> Self { ... }   // provided method: write it
//! }
//! ```
//!
//! The implementations are given: `u8`, `u32`, `i64`, `f64`, `bool`, `char`, `Option<Box<T>>`,
//! arrays `[T; N]` of zeroable `T`, and the `#[repr(C)]` struct `Pixel { r: u8, g: u8, b: u8, a: u8 }`.
//!
//! Implement:
//!
//! 1. The provided method `Zeroable::zeroed()`, with `std::mem::zeroed`.
//! 2. `zeroed_vec<T: Zeroable>(n: usize) -> Vec<T>`: `n` zeroed values.
//! 3. `reset<T: Zeroable>(value: &mut T)`: overwrite `value` with a zeroed value (the old value is
//!    dropped normally).
//! 4. `reset_all<T: Zeroable>(values: &mut [T]) -> usize`: reset every element; return how many
//!    elements were **not** already zero before, where "zero" means equal to `T::zeroed()`
//!    (requires `T: PartialEq`).
//!
//! ## Examples
//!
//! ```text
//! u32::zeroed()                    => 0
//! bool::zeroed()                   => false
//! <Option<Box<String>>>::zeroed()  => None
//! <[char; 3]>::zeroed()            => ['\0', '\0', '\0']
//! zeroed_vec::<Pixel>(2)           => [Pixel { r: 0, g: 0, b: 0, a: 0 }; 2]
//! reset_all(&mut [3, 0, 5])        => 2, values now [0, 0, 0]
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `std::mem::zeroed::<T>()` is `unsafe`: it is only sound for types where zero is valid, which
//!    is exactly what the trait's contract says. Write a `// SAFETY:` comment pointing to it.
//! 2. `reset` can simply assign `*value = T::zeroed();`.
//! 3. `(0..n).map(|_| T::zeroed()).collect()` builds the vector.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the only unsafe operation is in the provided method `zeroed`, and its soundness follows
//! from the trait's contract: every implementor promised that zero bytes form a valid value. All
//! the generic functions are then completely safe code built on `T::zeroed()`.
//!
//! **Rust concepts:**
//! - `unsafe trait` means "implementing this trait carries a proof obligation"; `unsafe impl` is
//!   where the implementor accepts it. `Send` and `Sync` are std's best-known unsafe traits.
//! - Using the trait is safe: `T::zeroed()` needs no `unsafe` block at the call site.
//! - `Option<Box<T>>` is guaranteed to use the null pointer for `None`, so zero is valid; `Box<T>`
//!   alone would be undefined behaviour when zeroed.
//! - `#[repr(C)]` fixes the layout of `Pixel`; with four `u8` fields every byte pattern is valid.
// @solution-end

/// # Safety
///
/// The all-zero bit pattern must be a valid value of `Self`.
pub unsafe trait Zeroable: Sized {
    fn zeroed() -> Self {
        // @solution-start
        // SAFETY: implementing `Zeroable` promises that all-zero bytes are a valid `Self`.
        unsafe { std::mem::zeroed() }
        // @solution-end
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// SAFETY (all impls below): zero is `0` for integers, `0.0` for floats, `false` for `bool`,
// `'\0'` for `char`, `None` for `Option<Box<T>>` (guaranteed null-pointer niche), and every field
// of `Pixel` and every element of a zeroable array is itself zeroable.
unsafe impl Zeroable for u8 {}
unsafe impl Zeroable for u32 {}
unsafe impl Zeroable for i64 {}
unsafe impl Zeroable for f64 {}
unsafe impl Zeroable for bool {}
unsafe impl Zeroable for char {}
unsafe impl<T> Zeroable for Option<Box<T>> {}
unsafe impl<T: Zeroable, const N: usize> Zeroable for [T; N] {}
unsafe impl Zeroable for Pixel {}

pub fn zeroed_vec<T: Zeroable>(n: usize) -> Vec<T> {
    // @solution-start
    (0..n).map(|_| T::zeroed()).collect()
    // @solution-end
}

pub fn reset<T: Zeroable>(value: &mut T) {
    // @solution-start
    *value = T::zeroed();
    // @solution-end
}

pub fn reset_all<T: Zeroable + PartialEq>(values: &mut [T]) -> usize {
    // @solution-start
    let zero = T::zeroed();
    let mut changed = 0;
    for value in values.iter_mut() {
        if *value != zero {
            changed += 1;
        }
        reset(value);
    }
    changed
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_zeros() {
        assert_eq!(u32::zeroed(), 0);
        assert_eq!(i64::zeroed(), 0);
        assert_eq!(f64::zeroed(), 0.0);
        assert!(!bool::zeroed());
        assert_eq!(char::zeroed(), '\0');
    }

    #[test]
    fn compound_zeros() {
        assert!(<Option<Box<String>>>::zeroed().is_none());
        assert_eq!(<[char; 3]>::zeroed(), ['\0'; 3]);
        assert_eq!(<[[u8; 2]; 2]>::zeroed(), [[0, 0], [0, 0]]);
        assert_eq!(
            zeroed_vec::<Pixel>(2),
            vec![
                Pixel {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0
                };
                2
            ]
        );
        assert!(zeroed_vec::<u8>(0).is_empty());
    }

    #[test]
    fn reset_drops_the_old_value() {
        let mut slot: Option<Box<String>> = Some(Box::new("heap".to_string()));
        reset(&mut slot);
        assert!(slot.is_none());
        let mut pixel = Pixel {
            r: 255,
            g: 128,
            b: 0,
            a: 1,
        };
        reset(&mut pixel);
        assert_eq!(pixel, Pixel::zeroed());
    }

    #[test]
    fn reset_all_counts_non_zero_values() {
        let mut values = [3_u32, 0, 5];
        assert_eq!(reset_all(&mut values), 2);
        assert_eq!(values, [0, 0, 0]);
        assert_eq!(reset_all(&mut values), 0);
        let mut flags = vec![true; 1_000];
        flags[10] = false;
        assert_eq!(reset_all(&mut flags), 999);
        assert!(flags.iter().all(|&f| !f));
        let mut boxes = vec![Some(Box::new(1_i64)), None, Some(Box::new(2))];
        assert_eq!(reset_all(&mut boxes), 2);
    }
}
