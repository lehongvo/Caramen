// ==================================================================
// PRACTICE 0394: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0394_seat_reservation_manager.md
// Run the tests:          cargo test practice::p0394
// ==================================================================

//! # Problem 0394: Seat Reservation Manager
//!
//! - **Difficulty:** Medium
//! - **Topics:** `BinaryHeap<Reverse<i32>>` as a min-heap, collecting a range into a heap, design struct
//! - **Source:** [LeetCode 1845. Seat Reservation Manager](https://leetcode.com/problems/seat-reservation-manager/)
//!
//! ## Problem
//!
//! A theatre has `n` seats numbered `1` to `n`, all free at the start. Implement the `SeatManager`
//! struct:
//!
//! - `SeatManager::new(n: i32) -> Self`: `n` free seats;
//! - `reserve(&mut self) -> i32`: reserve the **smallest-numbered** free seat and return its number;
//! - `unreserve(&mut self, seat_number: i32)`: free the seat `seat_number` again.
//!
//! `reserve` is only called when a free seat exists, and `unreserve` only for a reserved seat. The
//! mutating methods take `&mut self` instead of the template's `&self`, as LeetCode's comment
//! allows.
//!
//! ## Examples
//!
//! ```text
//! let mut m = SeatManager::new(5);
//! m.reserve()     => 1
//! m.reserve()     => 2
//! m.unreserve(2);
//! m.reserve()     => 2
//! m.reserve()     => 3
//! m.reserve()     => 4
//! m.reserve()     => 5
//! m.unreserve(5);
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 10^5`, `1 <= seat_number <= n`
//! - At most `10^5` calls in total; every call is valid.
//!
//! ## Hints
//!
//! 1. You need "remove the smallest" and "insert any" quickly: a min-heap.
//! 2. `BinaryHeap` is a max-heap; store `Reverse(seat)` to flip the order.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Hands out the smallest free seat number.
pub struct SeatManager {}

impl SeatManager {
    pub fn new(n: i32) -> Self {
        todo!("Write your solution here")
    }

    pub fn reserve(&mut self) -> i32 {
        todo!("Write your solution here")
    }

    pub fn unreserve(&mut self, seat_number: i32) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut m = SeatManager::new(5);
        assert_eq!(m.reserve(), 1);
        assert_eq!(m.reserve(), 2);
        m.unreserve(2);
        assert_eq!(m.reserve(), 2);
        assert_eq!(m.reserve(), 3);
        assert_eq!(m.reserve(), 4);
        assert_eq!(m.reserve(), 5);
        m.unreserve(5);
        assert_eq!(m.reserve(), 5);
    }

    #[test]
    fn single_seat() {
        let mut m = SeatManager::new(1);
        assert_eq!(m.reserve(), 1);
        m.unreserve(1);
        assert_eq!(m.reserve(), 1);
    }

    #[test]
    fn unreserved_seats_come_back_smallest_first() {
        let mut m = SeatManager::new(10);
        for _ in 0..10 {
            m.reserve();
        }
        m.unreserve(7);
        m.unreserve(3);
        m.unreserve(9);
        assert_eq!(m.reserve(), 3);
        assert_eq!(m.reserve(), 7);
        assert_eq!(m.reserve(), 9);
    }

    #[test]
    fn large_theatre() {
        let mut m = SeatManager::new(100_000);
        for expected in 1..=50_000 {
            assert_eq!(m.reserve(), expected);
        }
        m.unreserve(12_345);
        assert_eq!(m.reserve(), 12_345);
        assert_eq!(m.reserve(), 50_001);
    }
}
