// ==================================================================
// PRACTICE 0980: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0980_the_skyline_problem.md
// Run the tests:          cargo test practice::p0980
// ==================================================================

//! # Problem 0980: The Skyline Problem
//!
//! - **Difficulty:** Hard
//! - **Topics:** sweep line over sorted events, a multiset as `BTreeMap<height, count>`, grouping events by x, `chunk_by`
//! - **Source:** [LeetCode 218. The Skyline Problem](https://leetcode.com/problems/the-skyline-problem/)
//!
//! ## Problem
//!
//! Each building is a rectangle on flat ground: `buildings[i] = [left, right, height]` covers
//! `left <= x < right`. The **skyline** is the outer contour of all buildings seen from far away,
//! described by its **key points** `[x, y]`: each one is where the contour's height changes to `y`
//! at position `x`, sorted by `x`. The last key point always has `y = 0` (where the last building
//! ends). Consecutive key points never have the same height (merge them).
//!
//! Implement `Solution::get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  buildings = [[2, 9, 10], [3, 7, 15], [5, 12, 12], [15, 20, 10], [19, 24, 8]]
//! Output: [[2, 10], [3, 15], [7, 12], [12, 0], [15, 10], [20, 8], [24, 0]]
//!
//! Input:  buildings = [[0, 2, 3], [2, 5, 3]]
//! Output: [[0, 3], [5, 0]]       (same height: one flat segment)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= buildings.len() <= 10^4`
//! - `0 <= left < right <= 2^31 - 1`, `1 <= height <= 2^31 - 1`
//! - `buildings` is sorted by `left`.
//!
//! ## Hints
//!
//! 1. Turn each building into two events: its height starts at `left` and stops at `right`.
//! 2. Sweep the events in order of `x`, keeping the multiset of heights currently "on" (plus a ground
//!    height of 0). The contour height is the multiset's maximum.
//! 3. Apply **all** events with the same `x` before comparing the maximum with the previous height;
//!    emit `[x, max]` only when it changed.

use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(buildings: &[[i32; 3]]) -> Vec<[i32; 2]> {
        Solution::get_skyline(buildings.iter().map(|b| b.to_vec()).collect())
            .into_iter()
            .map(|p| [p[0], p[1]])
            .collect()
    }

    #[test]
    fn examples() {
        assert_eq!(
            run(&[
                [2, 9, 10],
                [3, 7, 15],
                [5, 12, 12],
                [15, 20, 10],
                [19, 24, 8]
            ]),
            vec![
                [2, 10],
                [3, 15],
                [7, 12],
                [12, 0],
                [15, 10],
                [20, 8],
                [24, 0]
            ]
        );
        assert_eq!(run(&[[0, 2, 3], [2, 5, 3]]), vec![[0, 3], [5, 0]]);
    }

    #[test]
    fn shared_edges() {
        assert_eq!(
            run(&[[1, 2, 1], [1, 2, 2], [1, 2, 3]]),
            vec![[1, 3], [2, 0]]
        );
        assert_eq!(
            run(&[[0, 5, 7], [5, 10, 7], [5, 10, 12], [10, 15, 7]]),
            vec![[0, 7], [5, 12], [10, 7], [15, 0]]
        );
        assert_eq!(run(&[[0, 3, 5], [3, 6, 2]]), vec![[0, 5], [3, 2], [6, 0]]);
    }

    #[test]
    fn nested_buildings() {
        assert_eq!(
            run(&[[0, 10, 3], [2, 4, 5], [6, 8, 1]]),
            vec![[0, 3], [2, 5], [4, 3], [10, 0]]
        );
        assert_eq!(
            run(&[[0, i32::MAX, i32::MAX]]),
            vec![[0, i32::MAX], [i32::MAX, 0]]
        );
    }

    #[test]
    fn matches_a_height_scan() {
        let buildings: Vec<[i32; 3]> = (0..200)
            .map(|i: i32| {
                let left = (i * 37) % 300;
                [left, left + (i * 13) % 40 + 1, (i * 7) % 11 + 1]
            })
            .collect();
        let mut sorted = buildings.clone();
        sorted.sort_unstable();
        let mut expected = Vec::new();
        let mut previous = 0;
        for x in 0..400 {
            let h = sorted
                .iter()
                .filter(|b| b[0] <= x && x < b[1])
                .map(|b| b[2])
                .max()
                .unwrap_or(0);
            if h != previous {
                expected.push([x, h]);
                previous = h;
            }
        }
        assert_eq!(run(&sorted), expected);
    }
}
