// ==================================================================
// PRACTICE 0738: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0738_capacity_to_ship_packages_within_d_days.md
// Run the tests:          cargo test practice::p0738
// ==================================================================

//! # Problem 0738: Capacity To Ship Packages Within D Days
//!
//! - **Difficulty:** Medium
//! - **Topics:** binary search on the answer, greedy feasibility check, iterator `max` and `sum`
//! - **Source:** [LeetCode 1011. Capacity To Ship Packages Within D Days](https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/)
//!
//! ## Problem
//!
//! Packages wait on a conveyor belt in a fixed order; package `i` weighs `weights[i]`. Every day a
//! ship loads packages **in order** (it cannot skip or reorder them) as long as the total weight
//! loaded that day does not exceed the ship's capacity, then sails.
//!
//! Return the smallest capacity that ships every package within `days` days.
//!
//! Implement `Solution::ship_within_days(weights: Vec<i32>, days: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  weights = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], days = 5
//! Output: 15            (days: [1, 2, 3, 4, 5], [6, 7], [8], [9], [10])
//!
//! Input:  weights = [3, 2, 2, 4, 1, 4], days = 3
//! Output: 6             (days: [3, 2], [2, 4], [1, 4])
//!
//! Input:  weights = [1, 2, 3, 1, 1], days = 4
//! Output: 3             (days: [1], [2], [3], [1, 1])
//! ```
//!
//! ## Constraints
//!
//! - `1 <= days <= weights.len() <= 5 * 10^4`
//! - `1 <= weights[i] <= 500`
//!
//! ## Hints
//!
//! 1. For a fixed capacity, the best plan is greedy: keep loading until the next package does not
//!    fit, then start a new day.
//! 2. A larger capacity never needs more days. What are the smallest and largest capacities worth
//!    trying?

pub struct Solution;

impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_five_days() {
        assert_eq!(Solution::ship_within_days((1..=10).collect(), 5), 15);
    }

    #[test]
    fn example_three_days() {
        assert_eq!(Solution::ship_within_days(vec![3, 2, 2, 4, 1, 4], 3), 6);
    }

    #[test]
    fn example_four_days() {
        assert_eq!(Solution::ship_within_days(vec![1, 2, 3, 1, 1], 4), 3);
    }

    #[test]
    fn one_package_per_day() {
        assert_eq!(Solution::ship_within_days(vec![7, 2, 9, 4], 4), 9);
    }

    #[test]
    fn single_day() {
        assert_eq!(Solution::ship_within_days(vec![7, 2, 9, 4], 1), 22);
    }

    #[test]
    fn long_input() {
        // 50_000 packages of 500 in 10 days: 5_000 packages per day.
        assert_eq!(Solution::ship_within_days(vec![500; 50_000], 10), 2_500_000);
    }
}
