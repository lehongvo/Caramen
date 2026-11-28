// ==================================================================
// PRACTICE 0476: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0476_counter_iterator.md
// Run the tests:          cargo test practice::p0476
// ==================================================================

//! # Problem 0476: A Counter Iterator
//!
//! - **Difficulty:** Easy
//! - **Topics:** `impl Iterator for` a struct, associated type `Item`, `next(&mut self)`, getting every adapter for free
//! - **Source:** Original · [The Rust Book 13.2: The `Iterator` trait and the `next` method](https://doc.rust-lang.org/book/ch13-02-iterators.html#the-iterator-trait-and-the-next-method) · [std `iter`: Implementing Iterator](https://doc.rust-lang.org/std/iter/index.html#implementing-iterator)
//!
//! ## Problem
//!
//! Any type can become an iterator by implementing the `Iterator` trait, which has one required
//! method:
//!
//! ```text
//! impl Iterator for Counter {
//!     type Item = u32;
//!     fn next(&mut self) -> Option<Self::Item> { ... }
//! }
//! ```
//!
//! Every other method (`map`, `filter`, `zip`, `sum`, ...) has a default implementation built on
//! `next`, so your type gets them all for free.
//!
//! Implement:
//!
//! - `Counter::new(limit: u32) -> Counter`: an iterator over `1, 2, ..., limit` (nothing at all when
//!   `limit == 0`). After the last number, `next` keeps returning `None`.
//! - `impl Iterator for Counter` with `type Item = u32`.
//! - `sum_of_products(limit: u32) -> u32`: pair each number of `Counter::new(limit)` with the next
//!   one (`(1, 2), (2, 3), ...`), multiply each pair, keep the products divisible by 3 and add them up.
//! - `squares(limit: u32) -> Vec<u32>`: the squares of the counter's numbers.
//!
//! ## Examples
//!
//! ```text
//! Counter::new(3) yields 1, 2, 3, then None, None, ...
//! Counter::new(0) yields nothing
//! sum_of_products(5)   -> 18     (pairs (1,2) (2,3) (3,4) (4,5) -> products 2, 6, 12, 20 -> 6 + 12)
//! squares(4)           -> [1, 4, 9, 16]
//! ```
//!
//! ## Constraints
//!
//! - `limit <= 10^4`.
//!
//! ## Hints
//!
//! 1. Store the last number produced and the limit; `next` increments and compares.
//! 2. Pair a counter with a second counter that skipped its first item:
//!    `Counter::new(limit).zip(Counter::new(limit).skip(1))`.

pub struct Counter {}

impl Counter {
    pub fn new(limit: u32) -> Counter {
        todo!("Write your solution here")
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Write your solution here")
    }
}

pub fn sum_of_products(limit: u32) -> u32 {
    todo!("Write your solution here")
}

pub fn squares(limit: u32) -> Vec<u32> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_to_the_limit_and_stays_done() {
        let mut c = Counter::new(3);
        assert_eq!(c.next(), Some(1));
        assert_eq!(c.next(), Some(2));
        assert_eq!(c.next(), Some(3));
        assert_eq!(c.next(), None);
        assert_eq!(c.next(), None);
    }

    #[test]
    fn empty_counter() {
        assert_eq!(Counter::new(0).next(), None);
        assert_eq!(Counter::new(0).count(), 0);
    }

    #[test]
    fn adapters_work_on_counter() {
        assert_eq!(Counter::new(5).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
        assert_eq!(Counter::new(10).filter(|n| n % 2 == 0).count(), 5);
        assert_eq!(Counter::new(100).sum::<u32>(), 5050);
        assert_eq!(Counter::new(4).rev_sum(), 10);
    }

    #[test]
    fn sum_of_products_examples() {
        assert_eq!(sum_of_products(5), 18);
        assert_eq!(sum_of_products(1), 0);
        assert_eq!(sum_of_products(0), 0);
        let expected: u32 = (1..100).map(|a| a * (a + 1)).filter(|x| x % 3 == 0).sum();
        assert_eq!(sum_of_products(100), expected);
    }

    #[test]
    fn squares_of_counter() {
        assert_eq!(squares(4), vec![1, 4, 9, 16]);
        assert_eq!(squares(0), Vec::<u32>::new());
        assert_eq!(squares(10_000).last(), Some(&100_000_000));
    }

    /// A helper trait used only in the tests: it shows that a generic method written for *any*
    /// iterator also works for `Counter`.
    trait RevSum {
        fn rev_sum(self) -> u32;
    }

    impl<I: Iterator<Item = u32>> RevSum for I {
        fn rev_sum(self) -> u32 {
            let all: Vec<u32> = self.collect();
            all.into_iter().rev().sum()
        }
    }
}
