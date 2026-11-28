//! # Problem 0962: First Missing Positive
//!
//! - **Difficulty:** Hard
//! - **Topics:** in-place cyclic placement with `swap`, using the array as its own hash table, `O(1)` extra space
//! - **Source:** [LeetCode 41. First Missing Positive](https://leetcode.com/problems/first-missing-positive/)
//!
//! ## Problem
//!
//! Given an unsorted integer array `nums`, return the **smallest positive integer** that does not
//! appear in it. The algorithm must run in `O(n)` time and use only `O(1)` extra memory (the input
//! vector itself may be modified).
//!
//! Implement `Solution::first_missing_positive(nums: Vec<i32>) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 0]
//! Output: 3
//!
//! Input:  nums = [3, 4, -1, 1]
//! Output: 2
//!
//! Input:  nums = [7, 8, 9, 11, 12]
//! Output: 1
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^5`
//! - `-2^31 <= nums[i] <= 2^31 - 1`
//!
//! ## Hints
//!
//! 1. The answer is in `1..=n + 1` for an array of length `n`: only values in `1..=n` matter.
//! 2. Try to put every value `v` in `1..=n` at index `v - 1` by swapping.
//! 3. Afterwards, the first index `i` with `nums[i] != i + 1` gives the answer `i + 1`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** cyclic placement. For each index, keep swapping its value into its "home" slot
//! `v - 1` while the value is in range and the home slot does not already hold `v` (that check also
//! stops infinite loops on duplicates). Each swap puts one value home for good, so there are at most
//! `n` swaps overall. Then scan for the first slot that is not home.
//!
//! **Complexity:** `O(n)` time, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - `nums.swap(i, j)` swaps two elements of a `Vec` in place.
//! - The loop condition checks `1 <= v <= n` **before** casting with `as usize`, so the cast can
//!   never wrap a negative value into a huge index.
//! - Taking `nums` by value (LeetCode's signature) lets us `let mut nums = nums;` and reuse its memory.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        // @solution-start
        let mut nums = nums;
        let n = nums.len();
        for i in 0..n {
            while nums[i] >= 1 && (nums[i] as usize) <= n {
                let home = nums[i] as usize - 1;
                if nums[home] == nums[i] {
                    break;
                }
                nums.swap(i, home);
            }
        }
        nums.iter()
            .enumerate()
            .find(|&(i, &v)| v != i as i32 + 1)
            .map_or(n as i32 + 1, |(i, _)| i as i32 + 1)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }

    #[test]
    fn duplicates_and_extremes() {
        assert_eq!(Solution::first_missing_positive(vec![1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![2, 2, 2]), 1);
        assert_eq!(
            Solution::first_missing_positive(vec![i32::MAX, i32::MIN, 1]),
            2
        );
        assert_eq!(Solution::first_missing_positive(vec![1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![0]), 1);
    }

    #[test]
    fn permutation_gives_n_plus_one() {
        let nums: Vec<i32> = (1..=100_000).rev().collect();
        assert_eq!(Solution::first_missing_positive(nums), 100_001);
    }

    #[test]
    fn one_hole_in_a_large_shuffle() {
        let n = 100_000;
        let mut nums: Vec<i32> = (0..n).map(|i| (i * 7_919) % n + 1).collect();
        // 7919 is coprime with 100_000, so this is a permutation of 1..=n; punch a hole at 54_321.
        let hole = nums.iter().position(|&v| v == 54_321).unwrap();
        nums[hole] = -5;
        assert_eq!(Solution::first_missing_positive(nums), 54_321);
    }
}
