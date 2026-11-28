//! # Problem 0532: The Monoid Trait
//!
//! - **Difficulty:** Medium
//! - **Topics:** traits with associated functions (`fn empty() -> Self`), `Self` in trait signatures, generic impls with bounds (`impl<A: Monoid, B: Monoid> Monoid for (A, B)`)
//! - **Source:** Original · [The Rust Book 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html#defining-a-trait)
//!
//! ## Problem
//!
//! A *monoid* is a type with an "empty" value and a way to combine two values into one (for example
//! numbers with `0` and `+`, or strings with `""` and concatenation). Model it as a trait:
//!
//! ```text
//! pub trait Monoid {
//!     fn empty() -> Self;                    // no `self`: an associated function
//!     fn combine(self, other: Self) -> Self;
//! }
//! ```
//!
//! Implement it for:
//!
//! - `String`: empty string, concatenation.
//! - `Vec<T>` for every `T`: empty vector, `self` followed by `other`.
//! - Three newtypes over `i64`: `Sum(i64)` (`0`, `+`), `Product(i64)` (`1`, `*`) and `Max(i64)`
//!   (`i64::MIN`, the larger of the two).
//! - Pairs: `(A, B)` is a monoid whenever `A` and `B` are, combining component by component.
//!
//! Then write two generic functions:
//!
//! - `fold_all<M: Monoid>(items: Vec<M>) -> M`: combines all items from left to right, starting with
//!   `M::empty()`.
//! - `fold_map<T, M, F>(items: &[T], f: F) -> M where M: Monoid, F: Fn(&T) -> M`: maps every item to a
//!   monoid value and folds the results.
//!
//! ## Examples
//!
//! ```text
//! fold_all(vec![Sum(2), Sum(3), Sum(5)])               => Sum(10)
//! fold_all(vec!["ab".to_string(), "c".to_string()])    => "abc"
//! fold_all::<Max>(vec![])                              => Max(i64::MIN)
//! fold_map(&[3, 1, 4], |&x| (Sum(x), Max(x)))          => (Sum(8), Max(4))
//! fold_map(&["a", "b"], |s| vec![s.len()])             => [1, 1]
//! ```
//!
//! ## Constraints
//!
//! - Results fit in `i64` in the tests.
//!
//! ## Hints
//!
//! 1. Inside a trait, `Self` means "the implementing type". `fn empty() -> Self` is called as
//!    `M::empty()` or `String::empty()`.
//! 2. The pair impl starts with `impl<A: Monoid, B: Monoid> Monoid for (A, B)`; its `empty` is
//!    `(A::empty(), B::empty())`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each impl states its identity element and combining rule. `fold_all` is a plain
//! `fold` starting at `M::empty()`; `fold_map` maps first. The pair impl lets one pass compute several
//! aggregates at once.
//!
//! **Rust concepts:**
//! - Associated functions without `self` (like `empty`) are how traits describe constructors; generic
//!   code calls them through the type parameter: `M::empty()`.
//! - `combine(self, other: Self)` takes both values by value, so combining `String`s and `Vec`s can
//!   reuse the left allocation instead of cloning.
//! - `impl<T> Monoid for Vec<T>` and `impl<A: Monoid, B: Monoid> Monoid for (A, B)` are generic
//!   impls: one block covers infinitely many types, and the bounds say when the impl applies.
//! - Newtypes (`Sum`, `Product`, `Max`) give the same `i64` several different monoid behaviours,
//!   because a type can implement a trait only once.
// @solution-end

