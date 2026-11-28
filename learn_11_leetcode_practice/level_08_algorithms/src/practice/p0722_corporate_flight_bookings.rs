// ==================================================================
// PRACTICE 0722: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0722_corporate_flight_bookings.md
// Run the tests:          cargo test practice::p0722
// ==================================================================

//! # Problem 0722: Corporate Flight Bookings
//!
//! - **Difficulty:** Medium
//! - **Topics:** difference array, prefix sums with `scan`, slice patterns `[a, b, c]`
//! - **Source:** [LeetCode 1109. Corporate Flight Bookings](https://leetcode.com/problems/corporate-flight-bookings/)
//!
//! ## Problem
//!
//! There are `n` flights numbered `1` to `n`. Each booking `[first, last, seats]` reserves `seats`
//! seats on **every** flight from `first` to `last` (inclusive). Return a vector `answer` of length
//! `n` where `answer[i]` is the total number of seats reserved on flight `i + 1`.
//!
//! Implement `Solution::corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  bookings = [[1, 2, 10], [2, 3, 20], [2, 5, 25]], n = 5
//! Output: [10, 55, 45, 25, 25]
//!
//!   flight:     1   2   3   4   5
//!   booking 1: 10  10
//!   booking 2:     20  20
//!   booking 3:     25  25  25  25
//!
//! Input:  bookings = [[1, 2, 10], [2, 2, 15]], n = 2
//! Output: [10, 25]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 2 * 10^4`
//! - `1 <= bookings.len() <= 2 * 10^4`
//! - `bookings[i].len() == 3` and `1 <= first <= last <= n`
//! - `1 <= seats <= 10^4`
//!
//! ## Hints
//!
//! 1. Adding `seats` to every flight of every booking is `O(n)` per booking: too slow in the worst
//!    case.
//! 2. Record only where each booking **starts** and where it **stops** contributing, then compute
//!    running totals.

pub struct Solution;

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_bookings() {
        let bookings = vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]];
        assert_eq!(
            Solution::corp_flight_bookings(bookings, 5),
            vec![10, 55, 45, 25, 25]
        );
    }

    #[test]
    fn example_two_flights() {
        let bookings = vec![vec![1, 2, 10], vec![2, 2, 15]];
        assert_eq!(Solution::corp_flight_bookings(bookings, 2), vec![10, 25]);
    }

    #[test]
    fn single_flight() {
        assert_eq!(
            Solution::corp_flight_bookings(vec![vec![1, 1, 7]], 1),
            vec![7]
        );
    }

    #[test]
    fn flights_without_bookings_stay_zero() {
        let bookings = vec![vec![2, 3, 5]];
        assert_eq!(
            Solution::corp_flight_bookings(bookings, 5),
            vec![0, 5, 5, 0, 0]
        );
    }

    #[test]
    fn largest_totals() {
        let n = 20_000;
        let bookings = vec![vec![1, n, 10_000]; 20_000];
        let answer = Solution::corp_flight_bookings(bookings, n);
        assert_eq!(answer.len(), 20_000);
        assert!(answer.iter().all(|&seats| seats == 200_000_000));
    }
}
