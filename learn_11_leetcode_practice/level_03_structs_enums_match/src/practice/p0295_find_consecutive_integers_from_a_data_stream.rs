// ==================================================================
// PRACTICE 0295: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0295_find_consecutive_integers_from_a_data_stream.md
// Run the tests:          cargo test practice::p0295
// ==================================================================

//! # Problem 0295: Find Consecutive Integers from a Data Stream
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct holding streaming state, `&mut self` update method, constant memory instead of storing the stream
//! - **Source:** [LeetCode 2526. Find Consecutive Integers from a Data Stream](https://leetcode.com/problems/find-consecutive-integers-from-a-data-stream/)
//!
//! ## Problem
//!
//! Integers arrive one at a time. Given a target `value` and a window size `k`, after each new
//! integer report whether **the last `k` integers** received are all equal to `value`. While fewer
//! than `k` integers have arrived, the answer is `false`.
//!
//! Implement the `DataStream` struct:
//!
//! - `DataStream::new(value: i32, k: i32) -> Self`;
//! - `consec(&mut self, num: i32) -> bool`: add `num` to the stream and return the answer.
//!
//! This is a LeetCode *design* problem, so there is no `impl Solution`.
//!
//! ## Examples
//!
//! ```text
//! let mut s = DataStream::new(4, 3);
//! s.consec(4) => false   (only one number so far)
//! s.consec(4) => false   (only two)
//! s.consec(4) => true    ([4, 4, 4])
//! s.consec(3) => false   ([4, 4, 3])
//! ```
//!
//! ## Constraints
//!
//! - `1 <= value, num <= 10^9`, `1 <= k <= 10^5`; at most `10^5` calls.
//!
//! ## Hints
//!
//! 1. You do not need to store the numbers: only how long the current run of `value` is.

/// Watches a stream for `k` consecutive copies of `value`.
pub struct DataStream {}

impl DataStream {
    /// A detector for `k` consecutive copies of `value`.
    pub fn new(value: i32, k: i32) -> Self {
        todo!("Write your solution here")
    }

    /// Adds `num`; `true` if the last `k` numbers all equal `value`.
    pub fn consec(&mut self, num: i32) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_session() {
        let mut s = DataStream::new(4, 3);
        assert!(!s.consec(4));
        assert!(!s.consec(4));
        assert!(s.consec(4));
        assert!(!s.consec(3));
    }

    #[test]
    fn window_of_one() {
        let mut s = DataStream::new(7, 1);
        assert!(s.consec(7));
        assert!(!s.consec(8));
        assert!(s.consec(7));
    }

    #[test]
    fn streak_continues_past_k() {
        let mut s = DataStream::new(1, 2);
        let answers: Vec<bool> = [1, 1, 1, 1, 2, 1, 1].iter().map(|&n| s.consec(n)).collect();
        assert_eq!(answers, vec![false, true, true, true, false, false, true]);
    }

    #[test]
    fn long_stream() {
        let mut s = DataStream::new(1_000_000_000, 100_000);
        for _ in 0..99_999 {
            assert!(!s.consec(1_000_000_000));
        }
        assert!(s.consec(1_000_000_000));
    }
}
