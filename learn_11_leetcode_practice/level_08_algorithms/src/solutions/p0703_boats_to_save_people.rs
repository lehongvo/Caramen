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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** sorting + two pointers, greedy: sort the weights. The heaviest remaining person needs
//! a boat anyway. If even the lightest remaining person cannot join them, nobody can, so they go
//! alone. If the lightest can join, pairing them is never worse than any other choice: any partner
//! that fits with a lighter person also fits with someone heavier. So each step sends the person
//! at `hi`, plus the person at `lo` when `people[lo] + people[hi] <= limit`.
//!
//! **Complexity:** `O(n log n)` time for the sort, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - `let mut people = people;` re-binds the owned vector as mutable so it can be sorted in place.
//! - `sort_unstable` is faster than `sort` and fine for plain integers, where stability is
//!   meaningless.
//! - A half-open window `lo..hi` (with `hi -= 1` before use) avoids `usize` underflow.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        // @solution-start
        let mut people = people;
        people.sort_unstable();
        let (mut lo, mut hi) = (0, people.len());
        let mut boats = 0;
        while lo < hi {
            hi -= 1; // the heaviest remaining person always boards
            if lo < hi && people[lo] + people[hi] <= limit {
                lo += 1; // the lightest remaining person joins them
            }
            boats += 1;
        }
        boats
        // @solution-end
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
