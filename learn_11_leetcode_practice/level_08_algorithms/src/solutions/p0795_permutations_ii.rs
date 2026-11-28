//! # Problem 0795: Permutations II
//!
//! - **Difficulty:** Medium
//! - **Topics:** backtracking with duplicates, sorted input, the "previous equal value must be used" rule
//! - **Source:** [LeetCode 47. Permutations II](https://leetcode.com/problems/permutations-ii/)
//!
//! ## Problem
//!
//! Given a vector `nums` that **may contain duplicates**, return all **distinct** permutations of
//! its values, in any order.
//!
//! Implement `Solution::permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 1, 2]
//! Output: [[1, 1, 2], [1, 2, 1], [2, 1, 1]]
//!
//! Input:  nums = [1, 2, 3]
//! Output: [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 8`
//! - `-10 <= nums[i] <= 10`
//!
//! ## Hints
//!
//! 1. Generating all `n!` orderings and removing duplicates with a set works but wastes time.
//! 2. Sort the values. Treat equal values as if they had to be used **in their original order**:
//!    a copy may only be placed after the previous equal copy has been placed.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** backtracking + order rule for equal values: sort, then run the Permutations
//! backtracking with one extra rule: skip `nums[i]` when `nums[i] == nums[i - 1]` and
//! `nums[i - 1]` is **not** used yet. This forces equal values to be placed in index order, so the
//! `c!` arrangements of `c` equal copies collapse into one: each distinct permutation is built
//! exactly once, and no duplicate is ever generated.
//!
//! **Complexity:** `O(n * P)` time, where `P` is the number of distinct permutations (at most
//! `n!`), `O(n)` extra space besides the output.
//!
//! **Rust concepts:**
//! - The rule is a single boolean expression with short-circuit `&&`, so `nums[i - 1]` and
//!   `used[i - 1]` are only read when `i > 0`.
//! - Sorting with `sort_unstable` is fine: equal integers are indistinguishable.
// @solution-end

pub struct Solution;

// @solution-start remove
fn backtrack(nums: &[i32], used: &mut [bool], current: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    if current.len() == nums.len() {
        result.push(current.clone());
        return;
    }
    for i in 0..nums.len() {
        // Equal values are placed in index order: copy i needs copy i - 1 to be placed already.
        if used[i] || (i > 0 && nums[i] == nums[i - 1] && !used[i - 1]) {
            continue;
        }
        used[i] = true;
        current.push(nums[i]);
        backtrack(nums, used, current, result);
        current.pop();
        used[i] = false;
    }
}
// @solution-end

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // @solution-start
        let mut nums = nums;
        nums.sort_unstable();
        let mut used = vec![false; nums.len()];
        let mut result = Vec::new();
        backtrack(
            &nums,
            &mut used,
            &mut Vec::with_capacity(nums.len()),
            &mut result,
        );
        result
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut perms: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        perms.sort();
        perms
    }

    #[test]
    fn example_with_duplicates() {
        assert_eq!(
            sorted(Solution::permute_unique(vec![1, 1, 2])),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );
    }

    #[test]
    fn example_distinct_values() {
        assert_eq!(Solution::permute_unique(vec![1, 2, 3]).len(), 6);
    }

    #[test]
    fn all_equal() {
        assert_eq!(Solution::permute_unique(vec![2, 2, 2]), vec![vec![2, 2, 2]]);
    }

    #[test]
    fn two_pairs() {
        // 4! / (2! * 2!) = 6
        let perms = sorted(Solution::permute_unique(vec![2, 1, 2, 1]));
        assert_eq!(perms.len(), 6);
        let mut deduped = perms.clone();
        deduped.dedup();
        assert_eq!(deduped, perms);
    }

    #[test]
    fn eight_values() {
        // 8! / (4! * 4!) = 70 distinct arrangements, and 8! = 40_320 for distinct values.
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 1, 1, 2, 2, 2, 2]).len(),
            70
        );
        assert_eq!(Solution::permute_unique((1..=8).collect()).len(), 40_320);
    }
}
