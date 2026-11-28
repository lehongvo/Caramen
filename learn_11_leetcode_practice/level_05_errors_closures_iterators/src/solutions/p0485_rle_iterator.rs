//! # Problem 0485: RLE Iterator
//!
//! - **Difficulty:** Medium
//! - **Topics:** LeetCode design struct, iterator-like state (`index`, `used`), consuming counts, `i64` to avoid overflow
//! - **Source:** [LeetCode 900. RLE Iterator](https://leetcode.com/problems/rle-iterator/)
//!
//! ## Problem
//!
//! A sequence is stored *run-length encoded* in an even-length vector `encoding`: for every even
//! index `i`, the value `encoding[i + 1]` is repeated `encoding[i]` times. For example
//! `[3, 8, 0, 9, 2, 5]` encodes `[8, 8, 8, 5, 5]` (the run of `9` has length `0`).
//!
//! Design `RLEIterator`, which walks through the encoded sequence without expanding it:
//!
//! - `RLEIterator::new(encoding: Vec<i32>) -> RLEIterator`;
//! - `next(&mut self, n: i32) -> i32`: consume the next `n` elements and return the **last** one
//!   consumed. If fewer than `n` elements remain, consume all of them and return `-1`.
//!
//! This is a LeetCode *design* problem: LeetCode calls `new` and then the methods. Its template
//! declares `next(&self, ...)`; here it takes `&mut self` because it changes the iterator, which
//! LeetCode accepts.
//!
//! ## Examples
//!
//! ```text
//! let mut it = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);    // [8, 8, 8, 5, 5]
//! it.next(2) -> 8     consumes 8, 8           remaining [8, 5, 5]
//! it.next(1) -> 8     consumes 8              remaining [5, 5]
//! it.next(1) -> 5     consumes 5              remaining [5]
//! it.next(2) -> -1    only one element left; it is consumed anyway
//! ```
//!
//! ## Constraints
//!
//! - `2 <= encoding.len() <= 1000`, `encoding.len()` is even
//! - `0 <= encoding[i] <= 10^9`
//! - `1 <= n <= 10^9`
//! - At most `1000` calls to `next`.
//!
//! ## Hints
//!
//! 1. Keep the index of the current run and how many elements of that run are already used.
//! 2. While `n` is larger than what is left in the current run, subtract the rest of the run and move
//!    to the next run (skipping runs of length `0` naturally).
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** never expand the runs. Each call walks forward over whole runs while `n` exceeds what
//! is left of the current run, then takes the remaining part from one run and returns its value.
//! Running past the last run means there were not enough elements.
//!
//! **Complexity:** `O(runs)` time in total over all calls (each run is passed at most once), `O(1)`
//! extra space.
//!
//! **Rust concepts:**
//! - The struct is an iterator in spirit; it does not implement `Iterator` because `next` takes an
//!   argument and returns a sentinel (`-1`) instead of `Option`, as LeetCode requires.
//! - Counts and `n` are converted to `i64`: `used + n` could exceed `i32::MAX`.
//! - `&mut self` is required to change `index` and `used`.
// @solution-end

pub struct RLEIterator {
    // @solution-start remove
    encoding: Vec<i32>,
    index: usize,
    used: i64,
    // @solution-end
}

impl RLEIterator {
    pub fn new(encoding: Vec<i32>) -> Self {
        // @solution-start
        RLEIterator {
            encoding,
            index: 0,
            used: 0,
        }
        // @solution-end
    }

    pub fn next(&mut self, n: i32) -> i32 {
        // @solution-start
        let mut n = i64::from(n);
        while self.index + 1 < self.encoding.len() {
            let left = i64::from(self.encoding[self.index]) - self.used;
            if n <= left {
                self.used += n;
                return self.encoding[self.index + 1];
            }
            n -= left;
            self.index += 2;
            self.used = 0;
        }
        -1
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut it = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);
        assert_eq!(it.next(2), 8);
        assert_eq!(it.next(1), 8);
        assert_eq!(it.next(1), 5);
        assert_eq!(it.next(2), -1);
    }

    #[test]
    fn exhausted_iterator_keeps_returning_minus_one() {
        let mut it = RLEIterator::new(vec![1, 4]);
        assert_eq!(it.next(1), 4);
        assert_eq!(it.next(1), -1);
        assert_eq!(it.next(5), -1);
    }

    #[test]
    fn zero_length_runs_are_skipped() {
        let mut it = RLEIterator::new(vec![0, 1, 0, 2, 2, 3, 0, 4, 1, 5]);
        assert_eq!(it.next(2), 3);
        assert_eq!(it.next(1), 5);
        assert_eq!(it.next(1), -1);
    }

    #[test]
    fn huge_counts_do_not_overflow() {
        let mut it = RLEIterator::new(vec![1_000_000_000, 7, 1_000_000_000, 9]);
        assert_eq!(it.next(999_999_999), 7);
        assert_eq!(it.next(1_000_000_000), 9);
        assert_eq!(it.next(1_000_000_000), -1);
    }
}
