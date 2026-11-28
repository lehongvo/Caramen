// ==================================================================
// PRACTICE 0484: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0484_interleave_iterators.md
// Run the tests:          cargo test practice::p0484
// ==================================================================

//! # Problem 0484: Interleave Two Iterators
//!
//! - **Difficulty:** Medium
//! - **Topics:** iterator struct generic over `A: Iterator` and `B: Iterator<Item = A::Item>`, `Iterator::fuse` / `Fuse`, `IntoIterator` parameters, `size_hint` arithmetic
//! - **Source:** Original · [std `Iterator::fuse`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fuse) · [std `IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
//!
//! ## Problem
//!
//! Iterator adapters are ordinary structs that wrap other iterators. Write one: `Interleave` takes
//! two iterators with the same item type and yields their items **alternately**, starting with the
//! first one. When one of them runs out, the rest of the other one follows.
//!
//! After an iterator has returned `None`, calling `next` on it again is allowed to do anything
//! (most iterators keep returning `None`, but nothing guarantees it). `iter.fuse()` wraps an
//! iterator in a `Fuse`, which guarantees `None` forever after the first `None`. The struct's
//! fields are already given: `a: Fuse<A>` and `b: Fuse<B>`.
//!
//! Implement:
//!
//! - `Interleave::new(a: A, b: B) -> Interleave<A, B>`.
//! - `impl Iterator for Interleave<A, B>` with `type Item = A::Item`, and a `size_hint` that adds up
//!   the hints of both sides (lower bounds with `saturating_add`, upper bounds with `checked_add` when
//!   both exist).
//! - `interleave<A, B>(a: A, b: B) -> Interleave<A::IntoIter, B::IntoIter>` where
//!   `A: IntoIterator` and `B: IntoIterator<Item = A::Item>`, so callers can pass vectors, ranges...
//!
//! ## Examples
//!
//! ```text
//! interleave(vec![1, 2, 3], vec![10, 20]).collect()   -> [1, 10, 2, 20, 3]
//! interleave(vec![1], 10..14).collect()               -> [1, 10, 11, 12, 13]
//! interleave("abc".chars(), "XY".chars()).collect::<String>()  -> "aXbYc"
//! interleave(1..=3, 7..=9).size_hint()                -> (6, Some(6))
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Keep a `bool` that says whose turn it is and flip it after every call.
//! 2. On A's turn: `self.a.next().or_else(|| self.b.next())`, and the mirror image on B's turn.
//! 3. `A::IntoIter` is the iterator type an `A: IntoIterator` turns into.

use std::iter::Fuse;

pub struct Interleave<A, B> {
    a: Fuse<A>,
    b: Fuse<B>,
}

impl<A, B> Interleave<A, B>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
{
    pub fn new(a: A, b: B) -> Interleave<A, B> {
        todo!("Write your solution here")
    }
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        todo!("Write your solution here")
    }
}

pub fn interleave<A, B>(a: A, b: B) -> Interleave<A::IntoIter, B::IntoIter>
where
    A: IntoIterator,
    B: IntoIterator<Item = A::Item>,
{
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alternate_then_finish_the_longer_side() {
        assert_eq!(
            interleave(vec![1, 2, 3], vec![10, 20]).collect::<Vec<_>>(),
            vec![1, 10, 2, 20, 3]
        );
        assert_eq!(
            interleave(vec![1], 10..14).collect::<Vec<_>>(),
            vec![1, 10, 11, 12, 13]
        );
    }

    #[test]
    fn works_with_any_iterators() {
        assert_eq!(
            interleave("abc".chars(), "XY".chars()).collect::<String>(),
            "aXbYc"
        );
        let words = ["one", "two"];
        assert_eq!(
            Interleave::new(words.iter(), ["uno", "dos"].iter())
                .copied()
                .collect::<Vec<_>>(),
            vec!["one", "uno", "two", "dos"]
        );
    }

    #[test]
    fn empty_sides() {
        assert_eq!(
            interleave(Vec::<i32>::new(), vec![1, 2]).collect::<Vec<_>>(),
            vec![1, 2]
        );
        assert_eq!(
            interleave(vec![1, 2], Vec::<i32>::new()).collect::<Vec<_>>(),
            vec![1, 2]
        );
        assert_eq!(
            interleave(Vec::<i32>::new(), Vec::<i32>::new()).next(),
            None
        );
    }

    #[test]
    fn size_hints_add_up() {
        assert_eq!(interleave(1..=3, 7..=9).size_hint(), (6, Some(6)));
        let mut it = interleave(0..5, 0..2);
        it.next();
        assert_eq!(it.size_hint(), (6, Some(6)));
        let unbounded = interleave(0.., 0..3);
        assert_eq!(unbounded.size_hint().1, None);
    }

    #[test]
    fn infinite_side_with_take() {
        let evens = (0..).step_by(2);
        let odds = (1..).step_by(2);
        assert_eq!(
            interleave(evens, odds).take(6).collect::<Vec<u32>>(),
            vec![0, 1, 2, 3, 4, 5]
        );
    }
}
