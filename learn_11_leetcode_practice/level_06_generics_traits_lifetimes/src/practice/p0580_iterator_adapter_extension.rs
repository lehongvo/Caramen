// ==================================================================
// PRACTICE 0580: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0580_iterator_adapter_extension.md
// Run the tests:          cargo test practice::p0580
// ==================================================================

//! # Problem 0580: An Iterator Adapter and Extension Trait
//!
//! - **Difficulty:** Medium
//! - **Topics:** adapter structs generic over an inner iterator, `I::Item` in bounds, an extension trait with a blanket `impl<I: Iterator> IterExt for I {}`
//! - **Source:** Original · [The Rust Book 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods)
//!
//! ## Problem
//!
//! Std's `map`, `filter`, `zip` are **adapters**: structs that wrap another iterator. Write two of your
//! own and make them available on every iterator through an extension trait.
//!
//! 1. `Pairs<I: Iterator>` yields overlapping pairs of consecutive items: `[1, 2, 3, 4]` gives
//!    `(1, 2), (2, 3), (3, 4)`. It stores the inner iterator and the previous item
//!    (`prev: Option<I::Item>`), and needs `I::Item: Clone` because each item appears in two pairs.
//! 2. `Interleave<A, B>` alternates between two iterators with the same item type, starting with `A`:
//!    `[1, 2, 3]` and `[10, 20]` give `1, 10, 2, 20, 3`. When one runs out, the rest of the other follows.
//!    It stores both iterators and a flag saying whose turn it is.
//! 3. The extension trait
//!
//!    ```text
//!    pub trait IterExt: Iterator + Sized {
//!        fn pairs(self) -> Pairs<Self> where Self::Item: Clone { ... }
//!        fn interleave<J>(self, other: J) -> Interleave<Self, J::IntoIter>
//!        where J: IntoIterator<Item = Self::Item> { ... }
//!    }
//!    impl<I: Iterator> IterExt for I {}
//!    ```
//!
//!    The empty **blanket impl** gives the two default methods to every iterator in the program.
//!
//! ## Examples
//!
//! ```text
//! [1, 2, 3, 4].into_iter().pairs().collect()           => [(1, 2), (2, 3), (3, 4)]
//! "abc".chars().pairs().map(|(a, b)| format!("{a}{b}")) => ["ab", "bc"]
//! [1, 2, 3].into_iter().interleave(vec![10, 20]).collect() => [1, 10, 2, 20, 3]
//! (0..3).interleave(100..105).collect()                => [0, 100, 1, 101, 2, 102, 103, 104]
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. In `Pairs::next`, fetch the first item lazily on the first call (`prev` starts as `None`).
//! 2. The supertrait `Iterator + Sized` lets default methods take `self` by value and name
//!    `Self::Item`.
//! 3. Blanket impls apply to types you did not write, including `std::ops::Range` and `Chars`.

/// Overlapping pairs of consecutive items.
pub struct Pairs<I: Iterator> {
    iter: I,
    prev: Option<I::Item>,
}

impl<I> Iterator for Pairs<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Write your solution here")
    }
}

/// Alternates between two iterators, starting with the first.
pub struct Interleave<A, B> {
    a: A,
    b: B,
    a_turn: bool,
}

impl<A, B> Iterator for Interleave<A, B>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
{
    type Item = A::Item;

    fn next(&mut self) -> Option<A::Item> {
        todo!("Write your solution here")
    }
}

/// Extra adapters for every iterator.
pub trait IterExt: Iterator + Sized {
    fn pairs(self) -> Pairs<Self>
    where
        Self::Item: Clone,
    {
        todo!("Write your solution here")
    }

    fn interleave<J>(self, other: J) -> Interleave<Self, J::IntoIter>
    where
        J: IntoIterator<Item = Self::Item>,
    {
        todo!("Write your solution here")
    }
}

impl<I: Iterator> IterExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs_of_numbers_and_chars() {
        assert_eq!(
            [1, 2, 3, 4].into_iter().pairs().collect::<Vec<_>>(),
            vec![(1, 2), (2, 3), (3, 4)]
        );
        let words: Vec<String> = "abc"
            .chars()
            .pairs()
            .map(|(a, b)| format!("{a}{b}"))
            .collect();
        assert_eq!(words, vec!["ab", "bc"]);
    }

    #[test]
    fn pairs_of_short_inputs() {
        assert_eq!([7].into_iter().pairs().count(), 0);
        assert_eq!(std::iter::empty::<u8>().pairs().count(), 0);
        let strings = vec!["x".to_string(), "y".to_string()];
        assert_eq!(
            strings.into_iter().pairs().collect::<Vec<_>>(),
            vec![("x".to_string(), "y".to_string())]
        );
    }

    #[test]
    fn interleaving() {
        assert_eq!(
            [1, 2, 3]
                .into_iter()
                .interleave(vec![10, 20])
                .collect::<Vec<_>>(),
            vec![1, 10, 2, 20, 3]
        );
        assert_eq!(
            (0..3).interleave(100..105).collect::<Vec<_>>(),
            vec![0, 100, 1, 101, 2, 102, 103, 104]
        );
        assert_eq!(
            std::iter::empty().interleave([5, 6]).collect::<Vec<i32>>(),
            vec![5, 6]
        );
    }

    #[test]
    fn adapters_compose() {
        let diffs: Vec<i64> = [1_i64, 4, 9, 16, 25]
            .into_iter()
            .pairs()
            .map(|(a, b)| b - a)
            .collect();
        assert_eq!(diffs, vec![3, 5, 7, 9]);
        let total: u64 = (0..10_000_u64)
            .interleave(10_000..20_000)
            .pairs()
            .map(|(a, _)| a)
            .sum();
        let expected: u64 = (0..10_000_u64)
            .interleave(10_000..20_000)
            .take(19_999)
            .sum();
        assert_eq!(total, expected);
    }
}
