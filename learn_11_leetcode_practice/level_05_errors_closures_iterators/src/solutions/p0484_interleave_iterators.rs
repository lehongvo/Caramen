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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** remember whose turn it is. Each call asks the current side first and falls back to the
//! other side when the current one is exhausted; the turn flips every time. Fusing both sides makes
//! "exhausted" a permanent state, so the fallback logic stays correct.
//!
//! **Rust concepts:**
//! - The bound `B: Iterator<Item = A::Item>` says "B yields the same type as A".
//! - `Fuse<I>` is the adapter returned by `fuse()`; storing it in a field is how adapters compose.
//! - `or_else` borrows `self.b` inside a closure while `self.a` was just used: closures capture
//!   disjoint fields, so the borrow checker accepts it.
//! - Taking `IntoIterator` parameters (like `zip` and `chain` do) makes the API pleasant to call.
// @solution-end

use std::iter::Fuse;

pub struct Interleave<A, B> {
    a: Fuse<A>,
    b: Fuse<B>,
    // @solution-start remove
    a_turn: bool,
    // @solution-end
}

impl<A, B> Interleave<A, B>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
{
    pub fn new(a: A, b: B) -> Interleave<A, B> {
        // @solution-start
        Interleave {
            a: a.fuse(),
            b: b.fuse(),
            a_turn: true,
        }
        // @solution-end
    }
}

impl<A, B> Iterator for Interleave<A, B>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
{
    type Item = A::Item;

    fn next(&mut self) -> Option<A::Item> {
        // @solution-start
        let item = if self.a_turn {
            self.a.next().or_else(|| self.b.next())
        } else {
            self.b.next().or_else(|| self.a.next())
        };
        self.a_turn = !self.a_turn;
        item
        // @solution-end
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // @solution-start
        let (a_low, a_high) = self.a.size_hint();
        let (b_low, b_high) = self.b.size_hint();
        let high = a_high.zip(b_high).and_then(|(x, y)| x.checked_add(y));
        (a_low.saturating_add(b_low), high)
        // @solution-end
    }
}

pub fn interleave<A, B>(a: A, b: B) -> Interleave<A::IntoIter, B::IntoIter>
where
    A: IntoIterator,
    B: IntoIterator<Item = A::Item>,
{
    // @solution-start
    Interleave::new(a.into_iter(), b.into_iter())
    // @solution-end
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
