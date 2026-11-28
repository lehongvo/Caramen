// ==================================================================
// PRACTICE 0386: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0386_smallest_number_in_infinite_set.md
// Run the tests:          cargo test practice::p0386
// ==================================================================

//! # Problem 0386: Smallest Number in Infinite Set
//!
//! - **Difficulty:** Medium
//! - **Topics:** `BTreeSet::pop_first`, representing an infinite set with a counter plus a small set, design struct
//! - **Source:** [LeetCode 2336. Smallest Number in Infinite Set](https://leetcode.com/problems/smallest-number-in-infinite-set/)
//!
//! ## Problem
//!
//! A set initially contains **every** positive integer `1, 2, 3, ...`. Implement the
//! `SmallestInfiniteSet` struct:
//!
//! - `SmallestInfiniteSet::new() -> Self`: the full set of positive integers;
//! - `pop_smallest(&mut self) -> i32`: remove the smallest number in the set and return it;
//! - `add_back(&mut self, num: i32)`: put `num` back into the set, if it is not already there.
//!
//! The mutating methods take `&mut self` instead of the template's `&self`, as LeetCode's comment
//! allows.
//!
//! ## Examples
//!
//! ```text
//! let mut s = SmallestInfiniteSet::new();
//! s.add_back(2);          (2 is already in the set: nothing changes)
//! s.pop_smallest() => 1
//! s.pop_smallest() => 2
//! s.pop_smallest() => 3
//! s.add_back(1);          (1 is back)
//! s.pop_smallest() => 1
//! s.pop_smallest() => 4
//! s.pop_smallest() => 5
//! ```
//!
//! ## Constraints
//!
//! - `1 <= num <= 1000`
//! - At most `1000` calls in total.
//!
//! ## Hints
//!
//! 1. You cannot store infinitely many numbers, but you can store "every number `>= next` is
//!    present" as a single integer `next`.
//! 2. Numbers smaller than `next` that were added back go into a `BTreeSet`, whose smallest element
//!    is always popped first.

use std::collections::BTreeSet;

/// All positive integers, with removals and re-insertions.
pub struct SmallestInfiniteSet {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl SmallestInfiniteSet {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn pop_smallest(&mut self) -> i32 {
        todo!("Write your solution here")
    }

    pub fn add_back(&mut self, num: i32) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut s = SmallestInfiniteSet::new();
        s.add_back(2);
        assert_eq!(s.pop_smallest(), 1);
        assert_eq!(s.pop_smallest(), 2);
        assert_eq!(s.pop_smallest(), 3);
        s.add_back(1);
        assert_eq!(s.pop_smallest(), 1);
        assert_eq!(s.pop_smallest(), 4);
        assert_eq!(s.pop_smallest(), 5);
    }

    #[test]
    fn repeated_add_back_counts_once() {
        let mut s = SmallestInfiniteSet::new();
        assert_eq!(s.pop_smallest(), 1);
        s.add_back(1);
        s.add_back(1);
        assert_eq!(s.pop_smallest(), 1);
        assert_eq!(s.pop_smallest(), 2);
    }

    #[test]
    fn added_back_numbers_come_out_sorted() {
        let mut s = SmallestInfiniteSet::new();
        for _ in 0..10 {
            s.pop_smallest();
        }
        s.add_back(7);
        s.add_back(3);
        s.add_back(9);
        let popped: Vec<i32> = (0..4).map(|_| s.pop_smallest()).collect();
        assert_eq!(popped, vec![3, 7, 9, 11]);
    }

    #[test]
    fn many_calls() {
        let mut s = SmallestInfiniteSet::new();
        for expected in 1..=500 {
            assert_eq!(s.pop_smallest(), expected);
        }
        for num in (1..=500).rev().step_by(2) {
            s.add_back(num);
        }
        // 500, 498, ..., 2 were added back: they come out in increasing order.
        assert_eq!(s.pop_smallest(), 2);
        assert_eq!(s.pop_smallest(), 4);
    }
}
