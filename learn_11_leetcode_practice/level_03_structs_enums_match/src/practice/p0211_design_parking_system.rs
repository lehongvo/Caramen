// ==================================================================
// PRACTICE 0211: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0211_design_parking_system.md
// Run the tests:          cargo test practice::p0211
// ==================================================================

//! # Problem 0211: Design Parking System
//!
//! - **Difficulty:** Easy
//! - **Topics:** LeetCode "design" problems, `new` constructor, `&mut self`, arrays as fields
//! - **Source:** [LeetCode 1603. Design Parking System](https://leetcode.com/problems/design-parking-system/)
//!
//! ## Problem
//!
//! A parking lot has three kinds of spaces: **big**, **medium** and **small**, with a fixed number of
//! spaces of each kind. Cars arrive one after another. A car of type `1` needs a big space, type `2`
//! a medium space, and type `3` a small space; a car can only park in a space of its own kind.
//!
//! Implement the `ParkingSystem` struct:
//!
//! - `ParkingSystem::new(big: i32, medium: i32, small: i32) -> Self` creates a lot with that many
//!   free spaces of each kind.
//! - `add_car(&mut self, car_type: i32) -> bool` parks the car if a free space of its kind exists
//!   (the space is then taken for good) and returns `true`; otherwise returns `false`.
//!
//! This is a LeetCode *design* problem, so there is no `impl Solution`: LeetCode creates the object
//! with `new` and then calls its methods. LeetCode's template writes the struct and methods without `pub` and every method with
//! `&self`; here they are `pub` (still accepted by LeetCode), and `add_car` takes `&mut self`
//! because it changes the lot, as the template's comment allows.
//!
//! ## Examples
//!
//! ```text
//! let mut lot = ParkingSystem::new(1, 1, 0);
//! lot.add_car(1) => true    (the only big space is taken)
//! lot.add_car(2) => true    (the only medium space is taken)
//! lot.add_car(3) => false   (there are no small spaces)
//! lot.add_car(1) => false   (no big space left)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= big, medium, small <= 1000`
//! - `car_type` is `1`, `2` or `3`.
//! - At most `1000` calls to `add_car`.
//!
//! ## Hints
//!
//! 1. Store the free counts in a field such as `free: [i32; 3]` and index it with `car_type - 1`.
//! 2. Check the count before decrementing it.

/// A parking lot with big, medium and small spaces.
pub struct ParkingSystem {}

impl ParkingSystem {
    /// Creates a lot with the given number of free spaces of each kind.
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        todo!("Write your solution here")
    }

    /// Parks a car of type 1 (big), 2 (medium) or 3 (small) if a space is free.
    pub fn add_car(&mut self, car_type: i32) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_big_one_medium() {
        let mut lot = ParkingSystem::new(1, 1, 0);
        assert!(lot.add_car(1));
        assert!(lot.add_car(2));
        assert!(!lot.add_car(3));
        assert!(!lot.add_car(1));
    }

    #[test]
    fn empty_lot_rejects_everyone() {
        let mut lot = ParkingSystem::new(0, 0, 0);
        assert!(!lot.add_car(1));
        assert!(!lot.add_car(2));
        assert!(!lot.add_car(3));
    }

    #[test]
    fn kinds_are_independent() {
        let mut lot = ParkingSystem::new(0, 0, 2);
        assert!(lot.add_car(3));
        assert!(!lot.add_car(2));
        assert!(lot.add_car(3));
        assert!(!lot.add_car(3));
    }

    #[test]
    fn large_lot_fills_exactly() {
        let mut lot = ParkingSystem::new(1000, 1000, 1000);
        let parked = (0..1000).filter(|_| lot.add_car(2)).count();
        assert_eq!(parked, 1000);
        assert!(!lot.add_car(2));
        assert!(lot.add_car(1));
    }
}
