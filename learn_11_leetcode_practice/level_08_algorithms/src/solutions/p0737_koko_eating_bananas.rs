//! # Problem 0737: Koko Eating Bananas
//!
//! - **Difficulty:** Medium
//! - **Topics:** binary search on the answer, feasibility check, `u64::div_ceil`, `u64` sums
//! - **Source:** [LeetCode 875. Koko Eating Bananas](https://leetcode.com/problems/koko-eating-bananas/)
//!
//! ## Problem
//!
//! There are `n` piles of bananas; pile `i` holds `piles[i]` bananas. Koko picks an integer
//! eating speed `k` (bananas per hour). Every hour she chooses one pile and eats `k` bananas from
//! it; if the pile has fewer than `k` bananas left, she eats all of them and does nothing else for
//! the rest of that hour. The guards come back in `h` hours.
//!
//! Return the **minimum** speed `k` that lets her finish every pile within `h` hours.
//!
//! Implement `Solution::min_eating_speed(piles: Vec<i32>, h: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  piles = [3, 6, 7, 11], h = 8
//! Output: 4             (hours at k = 4: 1 + 2 + 2 + 3 = 8; at k = 3: 1 + 2 + 3 + 4 = 10)
//!
//! Input:  piles = [30, 11, 23, 4, 20], h = 5
//! Output: 30            (one pile per hour, so k must cover the largest pile)
//!
//! Input:  piles = [30, 11, 23, 4, 20], h = 6
//! Output: 23
//! ```
//!
//! ## Constraints
//!
//! - `1 <= piles.len() <= 10^4`
//! - `piles.len() <= h <= 10^9`
//! - `1 <= piles[i] <= 10^9`
//!
//! ## Hints
//!
//! 1. At speed `k`, pile `p` takes `ceil(p / k)` hours. Faster speeds never take longer.
//! 2. The answer is between `1` and `max(piles)`. Binary search for the smallest speed that is
//!    fast enough.
//! 3. At speed `1`, the total number of hours can reach `10^4 * 10^9`, far beyond `i32`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** binary search on the answer: the total time `hours(k) = sum(ceil(p / k))` never
//! increases when `k` grows, so "`hours(k) <= h`" is `false` for small speeds and `true` from
//! some speed on. Search the range `1..=max(piles)` (at speed `max(piles)` every pile takes one
//! hour, and `h >= n`) for the first speed where it is `true`. Each test costs `O(n)`.
//!
//! **Complexity:** `O(n log M)` time where `M = max(piles)`, `O(1)` space.
//!
//! **Rust concepts:**
//! - [`u64::div_ceil`](https://doc.rust-lang.org/std/primitive.u64.html#method.div_ceil) (stable
//!   since 1.73) rounds the division up; for signed integers it is still unstable, hence the
//!   conversion to `u64`.
//! - The `hours` closure borrows `piles` and is called once per binary-search step.
//! - `iter().max()` returns an `Option`; the constraints guarantee a non-empty vector.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        // @solution-start
        let hours = |speed: u64| -> u64 { piles.iter().map(|&p| (p as u64).div_ceil(speed)).sum() };
        let max_pile = piles.iter().copied().max().unwrap_or(1); // piles is never empty
        let (mut lo, mut hi) = (1u64, max_pile as u64); // hours(max_pile) <= h always holds
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if hours(mid) <= h as u64 {
                hi = mid; // fast enough: maybe a slower speed works too
            } else {
                lo = mid + 1;
            }
        }
        lo as i32
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_speed_four() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    }

    #[test]
    fn example_one_pile_per_hour() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    }

    #[test]
    fn example_one_spare_hour() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }

    #[test]
    fn plenty_of_time() {
        assert_eq!(Solution::min_eating_speed(vec![5, 5, 5], 1_000_000_000), 1);
    }

    #[test]
    fn huge_single_pile() {
        assert_eq!(
            Solution::min_eating_speed(vec![1_000_000_000], 2),
            500_000_000
        );
        assert_eq!(
            Solution::min_eating_speed(vec![312_884_470], 312_884_469),
            2
        );
    }

    #[test]
    fn total_hours_exceed_i32() {
        // 10^4 piles of 10^9 bananas in 10^9 hours: each pile may take 10^5 hours.
        assert_eq!(
            Solution::min_eating_speed(vec![1_000_000_000; 10_000], 1_000_000_000),
            10_000
        );
    }
}
