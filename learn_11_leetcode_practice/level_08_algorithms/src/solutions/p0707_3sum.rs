//! # Problem 0707: 3Sum
//!
//! - **Difficulty:** Medium
//! - **Topics:** sorting + two pointers, skipping duplicates, `enumerate`, `match` on `Ordering`
//! - **Source:** [LeetCode 15. 3Sum](https://leetcode.com/problems/3sum/)
//!
//! ## Problem
//!
//! Given a vector `nums`, return every **distinct** triplet `[a, b, c]` of values taken from three
//! different positions such that `a + b + c == 0`.
//!
//! Two triplets are the same if they contain the same values (in any order); each distinct triplet
//! must appear only once. The triplets, and the values inside each triplet, may be returned in any
//! order.
//!
//! Implement `Solution::three_sum(nums: Vec<i32>) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [-1, 0, 1, 2, -1, -4]
//! Output: [[-1, -1, 2], [-1, 0, 1]]
//!
//! Input:  nums = [0, 1, 1]
//! Output: []
//!
//! Input:  nums = [0, 0, 0]
//! Output: [[0, 0, 0]]
//! ```
//!
//! ## Constraints
//!
//! - `3 <= nums.len() <= 3000`
//! - `-10^5 <= nums[i] <= 10^5`
//!
//! ## Hints
//!
//! 1. Fix the first value `a`; you then need two values that add up to `-a`. That is Two Sum II on
//!    the sorted rest of the array.
//! 2. To avoid duplicate triplets, never start with the same `a` twice, and after finding a triplet
//!    move the left pointer past equal values.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** sorting + two pointers: sort the values. For each index `i`, look for pairs in
//! `nums[i + 1..]` that sum to `-nums[i]` with the two-pointer scan from Two Sum II. Sorting also
//! makes duplicates adjacent, which gives two simple rules:
//! - skip `i` when `nums[i] == nums[i - 1]`: that first value was already fully explored;
//! - after recording a triplet, move `lo` past every copy of `nums[lo]`, so the same `(b, c)`
//!   pair is not recorded again (`c` is then determined).
//!
//! Once `nums[i] > 0`, all three values would be positive, so the loop can stop.
//!
//! **Complexity:** `O(n^2)` time (`n` scans of `O(n)` each), `O(1)` extra space besides the output.
//!
//! **Rust concepts:**
//! - `for (i, &first) in nums.iter().enumerate()` gives the index and the value together; the
//!   inner loop can still read `nums` because the vector is only borrowed immutably.
//! - The largest sum is `3 * 10^5`, so `i32` arithmetic cannot overflow here.
// @solution-end

use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // @solution-start
        let mut nums = nums;
        nums.sort_unstable();
        let mut triplets = Vec::new();
        for (i, &first) in nums.iter().enumerate() {
            if first > 0 {
                break; // every remaining value is positive
            }
            if i > 0 && first == nums[i - 1] {
                continue; // this first value was already explored
            }
            let (mut lo, mut hi) = (i + 1, nums.len() - 1);
            while lo < hi {
                match (first + nums[lo] + nums[hi]).cmp(&0) {
                    Ordering::Less => lo += 1,
                    Ordering::Greater => hi -= 1,
                    Ordering::Equal => {
                        triplets.push(vec![first, nums[lo], nums[hi]]);
                        lo += 1;
                        hi -= 1;
                        while lo < hi && nums[lo] == nums[lo - 1] {
                            lo += 1;
                        }
                    }
                }
            }
        }
        triplets
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sorts each triplet and the list of triplets, because any order is accepted.
    fn normalized(mut triplets: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for t in &mut triplets {
            t.sort_unstable();
        }
        triplets.sort();
        triplets
    }

    #[test]
    fn example_two_triplets() {
        assert_eq!(
            normalized(Solution::three_sum(vec![-1, 0, 1, 2, -1, -4])),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }

    #[test]
    fn example_no_triplet() {
        assert!(Solution::three_sum(vec![0, 1, 1]).is_empty());
    }

    #[test]
    fn example_all_zeros() {
        assert_eq!(
            normalized(Solution::three_sum(vec![0, 0, 0])),
            vec![vec![0, 0, 0]]
        );
    }

    #[test]
    fn duplicates_in_the_pair() {
        assert_eq!(
            normalized(Solution::three_sum(vec![-2, 0, 1, 1, 2])),
            vec![vec![-2, 0, 2], vec![-2, 1, 1]]
        );
    }

    #[test]
    fn many_zeros_give_one_triplet() {
        assert_eq!(
            normalized(Solution::three_sum(vec![0; 3000])),
            vec![vec![0, 0, 0]]
        );
    }

    #[test]
    fn symmetric_range() {
        // Values -3..=3: every triplet of distinct values summing to 0.
        let expected = vec![
            vec![-3, 0, 3],
            vec![-3, 1, 2],
            vec![-2, -1, 3],
            vec![-2, 0, 2],
            vec![-1, 0, 1],
        ];
        assert_eq!(
            normalized(Solution::three_sum((-3..=3).collect())),
            expected
        );
    }
}
