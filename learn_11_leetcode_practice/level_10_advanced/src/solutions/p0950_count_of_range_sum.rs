//! # Problem 0950: Count of Range Sum
//!
//! - **Difficulty:** Hard
//! - **Topics:** prefix sums in `i64`, merge sort with a sliding window over the right half, `slice::sort` of merged halves
//! - **Source:** [LeetCode 327. Count of Range Sum](https://leetcode.com/problems/count-of-range-sum/)
//!
//! ## Problem
//!
//! Given an integer array `nums` and two integers `lower <= upper`, count the **range sums** that lie
//! in `[lower, upper]` (inclusive). The range sum `S(i, j)` is `nums[i] + nums[i + 1] + ... + nums[j]`
//! for `0 <= i <= j < nums.len()`.
//!
//! Implement `Solution::count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [-2, 5, -1], lower = -2, upper = 2
//! Output: 3            (S(0,0) = -2, S(2,2) = -1, S(0,2) = 2)
//!
//! Input:  nums = [0], lower = 0, upper = 0
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `-2^31 <= nums[i] <= 2^31 - 1`
//! - `-10^5 <= lower <= upper <= 10^5`
//! - The answer fits in an `i32`.
//!
//! ## Hints
//!
//! 1. With prefix sums `P[0] = 0`, `P[k] = nums[0] + ... + nums[k-1]`, the range sum `S(i, j)` is
//!    `P[j + 1] - P[i]`. Count pairs `a < b` with `lower <= P[b] - P[a] <= upper`.
//! 2. Merge sort the prefix sums. Before merging two sorted halves, for every `P[a]` in the left half
//!    the valid `P[b]` in the right half form a contiguous window `[P[a] + lower, P[a] + upper]`.
//! 3. As `P[a]` increases, both window ends only move right: two pointers make the count linear.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** count index pairs `a < b` of prefix sums by divide and conquer. Pairs inside a half are
//! counted recursively; for crossing pairs both halves are sorted, so for each left value the
//! matching right values are found by two monotone pointers `lo` (first `>= left + lower`) and `hi`
//! (first `> left + upper`), contributing `hi - lo`. Then the halves are merged.
//!
//! **Complexity:** `O(n log n)` time, `O(n)` space.
//!
//! **Rust concepts:**
//! - Prefix sums of up to `10^5` values of magnitude `2^31` need `i64`.
//! - `scan` builds the prefix-sum vector in one expression.
//! - Merging by `sort_unstable` on the whole range would be `O(n log^2 n)`; an explicit merge keeps
//!   it `O(n log n)`.
// @solution-end

pub struct Solution;

// @solution-start remove
fn count(prefix: &mut [i64], lower: i64, upper: i64, scratch: &mut Vec<i64>) -> usize {
    if prefix.len() < 2 {
        return 0;
    }
    let mid = prefix.len() / 2;
    let (left, right) = prefix.split_at_mut(mid);
    let mut total = count(left, lower, upper, scratch) + count(right, lower, upper, scratch);
    let (mut lo, mut hi) = (0, 0);
    for &a in left.iter() {
        while lo < right.len() && right[lo] < a + lower {
            lo += 1;
        }
        while hi < right.len() && right[hi] <= a + upper {
            hi += 1;
        }
        total += hi - lo;
    }
    scratch.clear();
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            scratch.push(left[i]);
            i += 1;
        } else {
            scratch.push(right[j]);
            j += 1;
        }
    }
    scratch.extend_from_slice(&left[i..]);
    scratch.extend_from_slice(&right[j..]);
    prefix.copy_from_slice(scratch);
    total
}
// @solution-end

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        // @solution-start
        let mut prefix: Vec<i64> = std::iter::once(0)
            .chain(nums.iter().scan(0_i64, |sum, &x| {
                *sum += i64::from(x);
                Some(*sum)
            }))
            .collect();
        let mut scratch = Vec::with_capacity(prefix.len());
        count(
            &mut prefix,
            i64::from(lower),
            i64::from(upper),
            &mut scratch,
        ) as i32
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(nums: &[i32], lower: i32, upper: i32) -> i32 {
        let mut total = 0;
        for i in 0..nums.len() {
            let mut sum = 0_i64;
            for &x in &nums[i..] {
                sum += i64::from(x);
                if (i64::from(lower)..=i64::from(upper)).contains(&sum) {
                    total += 1;
                }
            }
        }
        total
    }

    #[test]
    fn examples() {
        assert_eq!(Solution::count_range_sum(vec![-2, 5, -1], -2, 2), 3);
        assert_eq!(Solution::count_range_sum(vec![0], 0, 0), 1);
    }

    #[test]
    fn extreme_values() {
        assert_eq!(
            Solution::count_range_sum(vec![i32::MIN, i32::MAX, 1], -1, 1),
            3
        );
        assert_eq!(
            Solution::count_range_sum(vec![i32::MAX, i32::MAX, i32::MAX], 0, 100_000),
            0
        );
    }

    #[test]
    fn all_zeros_count_every_range() {
        let n = 300;
        assert_eq!(
            Solution::count_range_sum(vec![0; n], 0, 0),
            (n * (n + 1) / 2) as i32
        );
    }

    #[test]
    fn matches_naive() {
        let nums: Vec<i32> = (0..400).map(|i: i32| (i * 7_919) % 41 - 20).collect();
        for (lower, upper) in [(-5, 5), (0, 0), (-100, -50), (10, 100)] {
            assert_eq!(
                Solution::count_range_sum(nums.clone(), lower, upper),
                naive(&nums, lower, upper),
                "bounds {lower}..={upper}"
            );
        }
    }

    #[test]
    fn large_input_runs_fast() {
        let nums: Vec<i32> = (0..50_000)
            .map(|i| if i % 2 == 0 { 1 } else { -1 })
            .collect();
        // Prefix sums alternate 0, 1, 0, 1, ...; a range sum is 0 exactly when it has even length.
        let n: i64 = 50_000;
        let even_length_ranges: i64 = (1..=n / 2).map(|half| n - 2 * half + 1).sum();
        assert_eq!(
            i64::from(Solution::count_range_sum(nums, 0, 0)),
            even_length_ranges
        );
    }
}
