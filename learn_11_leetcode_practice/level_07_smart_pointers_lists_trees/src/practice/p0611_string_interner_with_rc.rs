// ==================================================================
// PRACTICE 0611: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0611_string_interner_with_rc.md
// Run the tests:          cargo test practice::p0611
// ==================================================================

//! # Problem 0611: String Interner with Rc
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Rc<str>`, `Rc::ptr_eq`, `Rc::as_ptr`, `HashSet<Rc<str>>` looked up by `&str` (`Borrow`)
//! - **Source:** Original · [The Rust Book 15.4](https://doc.rust-lang.org/book/ch15-04-rc.html) · [`Rc::ptr_eq`](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.ptr_eq)
//!
//! ## Problem
//!
//! A program that handles many copies of the same few strings (identifiers in a compiler, tags in
//! a log) can store each distinct string **once** and hand out cheap shared handles to it. This
//! is called *interning*.
//!
//! Implement an `Interner`:
//!
//! - `Interner::new() -> Interner`;
//! - `intern(&mut self, text: &str) -> Rc<str>`: return a handle to the stored copy of `text`,
//!   creating it on first use. Two calls with equal text must return handles to the **same**
//!   allocation (`Rc::ptr_eq` is `true`);
//! - `len(&self) -> usize` / `is_empty(&self) -> bool`: the number of distinct strings stored;
//! - `contains(&self, text: &str) -> bool`.
//!
//! Also implement the free function `count_allocations(items: &[Rc<str>]) -> usize`, which counts
//! how many **different allocations** the handles point to. Equal text in two separate
//! allocations counts twice; compare pointers, not contents.
//!
//! ## Examples
//!
//! ```text
//! let mut interner = Interner::new();
//! let a = interner.intern("hello");
//! let b = interner.intern("hello");
//! let c = interner.intern("world");
//! Rc::ptr_eq(&a, &b) == true, Rc::ptr_eq(&a, &c) == false
//! interner.len() == 2
//! Rc::strong_count(&a) == 3          // the interner's copy + a + b
//!
//! count_allocations(&[a.clone(), b.clone(), c.clone()])            == 2
//! count_allocations(&[Rc::from("x"), Rc::from("x")])               == 2   // two separate copies
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` calls; strings are at most `100` bytes long.
//!
//! ## Hints
//!
//! 1. Store the handles in a `HashSet<Rc<str>>`. Because `Rc<str>: Borrow<str>`, you can call
//!    `set.get(text)` with a plain `&str`.
//! 2. `Rc::from(text)` copies a `&str` into a new `Rc<str>` allocation.
//! 3. `Rc::as_ptr(&rc)` gives a raw pointer you can put in a `HashSet` to count distinct
//!    allocations. For `str` it is a "fat" pointer (address + length); `.cast::<u8>()` keeps just
//!    the address.

use std::collections::HashSet;
use std::rc::Rc;

#[derive(Default)]
pub struct Interner {}

impl Interner {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn intern(&mut self, text: &str) -> Rc<str> {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn contains(&self, text: &str) -> bool {
        todo!("Write your solution here")
    }
}

pub fn count_allocations(items: &[Rc<str>]) -> usize {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_text_shares_one_allocation() {
        let mut interner = Interner::new();
        let a = interner.intern("hello");
        let b = interner.intern("hello");
        let c = interner.intern("world");
        assert!(Rc::ptr_eq(&a, &b));
        assert!(!Rc::ptr_eq(&a, &c));
        assert_eq!(&*a, "hello");
        assert_eq!(interner.len(), 2);
        assert_eq!(Rc::strong_count(&a), 3);
    }

    #[test]
    fn contains_and_empty() {
        let mut interner = Interner::new();
        assert!(interner.is_empty());
        assert!(!interner.contains("x"));
        let _x = interner.intern("x");
        assert!(interner.contains("x"));
        assert!(!interner.contains("y"));
        assert!(!interner.is_empty());
    }

    #[test]
    fn count_allocations_compares_pointers() {
        let mut interner = Interner::new();
        let a = interner.intern("tag");
        let b = interner.intern("tag");
        let c = interner.intern("other");
        assert_eq!(count_allocations(&[a.clone(), b, c, a]), 2);
        let copies: Vec<Rc<str>> = vec![Rc::from("x"), Rc::from("x")];
        assert_eq!(count_allocations(&copies), 2);
        assert_eq!(count_allocations(&[]), 0);
    }

    #[test]
    fn many_repeated_words() {
        let mut interner = Interner::new();
        let words = ["red", "green", "blue"];
        let handles: Vec<Rc<str>> = (0..3000).map(|i| interner.intern(words[i % 3])).collect();
        assert_eq!(interner.len(), 3);
        assert_eq!(count_allocations(&handles), 3);
        // 1000 handles in the vector + the interner's own copy.
        assert_eq!(Rc::strong_count(&handles[0]), 1001);
    }
}
