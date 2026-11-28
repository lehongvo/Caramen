//! # Problem 0963: Candy
//!
//! - **Difficulty:** Hard
//! - **Topics:** two-pass greedy (left-to-right, then right-to-left), `windows(2)`-style neighbour comparisons, `max`
//! - **Source:** [LeetCode 135. Candy](https://leetcode.com/problems/candy/)
//!
//! ## Problem
//!
//! Children stand in a line; `ratings[i]` is the rating of child `i`. Hand out candies so that:
//!
//! - every child gets **at least one** candy;
//! - a child with a **strictly higher** rating than an adjacent child gets **more** candies than
//!   that neighbour (equal ratings impose nothing).
//!
//! Return the **minimum** total number of candies. Implement `Solution::candy(ratings: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  ratings = [1, 0, 2]
//! Output: 5          (candies [2, 1, 2])
//!
//! Input:  ratings = [1, 2, 2]
//! Output: 4          (candies [1, 2, 1]; the last child only needs 1)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= ratings.len() <= 2 * 10^4`
//! - `0 <= ratings[i] <= 2 * 10^4`
//!
//! ## Hints
//!
//! 1. Handle the "left neighbour" rule and the "right neighbour" rule separately.
//! 2. Left to right: if `ratings[i] > ratings[i - 1]`, child `i` needs `left[i - 1] + 1`, else `1`.
//! 3. Right to left with the mirrored rule, and each child gets the **maximum** of both requirements.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the left pass computes the minimum each child needs because of its left neighbour, the
//! right pass the minimum because of its right neighbour. Taking the maximum satisfies both rules,
//! and no child can get less, because each value is forced by a strictly increasing run ending at
//! that child.
//!
//! **Complexity:** `O(n)` time, `O(n)` space.
//!
//! **Rust concepts:**
//! - A single `Vec` is updated in both passes: the second pass writes `max(current, needed)`.
//! - Iterating `(0..n - 1).rev()` walks the indices backwards without signed arithmetic.
//! - `iter().sum::<i32>()` adds up the result.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        // @solution-start
        let n = ratings.len();
        let mut candies = vec![1; n];
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                candies[i] = candies[i - 1] + 1;
            }
        }
        for i in (0..n.saturating_sub(1)).rev() {
            if ratings[i] > ratings[i + 1] {
                candies[i] = candies[i].max(candies[i + 1] + 1);
            }
        }
        candies.iter().sum()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    }

    #[test]
    fn small_cases() {
        assert_eq!(Solution::candy(vec![5]), 1);
        assert_eq!(Solution::candy(vec![3, 3, 3, 3]), 4);
        assert_eq!(Solution::candy(vec![1, 3, 4, 5, 2]), 11);
        assert_eq!(Solution::candy(vec![1, 2, 87, 87, 87, 2, 1]), 13);
    }

    #[test]
    fn peaks_and_valleys() {
        // A mountain gets [1, 2, 3, 2, 1]; a valley gets [3, 2, 1, 2, 3].
        assert_eq!(Solution::candy(vec![1, 2, 3, 2, 1]), 9);
        assert_eq!(Solution::candy(vec![3, 2, 1, 2, 3]), 11);
    }

    #[test]
    fn long_monotonic_lines() {
        let up: Vec<i32> = (0..20_000).collect();
        assert_eq!(Solution::candy(up), 20_000 * 20_001 / 2);
        let down: Vec<i32> = (0..20_000).rev().collect();
        assert_eq!(Solution::candy(down), 20_000 * 20_001 / 2);
    }
}
