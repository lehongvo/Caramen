//! # Problem 0954: Find Median from Data Stream
//!
//! - **Difficulty:** Hard
//! - **Topics:** two heaps, `BinaryHeap` as a max-heap and with `std::cmp::Reverse` as a min-heap, invariants between structures, `f64` results
//! - **Source:** [LeetCode 295. Find Median from Data Stream](https://leetcode.com/problems/find-median-from-data-stream/)
//!
//! ## Problem
//!
//! Numbers arrive one by one; at any time you must be able to report the **median** of all numbers
//! seen so far: the middle value of the sorted list, or the mean of the two middle values when the
//! count is even.
//!
//! Implement the `MedianFinder` struct:
//!
//! - `MedianFinder::new() -> Self`.
//! - `add_num(&mut self, num: i32)`.
//! - `find_median(&self) -> f64` (called only after at least one number was added).
//!
//! ## Examples
//!
//! ```text
//! add_num(1); add_num(2);
//! find_median()   => 1.5
//! add_num(3);
//! find_median()   => 2.0
//! ```
//!
//! ## Constraints
//!
//! - `-10^5 <= num <= 10^5`
//! - At most `5 * 10^4` calls in total.
//!
//! ## Hints
//!
//! 1. Keep the smaller half in a max-heap and the larger half in a min-heap.
//! 2. Invariant: every element of the lower half is `<=` every element of the upper half, and the
//!    lower half has the same size as the upper half or one more.
//! 3. To insert, push into the lower half, move its maximum to the upper half, and move one element
//!    back if the upper half became larger.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** with the two halves balanced as in the hints, the median is the top of the lower heap
//! (odd count) or the mean of both tops (even count). Each insertion performs at most three heap
//! operations and restores both invariants.
//!
//! **Complexity:** `add_num` is `O(log n)`, `find_median` is `O(1)`.
//!
//! **Rust concepts:**
//! - `BinaryHeap<T>` is a max-heap; `BinaryHeap<Reverse<T>>` turns it into a min-heap.
//! - `peek()` returns `Option<&T>`; with the size invariant, the lower heap is never empty once a
//!   number has been added.
//! - Convert to `f64` before adding the two middle values, so the mean is exact.
// @solution-end

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct MedianFinder {
    // @solution-start remove
    lower: BinaryHeap<i32>,
    upper: BinaryHeap<Reverse<i32>>,
    // @solution-end
}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl MedianFinder {
    pub fn new() -> Self {
        // @solution-start
        MedianFinder {
            lower: BinaryHeap::new(),
            upper: BinaryHeap::new(),
        }
        // @solution-end
    }

    pub fn add_num(&mut self, num: i32) {
        // @solution-start
        self.lower.push(num);
        if let Some(top) = self.lower.pop() {
            self.upper.push(Reverse(top));
        }
        if self.upper.len() > self.lower.len() {
            if let Some(Reverse(smallest)) = self.upper.pop() {
                self.lower.push(smallest);
            }
        }
        // @solution-end
    }

    pub fn find_median(&self) -> f64 {
        // @solution-start
        match (self.lower.peek(), self.upper.peek()) {
            (Some(&low), Some(&Reverse(high))) if self.lower.len() == self.upper.len() => {
                (f64::from(low) + f64::from(high)) / 2.0
            }
            (Some(&low), _) => f64::from(low),
            (None, _) => 0.0,
        }
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut m = MedianFinder::new();
        m.add_num(1);
        m.add_num(2);
        assert_eq!(m.find_median(), 1.5);
        m.add_num(3);
        assert_eq!(m.find_median(), 2.0);
    }

    #[test]
    fn negative_and_repeated_values() {
        let mut m = MedianFinder::new();
        m.add_num(-100_000);
        assert_eq!(m.find_median(), -100_000.0);
        m.add_num(-100_000);
        assert_eq!(m.find_median(), -100_000.0);
        m.add_num(5);
        m.add_num(6);
        assert_eq!(m.find_median(), -49_997.5);
    }

    #[test]
    fn matches_sorting_after_each_insert() {
        let mut m = MedianFinder::new();
        let mut seen = Vec::new();
        for i in 0..2_000_i32 {
            let x = (i * 7_919) % 2_001 - 1_000;
            m.add_num(x);
            seen.push(x);
            seen.sort_unstable();
            let n = seen.len();
            let expected = if n % 2 == 1 {
                f64::from(seen[n / 2])
            } else {
                (f64::from(seen[n / 2 - 1]) + f64::from(seen[n / 2])) / 2.0
            };
            assert_eq!(m.find_median(), expected, "after {n} numbers");
        }
    }

    #[test]
    fn many_numbers() {
        let mut m = MedianFinder::new();
        for x in (1..=50_000).rev() {
            m.add_num(x);
        }
        assert_eq!(m.find_median(), 25_000.5);
    }
}
