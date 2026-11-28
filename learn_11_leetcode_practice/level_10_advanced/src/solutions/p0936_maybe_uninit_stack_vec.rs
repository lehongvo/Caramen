//! # Problem 0936: A `StackVec` with `MaybeUninit`
//!
//! - **Difficulty:** Hard
//! - **Topics:** `std::mem::MaybeUninit`, `[const { MaybeUninit::uninit() }; N]`, `assume_init_read`, `ptr::drop_in_place`, `slice::from_raw_parts`, a correct `Drop`
//! - **Source:** Original · [std: `MaybeUninit`](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html) · [The Rustonomicon: Unchecked Uninitialized Memory](https://doc.rust-lang.org/nomicon/unchecked-uninit.html)
//!
//! ## Problem
//!
//! Problem 0917 stored `[Option<T>; N]`, paying for a tag in every slot. A real inline vector
//! (`arrayvec`, `smallvec`) stores **uninitialised** slots instead and keeps track itself of which
//! ones hold values:
//!
//! ```text
//! pub struct StackVec<T, const N: usize> { data: [MaybeUninit<T>; N], len: usize }
//! ```
//!
//! Invariant: slots `0..len` are initialised, slots `len..N` are not. Implement:
//!
//! - `new()` (and `Default`): no slot initialised.
//! - `push(&mut self, value: T) -> Result<(), T>`: `Err(value)` when full.
//! - `pop(&mut self) -> Option<T>`: moves the last value out.
//! - `len()`, `is_empty()`, `capacity()` (`N`).
//! - `as_slice(&self) -> &[T]` and `as_mut_slice(&mut self) -> &mut [T]`: the initialised part.
//! - `clear(&mut self)`: drops every value.
//! - `Drop`: drops the initialised values (and only those).
//!
//! `MaybeUninit<T>` never drops its content by itself, so forgetting the `Drop` impl leaks, and
//! dropping a slot twice (or an uninitialised one) is undefined behaviour.
//!
//! ## Examples
//!
//! ```text
//! let mut v: StackVec<String, 2> = StackVec::new();
//! v.push("a".into()) => Ok(())
//! v.push("b".into()) => Ok(())
//! v.push("c".into()) => Err("c")
//! v.as_slice()       => ["a", "b"]
//! v.pop()            => Some("b")
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `[const { MaybeUninit::uninit() }; N]` creates the array for any `T` (an inline `const` block
//!    may be repeated even though `MaybeUninit<T>` is not `Copy`).
//! 2. `self.data[i].write(value)` initialises a slot; `self.data[i].assume_init_read()` moves the
//!    value out (unsafe: the slot must be initialised, and it must then be treated as empty).
//! 3. `std::slice::from_raw_parts(self.data.as_ptr().cast::<T>(), self.len)` views the initialised
//!    prefix as `&[T]`; `MaybeUninit<T>` has the same layout as `T`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** every method keeps the invariant. `push` writes slot `len` then increments `len`;
//! `pop` decrements `len` then reads that slot out, so it is logically empty again. `clear` sets
//! `len = 0` **before** dropping the old prefix in place, so a panicking destructor cannot lead to a
//! double drop. `Drop` simply calls `clear`.
//!
//! **Complexity:** every operation is `O(1)` except `clear`/`drop`, which are `O(len)`.
//!
//! **Rust concepts:**
//! - `MaybeUninit<T>` is a `union`-like wrapper that may hold garbage; reading it is only sound
//!   once it has been written.
//! - `ptr::drop_in_place(slice)` runs the destructors of a whole slice of initialised values without
//!   moving them.
//! - `assume_init_read` performs a bitwise copy; the original slot must not be read or dropped
//!   again.
// @solution-end

use std::mem::MaybeUninit;
use std::ptr;

pub struct StackVec<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> Default for StackVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> StackVec<T, N> {
    pub fn new() -> Self {
        // @solution-start
        StackVec {
            data: [const { MaybeUninit::uninit() }; N],
            len: 0,
        }
        // @solution-end
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        // @solution-start
        if self.len == N {
            return Err(value);
        }
        self.data[self.len].write(value);
        self.len += 1;
        Ok(())
        // @solution-end
    }

    pub fn pop(&mut self) -> Option<T> {
        // @solution-start
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        // SAFETY: slot `len` was initialised (it was below the old length) and is now outside the
        // initialised prefix, so it will not be read or dropped again.
        Some(unsafe { self.data[self.len].assume_init_read() })
        // @solution-end
    }

    pub fn len(&self) -> usize {
        // @solution-start
        self.len
        // @solution-end
    }

    pub fn is_empty(&self) -> bool {
        // @solution-start
        self.len == 0
        // @solution-end
    }

    pub fn capacity(&self) -> usize {
        // @solution-start
        N
        // @solution-end
    }

    pub fn as_slice(&self) -> &[T] {
        // @solution-start
        // SAFETY: the first `len` slots are initialised and `MaybeUninit<T>` has `T`'s layout.
        unsafe { std::slice::from_raw_parts(self.data.as_ptr().cast::<T>(), self.len) }
        // @solution-end
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // @solution-start
        // SAFETY: as in `as_slice`, and `&mut self` guarantees unique access.
        unsafe { std::slice::from_raw_parts_mut(self.data.as_mut_ptr().cast::<T>(), self.len) }
        // @solution-end
    }

    pub fn clear(&mut self) {
        // @solution-start
        let initialised: *mut [T] = self.as_mut_slice();
        // Forget the values first: if a destructor panics, they are not dropped a second time.
        self.len = 0;
        // SAFETY: `initialised` covers exactly the slots that held values, and they are no longer
        // part of the vector, so each one is dropped exactly once.
        unsafe { ptr::drop_in_place(initialised) };
        // @solution-end
    }
}

impl<T, const N: usize> Drop for StackVec<T, N> {
    fn drop(&mut self) {
        // @solution-start replace: // TODO: drop the initialised values (never panic inside `drop`).
        self.clear();
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn push_pop_and_full() {
        let mut v: StackVec<String, 2> = StackVec::new();
        assert_eq!(v.push("a".to_string()), Ok(()));
        assert_eq!(v.push("b".to_string()), Ok(()));
        assert_eq!(v.push("c".to_string()), Err("c".to_string()));
        assert_eq!(v.as_slice(), ["a", "b"]);
        assert_eq!(v.pop(), Some("b".to_string()));
        assert_eq!((v.len(), v.capacity(), v.is_empty()), (1, 2, false));
        assert_eq!(v.pop(), Some("a".to_string()));
        assert_eq!(v.pop(), None);
        assert!(v.is_empty());
    }

    #[test]
    fn mutable_slice_view() {
        let mut v: StackVec<i32, 8> = StackVec::default();
        for i in 1..=5 {
            v.push(i).unwrap();
        }
        v.as_mut_slice().reverse();
        assert_eq!(v.as_slice(), [5, 4, 3, 2, 1]);
        v.as_mut_slice()[0] = 50;
        assert_eq!(v.as_slice().iter().sum::<i32>(), 60);
    }

    #[test]
    fn every_value_is_dropped_exactly_once() {
        let token = Rc::new(());
        {
            let mut v: StackVec<Rc<()>, 4> = StackVec::new();
            for _ in 0..4 {
                v.push(Rc::clone(&token)).unwrap();
            }
            assert_eq!(Rc::strong_count(&token), 5);
            // A rejected value is handed back, not dropped by the vector.
            let rejected = v.push(Rc::clone(&token)).unwrap_err();
            assert_eq!(Rc::strong_count(&token), 6);
            drop(rejected);
            drop(v.pop());
            assert_eq!(Rc::strong_count(&token), 4);
        }
        // Dropping the vector dropped the 3 values still inside.
        assert_eq!(Rc::strong_count(&token), 1);
    }

    #[test]
    fn clear_drops_and_allows_reuse() {
        let token = Rc::new(());
        let mut v: StackVec<Rc<()>, 3> = StackVec::new();
        v.push(Rc::clone(&token)).unwrap();
        v.push(Rc::clone(&token)).unwrap();
        v.clear();
        assert_eq!(Rc::strong_count(&token), 1);
        assert!(v.is_empty());
        v.push(Rc::clone(&token)).unwrap();
        assert_eq!(v.len(), 1);
        drop(v);
        assert_eq!(Rc::strong_count(&token), 1);
    }

    #[test]
    fn zero_capacity_and_many_cycles() {
        let mut none: StackVec<u8, 0> = StackVec::new();
        assert_eq!(none.push(1), Err(1));
        assert!(none.as_slice().is_empty());
        let mut v: StackVec<Vec<u8>, 16> = StackVec::new();
        for round in 0..1_000_u32 {
            v.push(vec![0; (round % 7) as usize]).unwrap();
            if v.len() == 16 {
                v.clear();
            }
        }
        assert_eq!(v.len(), 1_000 % 16);
    }
}
