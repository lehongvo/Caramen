// ==================================================================
// PRACTICE 0223: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0223_furthest_point_from_origin.md
// Run the tests:          cargo test practice::p0223
// ==================================================================

//! # Problem 0223: Furthest Point From Origin
//!
//! - **Difficulty:** Easy
//! - **Topics:** enum with a "wildcard" variant, counting with a struct, `match` on `char`
//! - **Source:** [LeetCode 2833. Furthest Point From Origin](https://leetcode.com/problems/furthest-point-from-origin/)
//!
//! ## Problem
//!
//! You stand at position `0` on a number line and receive a string `moves` of `'L'`, `'R'` and `'_'`
//! characters. Process them in order; each one is a single step:
//!
//! - `'L'` means you **must** step left (position `- 1`);
//! - `'R'` means you **must** step right (position `+ 1`);
//! - `'_'` means you may **choose** to step either left or right.
//!
//! Choosing the free steps as cleverly as possible, return the largest distance from `0` (the
//! absolute value of the final position) you can end at.
//!
//! Implement `Solution::furthest_distance_from_origin(moves: String) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  moves = "L_RL__R"
//! Output: 3      (2 forced L, 2 forced R, 3 free steps all to the left: final position -3)
//!
//! Input:  moves = "_R__LL_"
//! Output: 5      (forced: 1 R, 2 L; the 4 free steps go left: -1 - 4 = -5)
//!
//! Input:  moves = "_______"
//! Output: 7
//! ```
//!
//! ## Constraints
//!
//! - `1 <= moves.len() <= 50`
//! - `moves` contains only `'L'`, `'R'` and `'_'`.
//!
//! ## Hints
//!
//! 1. The order of the steps does not matter, only how many of each kind there are.
//! 2. All free steps should go the same way: towards the side that is already winning.

pub struct Solution;

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::furthest_distance_from_origin("L_RL__R".to_string()),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::furthest_distance_from_origin("_R__LL_".to_string()),
            5
        );
    }

    #[test]
    fn example_all_free() {
        assert_eq!(
            Solution::furthest_distance_from_origin("_______".to_string()),
            7
        );
    }

    #[test]
    fn forced_steps_only() {
        assert_eq!(
            Solution::furthest_distance_from_origin("LRLR".to_string()),
            0
        );
        assert_eq!(
            Solution::furthest_distance_from_origin("RRR".to_string()),
            3
        );
    }

    #[test]
    fn free_steps_follow_the_majority() {
        // Forced: 3 R, 1 L => +2; the 2 free steps go right too.
        assert_eq!(
            Solution::furthest_distance_from_origin("RRLR__".to_string()),
            4
        );
    }

    #[test]
    fn longest_input() {
        let moves = "L".repeat(20) + &"R".repeat(25) + &"_".repeat(5);
        assert_eq!(Solution::furthest_distance_from_origin(moves), 10);
    }
}
