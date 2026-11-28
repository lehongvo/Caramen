//! # Problem 0559: A Sorted Vec with `Deref`
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Deref<Target = [T]>` on a newtype, deref coercion, why not `DerefMut`, inherent methods shadowing slice methods, `FromIterator`
//! - **Source:** Original · [std::ops::Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html)
//!
//! ## Problem
//!
//! `SortedVec<T>` wraps a `Vec<T>` (private field `items`) and guarantees that the elements are always
//! sorted in non-decreasing order. Implement:
//!
//! - `impl<T: Ord> SortedVec<T>`:
//!   - `new() -> Self` and `from_vec(v: Vec<T>) -> Self` (sorts `v`).
//!   - `insert(&mut self, x: T)`: inserts `x` at its sorted position, **after** any equal elements.
//!   - `remove(&mut self, x: &T) -> bool`: removes one occurrence of `x` if present.
//!   - `contains(&self, x: &T) -> bool` using **binary search**.
//! - `impl<T> Deref for SortedVec<T>` with `type Target = [T]`: every read-only slice method (`len`,
//!   `first`, `iter`, `windows`, indexing `s[i]`, slicing `&s[1..3]`, ...) now works on a `SortedVec`.
//! - `impl<T> Default` and `impl<T: Ord> FromIterator<T>` (so `iter.collect::<SortedVec<_>>()` works).
//!
//! Do **not** implement `DerefMut`: it would give out `&mut [T]`, and code like `s[0] = 99` or
//! `s.reverse()` could break the sorted invariant.
//!
//! ## Examples
//!
//! ```text
//! let mut s = SortedVec::from_vec(vec![5, 1, 4]);
//! s.insert(3);                // [1, 3, 4, 5]
//! s.len()                     => 4          (slice method through Deref)
//! s[0]                        => 1
//! &s[1..3]                    => [3, 4]
//! s.contains(&4)              => true       (our binary-search version)
//! s.remove(&1)                => true       // [3, 4, 5]
//! (1..=3).rev().collect::<SortedVec<_>>().to_vec() => [1, 2, 3]
//! ```
//!
//! ## Constraints
//!
//! - Up to `10^5` elements.
//!
//! ## Hints
//!
//! 1. `self.items.partition_point(|y| y <= &x)` is the index after the last element `<= x`.
//! 2. `self.items.binary_search(x)` returns `Ok(index)` when `x` is present.
//! 3. When a type has an inherent method and its `Deref` target has one with the same name, the
//!    inherent one wins, so `s.contains(&x)` calls your `O(log n)` version, not the slice's `O(n)` one.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** all mutation goes through methods that keep the vector sorted; all reading goes through
//! `Deref` to `[T]`, so the type gets the whole slice API for free.
//!
//! **Complexity:** `insert`/`remove` are `O(n)` (shifting elements) plus `O(log n)` search;
//! `contains` is `O(log n)`.
//!
//! **Rust concepts:**
//! - `Deref` enables *deref coercion*: a `&SortedVec<T>` is accepted where a `&[T]` is expected, and
//!   method calls look through `Deref` automatically.
//! - Deref is best used for smart-pointer-like wrappers; giving only `Deref` (not `DerefMut`) is a
//!   common way to expose read-only access while protecting an invariant.
//! - Method resolution checks the type's own methods before auto-deref, which is why the inherent
//!   `contains` shadows `<[T]>::contains`.
// @solution-end

use std::ops::Deref;

/// A vector that is always sorted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortedVec<T> {
    items: Vec<T>,
}

impl<T: Ord> SortedVec<T> {
    pub fn new() -> Self {
        // @solution-start
        SortedVec { items: Vec::new() }
        // @solution-end
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        // @solution-start
        let mut items = v;
        items.sort();
        SortedVec { items }
        // @solution-end
    }

    pub fn insert(&mut self, x: T) {
        // @solution-start
        let pos = self.items.partition_point(|y| y <= &x);
        self.items.insert(pos, x);
        // @solution-end
    }

    pub fn remove(&mut self, x: &T) -> bool {
        // @solution-start
        match self.items.binary_search(x) {
            Ok(i) => {
                self.items.remove(i);
                true
            }
            Err(_) => false,
        }
        // @solution-end
    }

    pub fn contains(&self, x: &T) -> bool {
        // @solution-start
        self.items.binary_search(x).is_ok()
        // @solution-end
    }
}

impl<T> Deref for SortedVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        // @solution-start
        &self.items
        // @solution-end
    }
}

impl<T> Default for SortedVec<T> {
    fn default() -> Self {
        // @solution-start
        SortedVec { items: Vec::new() }
        // @solution-end
    }
}

impl<T: Ord> FromIterator<T> for SortedVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        // @solution-start
        SortedVec::from_vec(iter.into_iter().collect())
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sum(values: &[i32]) -> i32 {
        values.iter().sum()
    }

    #[test]
    fn example_sequence() {
        let mut s = SortedVec::from_vec(vec![5, 1, 4]);
        s.insert(3);
        assert_eq!(s.len(), 4);
        assert_eq!(s[0], 1);
        assert_eq!(&s[1..3], &[3, 4]);
        assert!(s.contains(&4));
        assert!(!s.contains(&2));
        assert!(s.remove(&1));
        assert!(!s.remove(&1));
        assert_eq!(s.to_vec(), vec![3, 4, 5]);
    }

    #[test]
    fn deref_coercion_to_slice() {
        let s: SortedVec<i32> = vec![3, 1, 2].into_iter().collect();
        assert_eq!(sum(&s), 6); // &SortedVec<i32> coerces to &[i32]
        assert_eq!(s.first(), Some(&1));
        assert_eq!(s.iter().rev().copied().collect::<Vec<_>>(), vec![3, 2, 1]);
        assert!(s.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn duplicates_and_strings() {
        let mut s: SortedVec<String> = SortedVec::new();
        for w in ["pear", "apple", "fig", "apple"] {
            s.insert(w.to_string());
        }
        assert_eq!(s.to_vec(), vec!["apple", "apple", "fig", "pear"]);
        assert!(s.remove(&"apple".to_string()));
        assert_eq!(s.len(), 3);
        let empty: SortedVec<u8> = SortedVec::default();
        assert!(empty.is_empty());
    }

    #[test]
    fn insert_after_equal_elements() {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
        struct Key(u8);
        let mut s = SortedVec::new();
        s.insert(Key(2));
        s.insert(Key(1));
        s.insert(Key(2));
        assert_eq!(s.to_vec(), vec![Key(1), Key(2), Key(2)]);
    }

    #[test]
    fn stays_sorted_under_many_inserts() {
        let mut s = SortedVec::new();
        for i in 0..20_000_u32 {
            s.insert(i.wrapping_mul(2_654_435_761) % 10_007);
        }
        assert_eq!(s.len(), 20_000);
        assert!(s.windows(2).all(|w| w[0] <= w[1]));
        let target = s[12_345];
        assert!(s.contains(&target));
    }
}
