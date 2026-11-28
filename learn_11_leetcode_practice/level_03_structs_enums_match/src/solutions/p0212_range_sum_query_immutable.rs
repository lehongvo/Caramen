//! # Problem 0212: Range Sum Query - Immutable
//!
//! - **Difficulty:** Easy
//! - **Topics:** design struct, precomputing in `new`, `&self` queries, prefix sums
//! - **Source:** [LeetCode 303. Range Sum Query - Immutable](https://leetcode.com/problems/range-sum-query-immutable/)
//!
//! ## Problem
//!
//! You receive an array of integers once, and then many questions of the form "what is the sum of
//! the elements from index `left` to index `right`, both included?". The array never changes.
//!
//! Implement the `NumArray` struct:
//!
//! - `NumArray::new(nums: Vec<i32>) -> Self` stores whatever it needs about `nums`.
//! - `sum_range(&self, left: i32, right: i32) -> i32` returns
//!   `nums[left] + nums[left + 1] + ... + nums[right]`.
//!
//! Queries only read the object, so `sum_range` takes `&self`. Make each query fast: do the heavy
//! work once in `new`. This is a LeetCode *design* problem, so there is no `impl Solution`: paste the
//! struct and its `impl NumArray` block into LeetCode.
//!
//! ## Examples
//!
//! ```text
//! let a = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
//! a.sum_range(0, 2) => 1     (-2 + 0 + 3)
//! a.sum_range(2, 5) => -1    (3 - 5 + 2 - 1)
//! a.sum_range(0, 5) => -3
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 10^4`
//! - `-10^5 <= nums[i] <= 10^5`
//! - `0 <= left <= right < nums.len()`
//! - At most `10^4` calls to `sum_range`.
//!
//! ## Hints
//!
//! 1. Store `prefix[i]` = sum of the first `i` elements (so `prefix[0] = 0`).
//! 2. The sum of `left..=right` is then `prefix[right + 1] - prefix[left]`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** build a prefix-sum vector of length `n + 1` in the constructor. Each query subtracts
//! two prefix values.
//!
//! **Complexity:** `O(n)` time and space in `new`; `O(1)` per query.
//!
//! **Rust concepts:**
//! - The struct keeps only the derived data (`prefix`); `nums` is consumed by `new` and dropped.
//! - Building the vector with `Vec::with_capacity` and `push` avoids reallocations.
//! - All sums fit in `i32`: at most `10^4 * 10^5 = 10^9` in absolute value.
// @solution-end

/// Answers range-sum queries on a fixed array.
pub struct NumArray {
    // @solution-start remove
    prefix: Vec<i32>,
    // @solution-end
}

impl NumArray {
    /// Preprocesses `nums`.
    pub fn new(nums: Vec<i32>) -> Self {
        // @solution-start
        let mut prefix = Vec::with_capacity(nums.len() + 1);
        prefix.push(0);
        let mut running = 0;
        for x in nums {
            running += x;
            prefix.push(running);
        }
        Self { prefix }
        // @solution-end
    }

    /// Sum of the elements with indices `left..=right`.
    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        // @solution-start
        self.prefix[right as usize + 1] - self.prefix[left as usize]
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_queries() {
        let a = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(a.sum_range(0, 2), 1);
        assert_eq!(a.sum_range(2, 5), -1);
        assert_eq!(a.sum_range(0, 5), -3);
    }

    #[test]
    fn single_element_ranges() {
        let a = NumArray::new(vec![7, -3, 9]);
        assert_eq!(a.sum_range(0, 0), 7);
        assert_eq!(a.sum_range(1, 1), -3);
        assert_eq!(a.sum_range(2, 2), 9);
    }

    #[test]
    fn single_element_array() {
        let a = NumArray::new(vec![-100_000]);
        assert_eq!(a.sum_range(0, 0), -100_000);
    }

    #[test]
    fn large_array_extremes() {
        let a = NumArray::new(vec![100_000; 10_000]);
        assert_eq!(a.sum_range(0, 9_999), 1_000_000_000);
        assert_eq!(a.sum_range(5_000, 5_009), 1_000_000);
        for i in 0..10_000 {
            assert_eq!(a.sum_range(i, i), 100_000);
        }
    }
}
