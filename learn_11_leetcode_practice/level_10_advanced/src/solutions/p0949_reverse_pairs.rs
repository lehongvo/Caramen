//! # Problem 0949: Reverse Pairs
//!
//! - **Difficulty:** Hard
//! - **Topics:** divide and conquer, merge sort that counts, `i64` to avoid overflow, `split_at_mut`, two pointers on sorted halves
//! - **Source:** [LeetCode 493. Reverse Pairs](https://leetcode.com/problems/reverse-pairs/)
//!
//! ## Problem
//!
//! Given an integer array `nums`, count the **reverse pairs**: pairs of indices `(i, j)` with
//! `i < j` and `nums[i] > 2 * nums[j]`.
//!
//! Implement `Solution::reverse_pairs(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 3, 2, 3, 1]
//! Output: 2          ((1, 4): 3 > 2 * 1 and (3, 4): 3 > 2 * 1)
//!
//! Input:  nums = [2, 4, 3, 5, 1]
//! Output: 3          (4 > 2, 3 > 2, 5 > 2, all paired with the final 1)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 5 * 10^4`
//! - `-2^31 <= nums[i] <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. Sort both halves recursively; a reverse pair is then either inside a half or crosses them.
//! 2. With both halves sorted, count crossing pairs with two pointers: for each `x` in the left half,
//!    advance a pointer in the right half while `x > 2 * right[k]`.
//! 3. `2 * nums[j]` overflows `i32` for large values: compare in `i64`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** a merge sort that returns the number of reverse pairs in its range. After sorting the two
//! halves, the crossing pairs are counted in one linear pass: as `x` grows through the left half, the
//! number of right elements `y` with `x > 2y` can only grow, so the right pointer never moves back.
//! Then the halves are merged as usual.
//!
//! **Complexity:** `O(n log n)` time, `O(n)` extra space for merging.
//!
//! **Rust concepts:**
//! - `split_at_mut` gives two independent `&mut` halves for the recursive calls.
//! - Working on `i64` copies of the values removes every overflow concern.
//! - A reusable scratch buffer avoids allocating at each level of the recursion.
// @solution-end

pub struct Solution;

// @solution-start remove
fn sort_count(values: &mut [i64], scratch: &mut Vec<i64>) -> usize {
    if values.len() < 2 {
        return 0;
    }
    let mid = values.len() / 2;
    let (left, right) = values.split_at_mut(mid);
    let mut count = sort_count(left, scratch) + sort_count(right, scratch);
    let mut k = 0;
    for &x in left.iter() {
        while k < right.len() && x > 2 * right[k] {
            k += 1;
        }
        count += k;
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
    values.copy_from_slice(scratch);
    count
}
// @solution-end

impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        // @solution-start
        let mut values: Vec<i64> = nums.into_iter().map(i64::from).collect();
        let mut scratch = Vec::with_capacity(values.len());
        sort_count(&mut values, &mut scratch) as i32
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(nums: &[i32]) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if i64::from(nums[i]) > 2 * i64::from(nums[j]) {
                    count += 1;
                }
            }
        }
        count
    }

    #[test]
    fn examples() {
        assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
        assert_eq!(Solution::reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
        assert_eq!(Solution::reverse_pairs(vec![7]), 0);
    }

    #[test]
    fn extreme_values_do_not_overflow() {
        assert_eq!(Solution::reverse_pairs(vec![i32::MAX, i32::MAX, -1]), 2);
        // -2^31 > 2 * -2^31 = -2^32, which only holds when computed in i64.
        assert_eq!(Solution::reverse_pairs(vec![i32::MIN, i32::MIN]), 1);
        assert_eq!(Solution::reverse_pairs(vec![-5, -5]), 1);
        assert_eq!(Solution::reverse_pairs(vec![i32::MAX, 1_073_741_824]), 0);
        assert_eq!(Solution::reverse_pairs(vec![i32::MAX, 1_073_741_823]), 1);
    }

    #[test]
    fn matches_naive() {
        let nums: Vec<i32> = (0..400).map(|i: i32| (i * 7_919) % 1_000 - 500).collect();
        assert_eq!(Solution::reverse_pairs(nums.clone()), naive(&nums));
    }

    #[test]
    fn large_input() {
        let nums: Vec<i32> = (0..50_000).rev().collect();
        // Pairs (i, j) with value_i > 2 * value_j over a strictly decreasing sequence 49_999..=0.
        let expected: i64 = (0..50_000_i64).map(|v| (v + 1) / 2).sum();
        assert_eq!(i64::from(Solution::reverse_pairs(nums)), expected);
    }
}