/// A type with an identity element and an associative combine operation.
pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sum(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Product(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Max(pub i64);

impl Monoid for String {
    fn empty() -> Self {
        // @solution-start
        String::new()
        // @solution-end
    }

    fn combine(self, other: Self) -> Self {
        // @solution-start
        self + &other
        // @solution-end
    }
}

impl<T> Monoid for Vec<T> {
    fn empty() -> Self {
        // @solution-start
        Vec::new()
        // @solution-end
    }

    fn combine(self, other: Self) -> Self {
        // @solution-start
        let mut out = self;
        out.extend(other);
        out
        // @solution-end
    }
}

impl Monoid for Sum {
    fn empty() -> Self {
        // @solution-start
        Sum(0)
        // @solution-end
    }

    fn combine(self, other: Self) -> Self {
        // @solution-start
        Sum(self.0 + other.0)
        // @solution-end
    }
}

impl Monoid for Product {
    fn empty() -> Self {
        // @solution-start
        Product(1)
        // @solution-end
    }

    fn combine(self, other: Self) -> Self {
        // @solution-start
        Product(self.0 * other.0)
        // @solution-end
    }
}

impl Monoid for Max {
    fn empty() -> Self {
        // @solution-start
        Max(i64::MIN)
        // @solution-end
    }

    fn combine(self, other: Self) -> Self {
        // @solution-start
        Max(self.0.max(other.0))
        // @solution-end
    }
}

impl<A: Monoid, B: Monoid> Monoid for (A, B) {
    fn empty() -> Self {
        // @solution-start
        (A::empty(), B::empty())
        // @solution-end
    }

    fn combine(self, other: Self) -> Self {
        // @solution-start
        (self.0.combine(other.0), self.1.combine(other.1))
        // @solution-end
    }
}

/// Combines all items from left to right, starting with `M::empty()`.
pub fn fold_all<M: Monoid>(items: Vec<M>) -> M {
    // @solution-start
    items.into_iter().fold(M::empty(), M::combine)
    // @solution-end
}

/// Maps every item with `f` and folds the results.
pub fn fold_map<T, M, F>(items: &[T], f: F) -> M
where
    M: Monoid,
    F: Fn(&T) -> M,
{
    // @solution-start
    items
        .iter()
        .fold(M::empty(), |acc, item| acc.combine(f(item)))
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_newtypes() {
        assert_eq!(fold_all(vec![Sum(2), Sum(3), Sum(5)]), Sum(10));
        assert_eq!(
            fold_all(vec![Product(2), Product(3), Product(5)]),
            Product(30)
        );
        assert_eq!(fold_all(vec![Max(-4), Max(9), Max(2)]), Max(9));
        assert_eq!(fold_all::<Max>(vec![]), Max(i64::MIN));
        assert_eq!(fold_all::<Product>(vec![]), Product(1));
    }

    #[test]
    fn strings_and_vectors() {
        assert_eq!(fold_all(vec!["ab".to_string(), "c".to_string()]), "abc");
        assert_eq!(fold_all(vec![vec![1, 2], vec![], vec![3]]), vec![1, 2, 3]);
        assert_eq!(String::empty(), "");
        assert_eq!(fold_map(&["a", "bcd"], |s| vec![s.len()]), vec![1, 3]);
    }

    #[test]
    fn pairs_compute_two_aggregates_at_once() {
        assert_eq!(
            fold_map(&[3, 1, 4], |&x| (Sum(x), Max(x))),
            (Sum(8), Max(4))
        );
        let (count, text): (Sum, String) =
            fold_map(&["x", "y", "z"], |s| (Sum(1), s.to_uppercase()));
        assert_eq!(count, Sum(3));
        assert_eq!(text, "XYZ");
        let nested: ((Sum, Product), Max) = fold_map(&[2, 5], |&x| ((Sum(x), Product(x)), Max(-x)));
        assert_eq!(nested, ((Sum(7), Product(10)), Max(-2)));
    }

    #[test]
    fn large_fold() {
        let values: Vec<i64> = (1..=100_000).collect();
        let (sum, max) = fold_map(&values, |&x| (Sum(x), Max(x)));
        assert_eq!(sum, Sum(5_000_050_000));
        assert_eq!(max, Max(100_000));
        let words: Vec<String> = (0..1000).map(|i| (i % 10).to_string()).collect();
        assert_eq!(fold_all(words).len(), 1000);
    }
}
