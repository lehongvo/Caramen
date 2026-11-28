// ==================================================================
// PRACTICE 0955: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0955_data_stream_as_disjoint_intervals.md
// Run the tests:          cargo test practice::p0955
// ==================================================================

//! # Problem 0955: Data Stream as Disjoint Intervals
//!
//! - **Difficulty:** Hard
//! - **Topics:** `BTreeMap` as an interval set, `range(..=x).next_back()` to find the predecessor, merging neighbours, `remove` + `insert`
//! - **Source:** [LeetCode 352. Data Stream as Disjoint Intervals](https://leetcode.com/problems/data-stream-as-disjoint-intervals/)
//!
//! ## Problem
//!
//! Non-negative integers arrive one at a time. At any moment, summarise the distinct numbers seen so
//! far as a sorted list of disjoint, **maximal** closed intervals `[start, end]`: consecutive numbers
//! belong to the same interval.
//!
//! Implement the `SummaryRanges` struct:
//!
//! - `SummaryRanges::new() -> Self`.
//! - `add_num(&mut self, value: i32)`: adds `value` (adding a number twice changes nothing).
//! - `get_intervals(&self) -> Vec<Vec<i32>>`: the intervals as `[start, end]` pairs, sorted by start.
//!
//! ## Examples
//!
//! ```text
//! add_num(1)       => intervals [[1, 1]]
//! add_num(3)       => [[1, 1], [3, 3]]
//! add_num(7)       => [[1, 1], [3, 3], [7, 7]]
//! add_num(2)       => [[1, 3], [7, 7]]
//! add_num(6)       => [[1, 3], [6, 7]]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= value <= 10^4`
//! - At most `3 * 10^4` calls to `add_num` and at most `10^2` calls to `get_intervals`.
//!
//! ## Hints
//!
//! 1. Store the intervals in a `BTreeMap<start, end>`.
//! 2. The only interval that can contain `value` or end at `value - 1` is the one with the largest
//!    start `<= value`; the only one that can start at `value + 1` is `map.get(&(value + 1))`.
//! 3. Four cases: already covered, join both neighbours, extend one neighbour, or a new `[v, v]`.

use std::collections::BTreeMap;

pub struct SummaryRanges {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl SummaryRanges {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn add_num(&mut self, value: i32) {
        todo!("Write your solution here")
    }

    pub fn get_intervals(&self) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut r = SummaryRanges::new();
        let mut snapshots = Vec::new();
        for v in [1, 3, 7, 2, 6] {
            r.add_num(v);
            snapshots.push(r.get_intervals());
        }
        assert_eq!(
            snapshots,
            vec![
                vec![vec![1, 1]],
                vec![vec![1, 1], vec![3, 3]],
                vec![vec![1, 1], vec![3, 3], vec![7, 7]],
                vec![vec![1, 3], vec![7, 7]],
                vec![vec![1, 3], vec![6, 7]],
            ]
        );
    }

    #[test]
    fn duplicates_and_zero() {
        let mut r = SummaryRanges::new();
        for v in [0, 0, 2, 2, 1, 1] {
            r.add_num(v);
        }
        assert_eq!(r.get_intervals(), vec![vec![0, 2]]);
    }

    #[test]
    fn values_inside_existing_intervals() {
        let mut r = SummaryRanges::new();
        for v in [5, 6, 7, 8, 6, 10] {
            r.add_num(v);
        }
        assert_eq!(r.get_intervals(), vec![vec![5, 8], vec![10, 10]]);
        r.add_num(9);
        assert_eq!(r.get_intervals(), vec![vec![5, 10]]);
    }

    #[test]
    fn many_values() {
        let mut r = SummaryRanges::new();
        for v in (0..=10_000).step_by(2) {
            r.add_num(v);
        }
        assert_eq!(r.get_intervals().len(), 5_001);
        for v in (1..10_000).step_by(2) {
            r.add_num(v);
        }
        assert_eq!(r.get_intervals(), vec![vec![0, 10_000]]);
    }
}
