// ==================================================================
// PRACTICE 0315: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0315_sort_integers_by_the_number_of_1_bits.md
// Run the tests:          cargo test practice::p0315
// ==================================================================

//! # Problem 0315: Sort Integers by The Number of 1 Bits
//!
//! - **Difficulty:** Easy
//! - **Topics:** `sort_unstable_by_key` with a tuple key, `count_ones`, lexicographic tuple order
//! - **Source:** [LeetCode 1356. Sort Integers by The Number of 1 Bits](https://leetcode.com/problems/sort-integers-by-the-number-of-1-bits/)
//!
//! ## Problem
//!
//! Sort the non-negative integers in `arr` by the number of `1` bits in their binary
//! representation, fewest first. When two numbers have the same number of `1` bits, the smaller
//! number comes first. Return the sorted vector.
//!
//! Implement `Solution::sort_by_bits(arr: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  arr = [0, 1, 2, 3, 4, 5, 6, 7, 8]
//! Output: [0, 1, 2, 4, 8, 3, 5, 6, 7]
//! Explanation: 0 has no 1 bits; 1, 2, 4, 8 have one; 3, 5, 6 have two; 7 has three.
//!
//! Input:  arr = [1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]
//! Output: [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= arr.len() <= 500`
//! - `0 <= arr[i] <= 10^4`
//!
//! ## Hints
//!
//! 1. `x.count_ones()` counts the `1` bits of an integer.
//! 2. Tuples compare element by element: `(a1, b1) < (a2, b2)` first compares `a1` with `a2`,
//!    and only looks at `b` when the `a`s are equal.

pub struct Solution;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            vec![0, 1, 2, 4, 8, 3, 5, 6, 7]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
            vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }

    #[test]
    fn duplicates_stay_together() {
        assert_eq!(
            Solution::sort_by_bits(vec![3, 1, 3, 1, 0]),
            vec![0, 1, 1, 3, 3]
        );
    }

    #[test]
    fn many_values_are_ordered_by_key() {
        let arr: Vec<i32> = (0..500).map(|i| (i * 37) % 10_001).collect();
        let result = Solution::sort_by_bits(arr.clone());
        assert_eq!(result.len(), 500);
        assert!(
            result
                .windows(2)
                .all(|w| (w[0].count_ones(), w[0]) <= (w[1].count_ones(), w[1]))
        );
        let mut a = arr;
        let mut b = result;
        a.sort_unstable();
        b.sort_unstable();
        assert_eq!(a, b);
    }
}
