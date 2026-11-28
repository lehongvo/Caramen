//! # Problem 0488: Product of Array Except Self
//!
//! - **Difficulty:** Medium
//! - **Topics:** exclusive prefix products with `scan`, suffix products with `rev` + `scan`, `zip` to combine them
//! - **Source:** [LeetCode 238. Product of Array Except Self](https://leetcode.com/problems/product-of-array-except-self/)
//!
//! ## Problem
//!
//! Given an integer array `nums`, return an array `answer` where `answer[i]` is the product of all
//! elements of `nums` **except** `nums[i]`. Do it in `O(n)` time and **without** the division
//! operator.
//!
//! Implement `Solution::product_except_self(nums: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [1, 2, 3, 4]
//! Output: [24, 12, 8, 6]
//!
//! Input:  nums = [-1, 1, 0, -3, 3]
//! Output: [0, 0, 9, 0, 0]
//! ```
//!
//! ## Constraints
//!
//! - `2 <= nums.len() <= 10^5`
//! - `-30 <= nums[i] <= 30`
//! - The product of any prefix or suffix of `nums` fits in an `i32`.
//!
//! ## Hints
//!
//! 1. `answer[i]` = (product of everything **before** `i`) * (product of everything **after** `i`).
//! 2. An *exclusive* running product: `scan(1, |acc, &x| { let before = *acc; *acc *= x; Some(before) })`
//!    yields the product of the elements before each position.
//! 3. Run the same scan on `nums.iter().rev()` for the products after each position (they come out
//!    in reverse order).
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** compute the products of the elements to the left of every position and to the right of
//! every position, then multiply them pairwise. Both are running products, i.e. `scan`s, one from
//! each end.
//!
//! **Complexity:** `O(n)` time, `O(n)` extra space for the suffix products (the output does not
//! count).
//!
//! **Rust concepts:**
//! - An exclusive scan emits the state **before** updating it.
//! - `iter().rev()` walks a slice backwards; collecting it gives the suffix products in reverse
//!   order, and `.iter().rev()` on that vector restores the original order for `zip`.
//! - The guarantee about prefix and suffix products means none of the multiplications overflows.
// @solution-end

pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // @solution-start
        let exclusive = |acc: &mut i32, &x: &i32| {
            let before = *acc;
            *acc *= x;
            Some(before)
        };
        let suffix: Vec<i32> = nums.iter().rev().scan(1, exclusive).collect();
        nums.iter()
            .scan(1, exclusive)
            .zip(suffix.iter().rev())
            .map(|(left, right)| left * right)
            .collect()
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_increasing() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }

    #[test]
    fn example_with_zero() {
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }

    #[test]
    fn two_elements_and_two_zeros() {
        assert_eq!(Solution::product_except_self(vec![3, -4]), vec![-4, 3]);
        assert_eq!(Solution::product_except_self(vec![0, 5, 0]), vec![0, 0, 0]);
    }

    #[test]
    fn matches_a_brute_force_check() {
        let nums = vec![2, -1, 3, 1, -2, 1, 1, 2];
        let expected: Vec<i32> = (0..nums.len())
            .map(|i| {
                nums.iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, &x)| x)
                    .product()
            })
            .collect();
        assert_eq!(Solution::product_except_self(nums), expected);
    }

    #[test]
    fn large_input_of_ones() {
        let mut nums = vec![1; 100_000];
        nums[500] = -1;
        let result = Solution::product_except_self(nums);
        assert_eq!(result[500], 1);
        assert_eq!(result[0], -1);
        assert_eq!(result.len(), 100_000);
    }
}
