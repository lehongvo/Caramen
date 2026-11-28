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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the trait describes indexed storage; each implementor says what it stores. The generic
//! functions walk indices `0..len()` through `get`.
//!
//! **Rust concepts:**
//! - With an associated type, a type implements `Container` **once**, and callers never write the item
//!   type: `describe(&s)` infers everything.
//! - `C::Item` is a projection; bounds such as `C::Item: Display` or `C::Item: PartialOrd` restrict which
//!   containers a function accepts.
//! - `Container<Item = i64>` is an equality constraint: like `Iterator<Item = i64>` in std.
// @solution-end

use std::fmt::Display;

/// Indexed storage of some item type.
pub trait Container {
    type Item;

    fn get(&self, i: usize) -> Option<&Self::Item>;
    fn put(&mut self, item: Self::Item);
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        // @solution-start
        self.len() == 0
        // @solution-end
    }

    fn first(&self) -> Option<&Self::Item> {
        // @solution-start
        self.get(0)
        // @solution-end
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
        // @solution-start
        Stack { items: Vec::new() }
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

impl Letters {
    pub fn new() -> Self {
        // @solution-start
        Letters { chars: Vec::new() }
        // @solution-end
    }
}

impl<T> Container for Stack<T> {
    type Item = T;

    fn get(&self, i: usize) -> Option<&T> {
        // @solution-start
        self.items.get(i)
        // @solution-end
    }

    fn put(&mut self, item: T) {
        // @solution-start
        self.items.push(item);
        // @solution-end
    }

    fn len(&self) -> usize {
        // @solution-start
        self.items.len()
        // @solution-end
    }
}

impl Container for Letters {
    type Item = char;

    fn get(&self, i: usize) -> Option<&char> {
        // @solution-start
        self.chars.get(i)
        // @solution-end
    }

    fn put(&mut self, item: char) {
        // @solution-start
        if item.is_ascii_alphabetic() {
            self.chars.push(item.to_ascii_lowercase());
        }
        // @solution-end
    }

    fn len(&self) -> usize {
        // @solution-start
        self.chars.len()
        // @solution-end
    }
}

/// `"[a, b, c]"`
pub fn describe<C>(c: &C) -> String
where
    C: Container,
    C::Item: Display,
{
    // @solution-start
    let parts: Vec<String> = (0..c.len())
        .filter_map(|i| c.get(i))
        .map(|x| x.to_string())
        .collect();
    format!("[{}]", parts.join(", "))
    // @solution-end
}

/// Puts every item in order.
pub fn fill<C: Container>(c: &mut C, items: Vec<C::Item>) {
    // @solution-start
    for item in items {
        c.put(item);
    }
    // @solution-end
}

/// The first maximum.
pub fn largest<C>(c: &C) -> Option<&C::Item>
where
    C: Container,
    C::Item: PartialOrd,
{
    // @solution-start
    let mut best: Option<&C::Item> = None;
    for i in 0..c.len() {
        if let Some(x) = c.get(i) {
            if best.is_none_or(|b| x > b) {
                best = Some(x);
            }
        }
    }
    best
    // @solution-end
}

/// The sum of a container of `i64`.
pub fn sum_all<C: Container<Item = i64>>(c: &C) -> i64 {
    // @solution-start
    (0..c.len()).filter_map(|i| c.get(i)).sum()
    // @solution-end
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
