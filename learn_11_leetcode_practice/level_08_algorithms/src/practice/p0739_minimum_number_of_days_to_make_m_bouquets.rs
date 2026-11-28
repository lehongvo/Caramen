// ==================================================================
// PRACTICE 0739: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0739_minimum_number_of_days_to_make_m_bouquets.md
// Run the tests:          cargo test practice::p0739
// ==================================================================

//! # Problem 0739: Minimum Number of Days to Make m Bouquets
//!
//! - **Difficulty:** Medium
//! - **Topics:** binary search on the answer, counting runs, `u64` overflow guard
//! - **Source:** [LeetCode 1482. Minimum Number of Days to Make m Bouquets](https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/)
//!
//! ## Problem
//!
//! A garden has `n` flowers in a row; flower `i` blooms on day `bloom_day[i]` and stays bloomed
//! afterwards. You want `m` bouquets, and every bouquet must use `k` **adjacent** bloomed flowers.
//! Each flower can be used in at most one bouquet.
//!
//! Return the minimum number of days to wait until you can make `m` bouquets, or `-1` if that is
//! impossible.
//!
//! Implement `Solution::min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  bloom_day = [1, 10, 3, 10, 2], m = 3, k = 1
//! Output: 3             (on day 3, flowers 0, 2 and 4 are bloomed: three 1-flower bouquets)
//!
//! Input:  bloom_day = [1, 10, 3, 10, 2], m = 3, k = 2
//! Output: -1            (6 flowers are needed but there are only 5)
//!
//! Input:  bloom_day = [7, 7, 7, 7, 12, 7, 7], m = 2, k = 3
//! Output: 12            (on day 7 only the first run of 4 gives a bouquet; on day 12 all bloom)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= bloom_day.len() <= 10^5`
//! - `1 <= bloom_day[i] <= 10^9`
//! - `1 <= m <= 10^6`
//! - `1 <= k <= bloom_day.len()`
//!
//! ## Hints
//!
//! 1. If `m * k > n`, there are not enough flowers. Careful: `m * k` can overflow `i32`.
//! 2. Waiting longer never reduces the number of bouquets. Binary search over the days.
//! 3. For a given day, scan the row and count runs of consecutive bloomed flowers.

pub struct Solution;

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_single_flower_bouquets() {
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 1), 3);
    }

    #[test]
    fn example_not_enough_flowers() {
        assert_eq!(Solution::min_days(vec![1, 10, 3, 10, 2], 3, 2), -1);
    }

    #[test]
    fn example_wait_for_the_last_flower() {
        assert_eq!(Solution::min_days(vec![7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
    }

    #[test]
    fn adjacency_matters() {
        // Day 8 gives runs of 1 and 6 (3 bouquets); day 9 joins them into a run of 8 (4 bouquets).
        assert_eq!(
            Solution::min_days(vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6], 4, 2),
            9
        );
    }

    #[test]
    fn huge_days() {
        assert_eq!(
            Solution::min_days(vec![1_000_000_000, 1_000_000_000], 1, 1),
            1_000_000_000
        );
    }

    #[test]
    fn product_m_times_k_overflows_i32() {
        // 10^6 * 10^5 = 10^11 flowers needed: impossible, and the check must not overflow.
        assert_eq!(Solution::min_days(vec![1; 100_000], 1_000_000, 100_000), -1);
        assert_eq!(Solution::min_days(vec![1; 100_000], 1, 100_000), 1);
    }
}
