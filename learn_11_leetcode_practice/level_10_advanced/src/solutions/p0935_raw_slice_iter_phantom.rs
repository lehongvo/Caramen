//! # Problem 0935: A Raw-Pointer Slice Iterator
//!
//! - **Difficulty:** Hard
//! - **Topics:** `PhantomData<&'a T>` and `PhantomData<&'a mut T>`, start/end pointer iteration, `offset_from`, `DoubleEndedIterator`, manual `Clone` without a `T: Clone` bound
//! - **Source:** Original · [The Rustonomicon: PhantomData](https://doc.rust-lang.org/nomicon/phantom-data.html) · [std: `pointer::offset_from`](https://doc.rust-lang.org/std/primitive.pointer.html#method.offset_from)
//!
//! ## Problem
//!
//! `std::slice::Iter` is not an index plus a reference: it is **two raw pointers**, the next element
//! and one past the last one. A raw pointer carries no lifetime, so the struct also holds a
//! `PhantomData<&'a T>`: it tells the compiler the iterator behaves like a `&'a T`, which ties it to
//! the slice's borrow and gives it the right variance and auto traits.
//!
//! ```text
//! pub struct RawIter<'a, T>    { ptr: *const T, end: *const T, _marker: PhantomData<&'a T> }
//! pub struct RawIterMut<'a, T> { ptr: *mut T,   end: *mut T,   _marker: PhantomData<&'a mut T> }
//! ```
//!
//! Implement:
//!
//! - `RawIter::new(slice: &'a [T])` and `RawIterMut::new(slice: &'a mut [T])`. Both panic with
//!   `"zero-sized types are not supported"` if `size_of::<T>() == 0` (for them every element would
//!   have the same address and the pointer distance would say nothing).
//! - For both: `Iterator` (items `&'a T` / `&'a mut T`), an exact `size_hint` computed from the two
//!   pointers with `offset_from`, `DoubleEndedIterator` and `ExactSizeIterator`.
//! - `Clone for RawIter<'a, T>` **by hand**, without requiring `T: Clone` (`#[derive(Clone)]` would
//!   add that bound even though only pointers are copied).
//!
//! ## Examples
//!
//! ```text
//! let data = [1, 2, 3];
//! RawIter::new(&data).copied().collect::<Vec<_>>()        => [1, 2, 3]
//! RawIter::new(&data).rev().copied().collect::<Vec<_>>()  => [3, 2, 1]
//! RawIter::new(&data).len()                               => 3
//! let mut v = [1, 2, 3];
//! for x in RawIterMut::new(&mut v) { *x *= 10; }          // v == [10, 20, 30]
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `end = ptr.add(len)` is allowed: a pointer one past the end of an allocation is valid to
//!    compute and compare, just not to dereference.
//! 2. `next`: if `ptr == end` return `None`; else take `&*ptr` and advance `ptr` by one.
//!    `next_back`: if not empty, move `end` back by one and return `&*end`.
//! 3. `end.offset_from(ptr) as usize` is the number of remaining elements.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the iterator owns the half-open range `[ptr, end)`. `next` hands out `ptr` and moves it
//! forward, `next_back` moves `end` backward and hands it out; the range only shrinks, so every
//! element is yielded once and two `&mut` items never alias. The lifetime `'a` of the items comes
//! from `PhantomData`, which is also why the iterator cannot outlive the slice.
//!
//! **Complexity:** every method is `O(1)`.
//!
//! **Rust concepts:**
//! - `PhantomData<&'a T>` makes `RawIter` covariant in `'a` and `T` and `Send`/`Sync` exactly when
//!   `&'a T` is; `PhantomData<&'a mut T>` makes `RawIterMut` invariant in `T`, like `&mut T`.
//! - Without the marker, `'a` would be an unused parameter (error E0392), and nothing would stop
//!   the iterator from outliving the slice.
//! - Derives add bounds on every type parameter; writing `Clone` by hand avoids `T: Clone`.
// @solution-end

use std::marker::PhantomData;
use std::mem::size_of;

pub struct RawIter<'a, T> {
    ptr: *const T,
    end: *const T,
    _marker: PhantomData<&'a T>,
}

pub struct RawIterMut<'a, T> {
    ptr: *mut T,
    end: *mut T,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> RawIter<'a, T> {
    pub fn new(slice: &'a [T]) -> Self {
        // @solution-start
        assert!(size_of::<T>() != 0, "zero-sized types are not supported");
        let ptr = slice.as_ptr();
        RawIter {
            ptr,
            // SAFETY: one past the end of the slice is still within its allocation.
            end: unsafe { ptr.add(slice.len()) },
            _marker: PhantomData,
        }
        // @solution-end
    }
}

