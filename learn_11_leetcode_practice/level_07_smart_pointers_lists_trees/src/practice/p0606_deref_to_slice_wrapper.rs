// ==================================================================
// PRACTICE 0606: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0606_deref_to_slice_wrapper.md
// Run the tests:          cargo test practice::p0606
// ==================================================================

//! # Problem 0606: A Stack that Derefs to a Slice
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Deref<Target = [T]>`, `DerefMut`, slice methods through a wrapper, generic structs
//! - **Source:** Original · [The Rust Book 15.2](https://doc.rust-lang.org/book/ch15-02-deref.html) · [`std::ops::DerefMut`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html)
//!
//! ## Problem
//!
//! `Vec<T>` dereferences to the slice `[T]`; that is why every slice method (`len`, `iter`,
//! `contains`, `sort`, indexing, ...) works on a vector. Give your own wrapper the same power.
//!
//! ```text
//! pub struct Stack<T> {
//!     items: Vec<T>,
//! }
//! ```
//!
//! Implement:
//!
//! - `Stack::new() -> Stack<T>`: an empty stack;
//! - `push(&mut self, item: T)` and `pop(&mut self) -> Option<T>` (last in, first out);
//! - `Deref` with `Target = [T]`: the slice of items, **bottom first** (index `0` is the oldest);
//! - `DerefMut`: the same slice, mutable, so callers can sort it or change items in place.
//!
//! You must not write `len`, `iter`, `contains`, ... yourself: they come from the slice.
//!
//! ## Examples
//!
//! ```text
//! let mut s = Stack::new();
//! s.push(3); s.push(1); s.push(2);
//! s.len()            -> 3
//! s[0]               -> 3              // Index on the slice
//! s.contains(&1)     -> true
//! s.iter().max()     -> Some(&3)
//! s.sort();                            // DerefMut: now [1, 2, 3]
//! s.pop()            -> Some(3)
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `fn deref(&self) -> &[T] { &self.items }` compiles because `&Vec<T>` coerces to `&[T]`;
//!    `self.items.as_slice()` says the same thing explicitly.
//! 2. `DerefMut` has no `Target` of its own: it reuses the one from `Deref`.

use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn push(&mut self, item: T) {
        todo!("Write your solution here")
    }

    pub fn pop(&mut self) -> Option<T> {
        todo!("Write your solution here")
    }
}

impl<T> Deref for Stack<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        todo!("Write your solution here")
    }
}

impl<T> DerefMut for Stack<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stack_of(values: &[i32]) -> Stack<i32> {
        let mut s = Stack::new();
        for &v in values {
            s.push(v);
        }
        s
    }

    #[test]
    fn slice_methods_work_through_deref() {
        let s = stack_of(&[3, 1, 2]);
        assert_eq!(s.len(), 3);
        assert_eq!(s[0], 3);
        assert!(s.contains(&1));
        assert_eq!(s.iter().max(), Some(&3));
        assert_eq!(s.first(), Some(&3));
        assert_eq!(s.last(), Some(&2));
    }

    #[test]
    fn deref_mut_allows_sorting_and_editing() {
        let mut s = stack_of(&[3, 1, 2]);
        s.sort();
        assert_eq!(&*s, &[1, 2, 3]);
        s[0] = 10;
        if let Some(top) = s.last_mut() {
            *top *= 100;
        }
        assert_eq!(&*s, &[10, 2, 300]);
    }

    #[test]
    fn push_and_pop_are_lifo() {
        let mut s = stack_of(&[1, 2, 3]);
        assert_eq!(s.pop(), Some(3));
        assert_eq!(s.pop(), Some(2));
        s.push(9);
        assert_eq!(s.pop(), Some(9));
        assert_eq!(s.pop(), Some(1));
        assert_eq!(s.pop(), None);
        assert!(s.is_empty());
    }

    #[test]
    fn works_for_non_copy_items() {
        let mut s: Stack<String> = Stack::new();
        s.push("b".to_string());
        s.push("a".to_string());
        s.sort();
        assert_eq!(s.join(","), "a,b");
    }
}
