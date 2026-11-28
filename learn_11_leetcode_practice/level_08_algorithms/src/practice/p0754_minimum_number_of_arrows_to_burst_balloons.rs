// ==================================================================
// PRACTICE 0754: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0754_minimum_number_of_arrows_to_burst_balloons.md
// Run the tests:          cargo test practice::p0754
// ==================================================================

//! # Problem 0754: Minimum Number of Arrows to Burst Balloons
//!
//! - **Difficulty:** Medium
//! - **Topics:** greedy by earliest end, extreme `i32` values without overflow, `Option` state
//! - **Source:** [LeetCode 452. Minimum Number of Arrows to Burst Balloons](https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/)
//!
//! ## Problem
//!
//! Balloons are taped to a wall; balloon `i` covers the horizontal range
//! `points[i] = [x_start, x_end]`. An arrow shot straight up from position `x` bursts every
//! balloon with `x_start <= x <= x_end` (touching the edge is enough) and keeps flying.
//!
//! Return the minimum number of arrows needed to burst every balloon.
//!
//! Implement `Solution::find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  points = [[10, 16], [2, 8], [1, 6], [7, 12]]
//! Output: 2             (an arrow at x = 6 bursts [2, 8] and [1, 6]; one at x = 11 bursts the others)
//!
//! Input:  points = [[1, 2], [3, 4], [5, 6], [7, 8]]
//! Output: 4
//!
//! Input:  points = [[1, 2], [2, 3], [3, 4], [4, 5]]
//! Output: 2             (arrows at x = 2 and x = 4)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= points.len() <= 10^5`
//! - `points[i].len() == 2` and `-2^31 <= x_start < x_end <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. Look at the balloon that **ends** first. Some arrow must hit it; where is the best place to
//!    shoot that arrow?
//! 2. The coordinates use the whole `i32` range: avoid computing differences such as
//!    `a[1] - b[1]` in a comparator, which can overflow.

pub struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(points: &[[i32; 2]]) -> i32 {
        Solution::find_min_arrow_shots(points.iter().map(|p| p.to_vec()).collect())
    }

    #[test]
    fn example_two_arrows() {
        assert_eq!(run(&[[10, 16], [2, 8], [1, 6], [7, 12]]), 2);
    }

    #[test]
    fn example_all_separate() {
        assert_eq!(run(&[[1, 2], [3, 4], [5, 6], [7, 8]]), 4);
    }

    #[test]
    fn example_touching_edges() {
        assert_eq!(run(&[[1, 2], [2, 3], [3, 4], [4, 5]]), 2);
    }

    #[test]
    fn single_balloon() {
        assert_eq!(run(&[[1, 2]]), 1);
    }

    #[test]
    fn extreme_coordinates() {
        assert_eq!(run(&[[i32::MIN, i32::MAX]]), 1);
        assert_eq!(run(&[[i32::MIN, 0], [0, i32::MAX]]), 1);
        assert_eq!(
            run(&[[i32::MIN, i32::MIN + 1], [i32::MAX - 1, i32::MAX]]),
            2
        );
    }

    #[test]
    fn many_disjoint_balloons() {
        let points: Vec<Vec<i32>> = (0..100_000).rev().map(|i| vec![i * 3, i * 3 + 1]).collect();
        assert_eq!(Solution::find_min_arrow_shots(points), 100_000);
    }
}