impl<'a, T> RawIterMut<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Self {
        // @solution-start
        assert!(size_of::<T>() != 0, "zero-sized types are not supported");
        let ptr = slice.as_mut_ptr();
        RawIterMut {
            ptr,
            // SAFETY: one past the end of the slice is still within its allocation.
            end: unsafe { ptr.add(slice.len()) },
            _marker: PhantomData,
        }
        // @solution-end
    }
}

impl<T> Clone for RawIter<'_, T> {
    fn clone(&self) -> Self {
        // @solution-start
        RawIter {
            ptr: self.ptr,
            end: self.end,
            _marker: PhantomData,
        }
        // @solution-end
    }
}

impl<'a, T> Iterator for RawIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        // @solution-start
        if self.ptr == self.end {
            return None;
        }
        // SAFETY: `ptr < end`, so `ptr` points to an element of the borrowed slice, which lives for
        // `'a`; advancing by one keeps it within (or one past) the slice.
        unsafe {
            let item = &*self.ptr;
            self.ptr = self.ptr.add(1);
            Some(item)
        }
        // @solution-end
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // @solution-start
        // SAFETY: both pointers are in the same slice and `ptr <= end`.
        let remaining = unsafe { self.end.offset_from(self.ptr) } as usize;
        (remaining, Some(remaining))
        // @solution-end
    }
}

impl<T> DoubleEndedIterator for RawIter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // @solution-start
        if self.ptr == self.end {
            return None;
        }
        // SAFETY: the range is not empty, so `end - 1` is an element of the slice.
        unsafe {
            self.end = self.end.sub(1);
            Some(&*self.end)
        }
        // @solution-end
    }
}

impl<T> ExactSizeIterator for RawIter<'_, T> {}

impl<'a, T> Iterator for RawIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        // @solution-start
        if self.ptr == self.end {
            return None;
        }
        // SAFETY: `ptr < end` points into the uniquely borrowed slice, and each element is handed
        // out only once because the range `[ptr, end)` only shrinks.
        unsafe {
            let item = &mut *self.ptr;
            self.ptr = self.ptr.add(1);
            Some(item)
        }
        // @solution-end
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // @solution-start
        // SAFETY: both pointers are in the same slice and `ptr <= end`.
        let remaining = unsafe { self.end.offset_from(self.ptr) } as usize;
        (remaining, Some(remaining))
        // @solution-end
    }
}

impl<T> DoubleEndedIterator for RawIterMut<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // @solution-start
        if self.ptr == self.end {
            return None;
        }
        // SAFETY: the range is not empty, so `end - 1` is an element not handed out yet.
        unsafe {
            self.end = self.end.sub(1);
            Some(&mut *self.end)
        }
        // @solution-end
    }
}

impl<T> ExactSizeIterator for RawIterMut<'_, T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterates_forwards_and_backwards() {
        let data = [1, 2, 3];
        assert_eq!(
            RawIter::new(&data).copied().collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
        assert_eq!(
            RawIter::new(&data).rev().copied().collect::<Vec<_>>(),
            vec![3, 2, 1]
        );
        let mut it = RawIter::new(&data);
        assert_eq!(it.len(), 3);
        assert_eq!(it.next_back(), Some(&3));
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.len(), 1);
        assert_eq!(it.next(), Some(&2));
        assert_eq!((it.next(), it.next_back()), (None, None));
    }

    #[test]
    fn clone_without_t_clone() {
        struct NotClone(u8);
        let data = [NotClone(1), NotClone(2)];
        let it = RawIter::new(&data);
        let copy = it.clone();
        assert_eq!(it.map(|x| x.0).sum::<u8>(), 3);
        assert_eq!(copy.len(), 2);
    }

    #[test]
    fn mutable_iteration() {
        let mut v = [1, 2, 3];
        for x in RawIterMut::new(&mut v) {
            *x *= 10;
        }
        assert_eq!(v, [10, 20, 30]);
        let mut it = RawIterMut::new(&mut v);
        let first = it.next().unwrap();
        let last = it.next_back().unwrap();
        std::mem::swap(first, last);
        assert_eq!(it.len(), 1);
        assert_eq!(v, [30, 20, 10]);
    }

    #[test]
    fn empty_and_strings() {
        let empty: [String; 0] = [];
        assert_eq!(RawIter::new(&empty).next(), None);
        let mut words = vec!["a".to_string(), "b".to_string()];
        for w in RawIterMut::new(&mut words) {
            w.push('!');
        }
        let joined: Vec<&str> = RawIter::new(&words).map(String::as_str).collect();
        assert_eq!(joined, ["a!", "b!"]);
    }

    #[test]
    #[should_panic(expected = "zero-sized types are not supported")]
    fn zero_sized_types_panic() {
        let units = [(), ()];
        let _ = RawIter::new(&units);
    }

    #[test]
    fn long_slice() {
        let data: Vec<u64> = (1..=100_000).collect();
        assert_eq!(RawIter::new(&data).sum::<u64>(), 5_000_050_000);
        assert_eq!(RawIter::new(&data).rev().nth(9), Some(&99_991));
    }
}
