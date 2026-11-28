// ==================================================================
// PRACTICE 0576: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0576_into_iterator_for_bag.md
// Run the tests:          cargo test practice::p0576
// ==================================================================

//! # Problem 0576: `IntoIterator` for a Collection
//!
//! - **Difficulty:** Medium
//! - **Topics:** `IntoIterator` for `Bag<T>`, `&Bag<T>` and `&mut Bag<T>`, associated types `Item`/`IntoIter`, `FromIterator`, `Extend`
//! - **Source:** Original · [std::iter::IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
//!
//! ## Problem
//!
//! `for x in collection` calls `IntoIterator::into_iter(collection)`. Standard collections implement it
//! three times, and so should yours. `Bag<T>` is a multiset that keeps insertion order in a private
//! field `items: Vec<T>`.
//!
//! Implement:
//!
//! - `impl<T> Bag<T>`: `new()`, `add(&mut self, item: T)`, `len(&self)`, `is_empty(&self)`,
//!   `iter(&self) -> std::slice::Iter<'_, T>` and `iter_mut(&mut self) -> std::slice::IterMut<'_, T>`.
//! - `impl<T: PartialEq> Bag<T>`: `count(&self, item: &T) -> usize`.
//! - `impl<T> IntoIterator for Bag<T>` with `Item = T` and `IntoIter = std::vec::IntoIter<T>` (consumes
//!   the bag).
//! - `impl<'a, T> IntoIterator for &'a Bag<T>` with `Item = &'a T` (so `for x in &bag` works).
//! - `impl<'a, T> IntoIterator for &'a mut Bag<T>` with `Item = &'a mut T` (so `for x in &mut bag`
//!   can modify the items).
//! - `impl<T> FromIterator<T> for Bag<T>` (so `.collect::<Bag<_>>()` works) and
//!   `impl<T> Extend<T> for Bag<T>`.
//! - `impl<T> Default for Bag<T>`.
//!
//! ## Examples
//!
//! ```text
//! let mut bag: Bag<i32> = [3, 1, 3].into_iter().collect();
//! for x in &mut bag { *x *= 10; }            // [30, 10, 30]
//! bag.count(&30)                             => 2
//! let total: i32 = (&bag).into_iter().sum(); => 70
//! bag.extend([5, 5]);
//! bag.into_iter().collect::<Vec<_>>()        => [30, 10, 30, 5, 5]
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Each impl can delegate to the matching impl of `Vec<T>`: `self.items.into_iter()`,
//!    `self.items.iter()`, `self.items.iter_mut()`.
//! 2. The reference impls need a named lifetime: `impl<'a, T> IntoIterator for &'a Bag<T> { type Item =
//!    &'a T; type IntoIter = std::slice::Iter<'a, T>; ... }`.

/// An insertion-ordered multiset.
#[derive(Debug, Clone, PartialEq)]
pub struct Bag<T> {
    items: Vec<T>,
}

impl<T> Bag<T> {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn add(&mut self, item: T) {
        todo!("Write your solution here")
    }

    pub fn len(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn is_empty(&self) -> bool {
        todo!("Write your solution here")
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        todo!("Write your solution here")
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        todo!("Write your solution here")
    }
}

impl<T: PartialEq> Bag<T> {
    pub fn count(&self, item: &T) -> usize {
        todo!("Write your solution here")
    }
}

impl<T> Default for Bag<T> {
    fn default() -> Self {
        todo!("Write your solution here")
    }
}

impl<T> IntoIterator for Bag<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!("Write your solution here")
    }
}

impl<'a, T> IntoIterator for &'a Bag<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!("Write your solution here")
    }
}

impl<'a, T> IntoIterator for &'a mut Bag<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!("Write your solution here")
    }
}

impl<T> FromIterator<T> for Bag<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        todo!("Write your solution here")
    }
}

impl<T> Extend<T> for Bag<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_kinds_of_for_loops() {
        let mut bag: Bag<i32> = [3, 1, 3].into_iter().collect();
        for x in &mut bag {
            *x *= 10;
        }
        let mut seen = Vec::new();
        for x in &bag {
            seen.push(*x);
        }
        assert_eq!(seen, vec![30, 10, 30]);
        let mut owned = Vec::new();
        for x in bag {
            owned.push(x);
        }
        assert_eq!(owned, vec![30, 10, 30]);
    }

    #[test]
    fn counting_extending_and_summing() {
        let mut bag: Bag<i32> = [3, 1, 3].into_iter().map(|x| x * 10).collect();
        assert_eq!(bag.count(&30), 2);
        assert_eq!(bag.count(&7), 0);
        let total: i32 = (&bag).into_iter().sum();
        assert_eq!(total, 70);
        bag.extend([5, 5]);
        bag.extend(vec![6]);
        assert_eq!(bag.len(), 6);
        assert_eq!(
            bag.into_iter().collect::<Vec<_>>(),
            vec![30, 10, 30, 5, 5, 6]
        );
    }

    #[test]
    fn works_with_strings() {
        let mut bag = Bag::new();
        bag.add("a".to_string());
        bag.add("b".to_string());
        for s in bag.iter_mut() {
            s.push('!');
        }
        let joined: Vec<&str> = bag.iter().map(String::as_str).collect();
        assert_eq!(joined, vec!["a!", "b!"]);
        assert!(Bag::<String>::default().is_empty());
    }

    #[test]
    fn large_bag() {
        let bag: Bag<u32> = (0..100_000).map(|i| i % 10).collect();
        assert_eq!(bag.count(&3), 10_000);
        let sum: u64 = bag.iter().map(|&x| u64::from(x)).sum();
        assert_eq!(sum, 450_000);
    }
}
