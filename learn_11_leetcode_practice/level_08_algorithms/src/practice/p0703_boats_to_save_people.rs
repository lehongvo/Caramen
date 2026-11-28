// ==================================================================
// PRACTICE 0703: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0703_boats_to_save_people.md
// Run the tests:          cargo test practice::p0703
// ==================================================================

//! # Problem 0703: Boats to Save People
//!
//! - **Difficulty:** Medium
//! - **Topics:** sorting + two pointers, greedy pairing, `sort_unstable`
//! - **Source:** [LeetCode 881. Boats to Save People](https://leetcode.com/problems/boats-to-save-people/)
//!
//! ## Problem
//!
//! `people[i]` is the weight of the `i`-th person. Every boat can carry **at most two** people at
//! the same time, and the total weight in a boat must not exceed `limit`. Nobody is heavier than
//! `limit`, so everyone fits in a boat alone.
//!
//! Return the minimum number of boats needed to carry everybody.
//!
//! Implement `Solution::num_rescue_boats(people: Vec<i32>, limit: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  people = [1, 2], limit = 3
//! Output: 1             (both fit together)
//!
//! Input:  people = [3, 2, 2, 1], limit = 3
//! Output: 3             (boats: (1, 2), (2), (3))
//!
//! Input:  people = [3, 5, 3, 4], limit = 5
//! Output: 4             (no two people fit together)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= people.len() <= 5 * 10^4`
//! - `1 <= people[i] <= limit <= 3 * 10^4`
//!
//! ## Hints
//!
//! 1. Think about the heaviest person. Who is the best partner for them?
//! 2. After sorting, the lightest and the heaviest remaining people are at the two ends.

pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one_boat() {
        assert_eq!(Solution::num_rescue_boats(vec![1, 2], 3), 1);
    }

    #[test]
    fn example_three_boats() {
        assert_eq!(Solution::num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
    }

    #[test]
    fn example_nobody_shares() {
        assert_eq!(Solution::num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
    }

    #[test]
    fn single_person() {
        assert_eq!(Solution::num_rescue_boats(vec![5], 5), 1);
    }

    #[test]
    fn heaviest_pairs_with_lightest() {
        // Sorted: 1 2 4 5, limit 6: (1, 5) and (2, 4).
        assert_eq!(Solution::num_rescue_boats(vec![5, 1, 4, 2], 6), 2);
    }

    #[test]
    fn many_light_people() {
        assert_eq!(Solution::num_rescue_boats(vec![1; 50_000], 2), 25_000);
    }
}
