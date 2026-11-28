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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** binary search on the answer: for a day `d`, scan the row counting consecutive
//! bloomed flowers (`bloom_day[i] <= d`); every time the run reaches `k`, one bouquet is made and
//! the run restarts. More days never mean fewer bouquets, so search `min..=max` of `bloom_day` for
//! the first day with at least `m` bouquets. When `m * k > n` no day works, so return `-1` first
//! (otherwise the search would return `max` without checking it).
//!
//! **Complexity:** `O(n log D)` time where `D = max(bloom_day)`, `O(1)` space.
//!
//! **Rust concepts:**
//! - `m as u64 * k as u64` widens **before** multiplying: the product can reach
//!   `10^6 * 10^5 = 10^11`, which fits in `u64` but would overflow `i32`.
//! - The feasibility test is a closure returning `bool`, so the search loop reads like the idea.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        // @solution-start
        if m as u64 * k as u64 > bloom_day.len() as u64 {
            return -1;
        }
        let (m, k) = (m as usize, k as usize);
        let enough_bouquets = |day: i32| {
            let (mut bouquets, mut run) = (0, 0);
            for &bloom in &bloom_day {
                if bloom <= day {
                    run += 1;
                    if run == k {
                        bouquets += 1;
                        run = 0;
                    }
                } else {
                    run = 0;
                }
            }
            bouquets >= m
        };
        let mut lo = bloom_day.iter().copied().min().unwrap_or(0);
        let mut hi = bloom_day.iter().copied().max().unwrap_or(0); // works when m * k <= n
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if enough_bouquets(mid) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
        // @solution-end
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
