// ==================================================================
// PRACTICE 0594: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0594_associated_types_container.md
// Run the tests:          cargo test practice::p0594
// ==================================================================

//! # Problem 0594: Associated Types
//!
//! - **Difficulty:** Medium
//! - **Topics:** `type Item` in a trait, `Self::Item` in signatures, bounds on associated types (`C::Item: Display`), equality constraints (`C: Container<Item = i64>`)
//! - **Source:** Original · [The Rust Book 20.2](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types)
//!
//! ## Problem
//!
//! An **associated type** is a placeholder type that each implementor fills in once. Define
//!
//! ```text
//! pub trait Container {
//!     type Item;
//!     fn get(&self, i: usize) -> Option<&Self::Item>;
//!     fn put(&mut self, item: Self::Item);
//!     fn len(&self) -> usize;
//!     fn is_empty(&self) -> bool { ... }                 // default: len() == 0
//!     fn first(&self) -> Option<&Self::Item> { ... }     // default: get(0)
//! }
//! ```
//!
//! and implement it for:
//!
//! - `Stack<T> { items: Vec<T> }`, generic: `type Item = T`; `put` pushes, `get(0)` is the **bottom**.
//! - `Letters { chars: Vec<char> }`: `type Item = char`; `put` only stores ASCII letters, lowercased
//!   (anything else is ignored).
//!
//! Then write generic functions over any container:
//!
//! - `describe<C>(c: &C) -> String where C: Container, C::Item: Display`: `"[a, b, c]"`.
//! - `fill<C: Container>(c: &mut C, items: Vec<C::Item>)`: puts every item in order.
//! - `largest<C>(c: &C) -> Option<&C::Item> where C: Container, C::Item: PartialOrd`: the first maximum.
//! - `sum_all<C: Container<Item = i64>>(c: &C) -> i64`: only accepts containers **of `i64`**.
//!
//! Both struct types have a `new()` constructor and implement `Default`.
//!
//! ## Examples
//!
//! ```text
//! let mut s = Stack::new(); fill(&mut s, vec![3, 9, 4]);
//! describe(&s)              => "[3, 9, 4]"
//! largest(&s)               => Some(&9)
//! let mut l = Letters::new(); fill(&mut l, "Hi 5!".chars().collect());
//! describe(&l)              => "[h, i]"
//! sum_all(&Stack { items: vec![1_i64, 2, 3] }) => 6
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Inside the trait, refer to the placeholder as `Self::Item`; in generic code, as `C::Item`.
//! 2. `where C::Item: Display` puts a bound on the associated type, not on `C`.
//! 3. `C: Container<Item = i64>` fixes the associated type, so `c.get(i)` returns `Option<&i64>`.

use std::fmt::Display;

/// Indexed storage of some item type.
pub trait Container {
    type Item;

    fn get(&self, i: usize) -> Option<&Self::Item>;
    fn put(&mut self, item: Self::Item);
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    fn first(&self) -> Option<&Self::Item> {
        todo!("Write your solution here")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stack<T> {
    pub items: Vec<T>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Letters {
    pub chars: Vec<char>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        todo!("Write your solution here")
    }
}

impl Letters {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }
}

impl<T> Container for Stack<T> {
    type Item = T;

    fn get(&self, i: usize) -> Option<&T> {
        todo!("Write your solution here")
    }

    fn put(&mut self, item: T) {
        todo!("Write your solution here")
    }

    fn len(&self) -> usize {
        todo!("Write your solution here")
    }
}

impl Container for Letters {
    type Item = char;

    fn get(&self, i: usize) -> Option<&char> {
        todo!("Write your solution here")
    }

    fn put(&mut self, item: char) {
        todo!("Write your solution here")
    }

    fn len(&self) -> usize {
        todo!("Write your solution here")
    }
}

/// `"[a, b, c]"`
pub fn describe<C>(c: &C) -> String
where
    C: Container,
    C::Item: Display,
{
    todo!("Write your solution here")
}

/// Puts every item in order.
pub fn fill<C: Container>(c: &mut C, items: Vec<C::Item>) {
    todo!("Write your solution here")
}

/// The first maximum.
pub fn largest<C>(c: &C) -> Option<&C::Item>
where
    C: Container,
    C::Item: PartialOrd,
{
    todo!("Write your solution here")
}

/// The sum of a container of `i64`.
pub fn sum_all<C: Container<Item = i64>>(c: &C) -> i64 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_container() {
        let mut s = Stack::new();
        fill(&mut s, vec![3, 9, 4]);
        assert_eq!(describe(&s), "[3, 9, 4]");
        assert_eq!(largest(&s), Some(&9));
        assert_eq!(s.first(), Some(&3));
        assert_eq!(s.len(), 3);
        assert!(!s.is_empty());
    }

    #[test]
    fn letters_container() {
        let mut l = Letters::new();
        fill(&mut l, "Hi 5!".chars().collect());
        assert_eq!(describe(&l), "[h, i]");
        assert_eq!(largest(&l), Some(&'i'));
        assert!(Letters::default().is_empty());
        assert_eq!(Letters::default().first(), None);
    }

    #[test]
    fn equality_constraint_on_item() {
        assert_eq!(
            sum_all(&Stack {
                items: vec![1_i64, 2, 3]
            }),
            6
        );
        let mut big: Stack<i64> = Stack::default();
        fill(&mut big, (1..=1000).collect());
        assert_eq!(sum_all(&big), 500_500);
    }

    #[test]
    fn works_with_strings_and_floats() {
        let mut words = Stack::new();
        fill(&mut words, vec!["pear".to_string(), "fig".to_string()]);
        assert_eq!(describe(&words), "[pear, fig]");
        assert_eq!(largest(&words).map(String::as_str), Some("pear"));
        let floats = Stack {
            items: vec![1.5, 2.5, 2.5],
        };
        assert!(std::ptr::eq(largest(&floats).unwrap(), &floats.items[1]));
        assert_eq!(describe(&Stack::<u8>::new()), "[]");
    }
}
