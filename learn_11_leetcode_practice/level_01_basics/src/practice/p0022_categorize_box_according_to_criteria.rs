// ==================================================================
// PRACTICE 0022: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0022_categorize_box_according_to_criteria.md
// Run the tests:          cargo test practice::p0022
// ==================================================================

//! # Problem 0022: Categorize Box According to Criteria
//!
//! - **Difficulty:** Easy
//! - **Topics:** integer overflow, widening `i32` to `i64`, `bool` variables, `match` on a tuple of `bool`s
//! - **Source:** [LeetCode 2525. Categorize Box According to Criteria](https://leetcode.com/problems/categorize-box-according-to-criteria/)
//!
//! ## Problem
//!
//! A warehouse labels each box from its three dimensions (`length`, `width`, `height`) and its
//! `mass`:
//!
//! - The box is **bulky** if at least one dimension is `>= 10^4`, **or** if its volume
//!   (`length * width * height`) is `>= 10^9`.
//! - The box is **heavy** if its mass is `>= 100`.
//!
//! Return the label as a `String`:
//!
//! - `"Both"` if the box is bulky and heavy;
//! - `"Bulky"` if it is bulky but not heavy;
//! - `"Heavy"` if it is heavy but not bulky;
//! - `"Neither"` otherwise.
//!
//! Implement `Solution::categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String`.
//!
//! ## Examples
//!
//! ```text
//! Input:  length = 1000, width = 35, height = 700, mass = 300
//! Output: "Heavy"
//! Explanation: no side reaches 10^4 and the volume is 24_500_000 < 10^9; the mass is >= 100.
//!
//! Input:  length = 200, width = 50, height = 800, mass = 50
//! Output: "Neither"
//!
//! Input:  length = 2000, width = 1000, height = 1000, mass = 10
//! Output: "Bulky"
//! Explanation: the volume is 2 * 10^9.
//! ```
//!
//! ## Constraints
//!
//! - `1 <= length, width, height <= 10^5`
//! - `1 <= mass <= 10^3`
//!
//! ## Hints
//!
//! 1. The volume can be as large as `10^15`, but an `i32` stops at about `2.1 * 10^9`. Which type is big enough?
//! 2. Compute two `bool` variables, `bulky` and `heavy`, then decide the label from the pair.

pub struct Solution;

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_heavy() {
        assert_eq!(Solution::categorize_box(1000, 35, 700, 300), "Heavy");
    }

    #[test]
    fn example_neither() {
        assert_eq!(Solution::categorize_box(200, 50, 800, 50), "Neither");
    }

    #[test]
    fn example_bulky_by_volume() {
        assert_eq!(Solution::categorize_box(2000, 1000, 1000, 10), "Bulky");
    }

    #[test]
    fn bulky_by_one_dimension() {
        assert_eq!(Solution::categorize_box(10_000, 1, 1, 99), "Bulky");
        assert_eq!(Solution::categorize_box(9_999, 1, 1, 99), "Neither");
    }

    #[test]
    fn both_with_maximum_values_does_not_overflow() {
        assert_eq!(
            Solution::categorize_box(100_000, 100_000, 100_000, 1000),
            "Both"
        );
    }

    #[test]
    fn volume_exactly_at_the_limit() {
        assert_eq!(Solution::categorize_box(1000, 1000, 1000, 100), "Both");
        assert_eq!(Solution::categorize_box(1000, 1000, 999, 100), "Heavy");
    }
}
