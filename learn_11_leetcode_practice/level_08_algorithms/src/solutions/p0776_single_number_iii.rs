//! # Problem 0776: Single Number III
//!
//! - **Difficulty:** Medium
//! - **Topics:** XOR of everything, isolating the lowest set bit `x & x.wrapping_neg()`, partitioning
//! - **Source:** [LeetCode 260. Single Number III](https://leetcode.com/problems/single-number-iii/)
//!
//! ## Problem
//!
//! In the vector `nums`, exactly **two** values appear once and every other value appears exactly
//! twice. Return the two single values, in any order. Use linear time and constant extra memory.
//!
//! Implement `Solution::single_number(nums: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 1, 3, 2, 5]
//! Output: [3, 5]        ([5, 3] is also accepted)
//!
//! Input:  nums = [-1, 0]
//! Output: [-1, 0]
//! ```
//!
//! ## Constraints
//!
//! - `2 <= nums.len() <= 3 * 10^4`
//! - `-2^31 <= nums[i] <= 2^31 - 1`
//! - Exactly two values appear once; every other value appears twice.
//!
//! ## Hints
//!
//! 1. XOR all the values: pairs cancel out, leaving `a ^ b` for the two single values `a` and `b`.
//! 2. `a != b`, so `a ^ b` has at least one set bit, where `a` and `b` differ. Use it to split the
//!    values into two groups.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** XOR + partition by one bit: `x ^ x == 0`, so XOR-ing everything leaves `a ^ b`.
//! Pick any set bit of it, for instance the lowest one, `diff & -diff` in two's complement.
//! `a` has that bit and `b` does not (or the reverse). Splitting all values by that bit puts `a`
//! and `b` in different groups, while both copies of every other value land in the same group. So
//! the XOR of one group is `a`, and `b = (a ^ b) ^ a`.
//!
//! **Complexity:** `O(n)` time, `O(1)` space.
//!
//! **Rust concepts:**
//! - `diff.wrapping_neg()` is `-diff` without the overflow panic when `diff == i32::MIN` (whose
//!   negation does not fit in `i32`). `diff & diff.wrapping_neg()` keeps only the lowest set bit.
//! - `fold(0, |acc, &x| acc ^ x)` XORs an iterator together.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // @solution-start
        let diff = nums.iter().fold(0, |acc, &x| acc ^ x); // a ^ b, never 0
        let bit = diff & diff.wrapping_neg(); // lowest bit where a and b differ
        let a = nums
            .iter()
            .filter(|&&x| x & bit != 0)
            .fold(0, |acc, &x| acc ^ x);
        vec![a, diff ^ a]
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Solution::single_number(nums);
        result.sort_unstable(); // any order is accepted
        result
    }

    #[test]
    fn example_six_values() {
        assert_eq!(run(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
    }

    #[test]
    fn example_two_values() {
        assert_eq!(run(vec![-1, 0]), vec![-1, 0]);
        assert_eq!(run(vec![0, 1]), vec![0, 1]);
    }

    #[test]
    fn extreme_values() {
        // a ^ b has the sign bit set: the lowest set bit must still be found safely.
        assert_eq!(
            run(vec![i32::MIN, 5, 5, i32::MAX]),
            vec![i32::MIN, i32::MAX]
        );
        assert_eq!(run(vec![i32::MIN, 0]), vec![i32::MIN, 0]);
    }

    #[test]
    fn values_differing_only_in_a_high_bit() {
        assert_eq!(run(vec![7, (1 << 20) | 7, 3, 3]), vec![7, (1 << 20) | 7]);
    }

    #[test]
    fn long_input() {
        // -7_499..=7_499 twice each (29_998 values), plus the two singles.
        let mut nums: Vec<i32> = (-7_499..=7_499).flat_map(|i| [i, i]).collect();
        nums.push(100_000);
        nums.push(-100_000);
        assert_eq!(run(nums), vec![-100_000, 100_000]);
    }
}
