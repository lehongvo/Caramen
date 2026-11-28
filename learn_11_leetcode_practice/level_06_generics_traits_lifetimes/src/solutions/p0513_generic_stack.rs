//! # Problem 0513: A Generic Stack
//!
//! - **Difficulty:** Easy
//! - **Topics:** generic data structure `Stack<T>`, `Option<&T>`, `Option<&mut T>`, `Default`
//! - **Source:** Original · [The Rust Book 10.1](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-struct-definitions)
//!
//! ## Problem
//!
//! Build a last-in, first-out stack that can hold values of any type `T`. The struct keeps its
//! elements in a private field `items: Vec<T>` (the top of the stack is the end of the vector).
//!
//! Implement in `impl<T> Stack<T>`:
//!
//! - `new() -> Self`: an empty stack.
//! - `push(&mut self, item: T)`: puts `item` on top.
//! - `pop(&mut self) -> Option<T>`: removes and returns the top, or `None` when empty.
//! - `peek(&self) -> Option<&T>`: borrows the top without removing it.
//! - `peek_mut(&mut self) -> Option<&mut T>`: a mutable borrow of the top, to change it in place.
//! - `len(&self) -> usize` and `is_empty(&self) -> bool`.
//! - `into_vec(self) -> Vec<T>`: consumes the stack and returns its elements from **bottom** to top.
//!
//! Also implement `Default` for `Stack<T>` (an empty stack), so `Stack::default()` works and
//! `#[derive(Default)]` structs can contain a stack.
//!
//! ## Examples
//!
//! ```text
//! let mut s = Stack::new();
//! s.push(1); s.push(2); s.push(3);
//! s.peek()          => Some(&3)
//! s.pop()           => Some(3)
//! *s.peek_mut().unwrap() *= 10;
//! s.into_vec()      => [1, 20]
//! ```
//!
//! ## Constraints
//!
//! - Up to `10^5` operations.
//!
//! ## Hints
//!
//! 1. Every method maps to one `Vec` method: `push`, `pop`, `last`, `last_mut`, `len`, `is_empty`.
//! 2. `impl<T> Default for Stack<T>` needs no bound on `T`: an empty `Vec<T>` exists for every `T`.
//!    (`#[derive(Default)]` would add a `T: Default` bound, which is why we write it by hand.)
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** a thin wrapper around `Vec<T>` whose end is the top of the stack.
//!
//! **Rust concepts:**
//! - The struct is generic, so `Stack<String>`, `Stack<(u8, char)>` and `Stack<Vec<i32>>` all reuse
//!   the same code; the compiler generates a specialised copy for each `T` (monomorphization).
//! - `peek` returns `Option<&T>` and `peek_mut` returns `Option<&mut T>`: callers borrow the top
//!   instead of copying it, so `T` needs no `Clone` bound.
//! - A manual `impl<T> Default` avoids the `T: Default` bound that `#[derive(Default)]` adds to
//!   generic types. Clippy's `new_without_default` lint also recommends providing `Default` next to
//!   a `new()` without arguments.
// @solution-end

/// A last-in, first-out stack.
#[derive(Debug, Clone, PartialEq)]
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        // @solution-start
        Stack { items: Vec::new() }
        // @solution-end
    }

    pub fn push(&mut self, item: T) {
        // @solution-start
        self.items.push(item);
        // @solution-end
    }

    pub fn pop(&mut self) -> Option<T> {
        // @solution-start
        self.items.pop()
        // @solution-end
    }

    pub fn peek(&self) -> Option<&T> {
        // @solution-start
        self.items.last()
        // @solution-end
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // @solution-start
        self.items.last_mut()
        // @solution-end
    }

    pub fn len(&self) -> usize {
        // @solution-start
        self.items.len()
        // @solution-end
    }

    pub fn is_empty(&self) -> bool {
        // @solution-start
        self.items.is_empty()
        // @solution-end
    }

    /// The elements from bottom to top.
    pub fn into_vec(self) -> Vec<T> {
        // @solution-start
        self.items
        // @solution-end
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        // @solution-start
        Self::new()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut s = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.peek(), Some(&3));
        assert_eq!(s.pop(), Some(3));
        *s.peek_mut().unwrap() *= 10;
        assert_eq!(s.len(), 2);
        assert_eq!(s.into_vec(), vec![1, 20]);
    }

    #[test]
    fn empty_stack() {
        let mut s: Stack<String> = Stack::default();
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
        assert_eq!(s.peek(), None);
        assert_eq!(s.peek_mut(), None);
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn holds_non_copy_values() {
        let mut s = Stack::new();
        s.push(String::from("a"));
        s.push(String::from("b"));
        s.peek_mut().unwrap().push('!');
        assert_eq!(s.pop().as_deref(), Some("b!"));
        assert_eq!(s.peek().map(String::as_str), Some("a"));
    }

    #[test]
    fn default_inside_derived_struct() {
        #[derive(Default)]
        struct Undo {
            history: Stack<Vec<u8>>,
        }
        let mut u = Undo::default();
        u.history.push(vec![1]);
        assert_eq!(u.history.len(), 1);
    }

    #[test]
    fn many_operations() {
        let mut s = Stack::new();
        for i in 0..100_000 {
            s.push(i);
        }
        let mut expected = 99_999;
        while let Some(top) = s.pop() {
            assert_eq!(top, expected);
            expected -= 1;
            if top == 50_000 {
                break;
            }
        }
        assert_eq!(s.len(), 50_000);
        assert_eq!(s.peek(), Some(&49_999));
    }
}
