// ==================================================================
// PRACTICE 0947: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0947_range_sum_query_mutable.md
// Run the tests:          cargo test practice::p0947
// ==================================================================

//! # Problem 0947: Range Sum Query - Mutable
//!
//! - **Difficulty:** Medium
//! - **Topics:** Fenwick tree (binary indexed tree), lowest set bit `i & i.wrapping_neg()`, prefix sums under updates
//! - **Source:** [LeetCode 307. Range Sum Query - Mutable](https://leetcode.com/problems/range-sum-query-mutable/)
//!
//! ## Problem
//!
//! Keep an integer array that supports two operations, both fast even when they are interleaved:
//!
//! - `NumArray::new(nums: Vec<i32>) -> Self`.
//! - `update(&mut self, index: i32, val: i32)`: sets `nums[index] = val`.
//! - `sum_range(&self, left: i32, right: i32) -> i32`: the sum of `nums[left..=right]`.
//!
//! A plain prefix-sum array answers sums in `O(1)` but needs `O(n)` per update. Use a **Fenwick tree**
//! so that both operations cost `O(log n)`.
//!
//! ## Examples
//!
//! ```text
//! let mut a = NumArray::new(vec![1, 3, 5]);
//! a.sum_range(0, 2)   => 9
//! a.update(1, 2)      // nums = [1, 2, 5]
//! a.sum_range(0, 2)   => 8
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 3 * 10^4`
//! - `-100 <= nums[i], val <= 100`
//! - `0 <= index < nums.len()`, `0 <= left <= right < nums.len()`
//! - At most `3 * 10^4` calls to `update` and `sum_range`.
//!
//! ## Hints
//!
//! 1. Use 1-based positions inside the tree: `tree[i]` stores the sum of the `i & -i` elements ending
//!    at position `i`.
//! 2. Prefix sum of the first `i` elements: add `tree[i]`, then `i -= i & -i`, until `i == 0`.
//!    Point add: add to `tree[i]`, then `i += i & -i`, while `i <= n`.
//! 3. `update` needs the old value to compute the difference; keep a copy of `nums`.

pub struct NumArray {}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        todo!("Write your solution here")
    }

    pub fn update(&mut self, index: i32, val: i32) {
        todo!("Write your solution here")
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut a = NumArray::new(vec![1, 3, 5]);
        assert_eq!(a.sum_range(0, 2), 9);
        a.update(1, 2);
        assert_eq!(a.sum_range(0, 2), 8);
        assert_eq!(a.sum_range(1, 1), 2);
    }

    #[test]
    fn negative_values_and_single_element() {
        let mut a = NumArray::new(vec![-7]);
        assert_eq!(a.sum_range(0, 0), -7);
        a.update(0, 100);
        assert_eq!(a.sum_range(0, 0), 100);
        let b = NumArray::new(vec![-1, -2, 3, -4]);
        assert_eq!(b.sum_range(0, 3), -4);
        assert_eq!(b.sum_range(1, 2), 1);
    }

    #[test]
    fn matches_a_naive_model() {
        let mut model: Vec<i32> = (0..300).map(|i| (i * 37 % 201) - 100).collect();
        let mut a = NumArray::new(model.clone());
        for step in 0..2_000_usize {
            let i = step * 7 % model.len();
            let val = (step * 13 % 201) as i32 - 100;
            a.update(i as i32, val);
            model[i] = val;
            let (l, r) = (step % 300, (step * 11) % 300);
            let (l, r) = (l.min(r), l.max(r));
            let expected: i32 = model[l..=r].iter().sum();
            assert_eq!(a.sum_range(l as i32, r as i32), expected);
        }
    }

    #[test]
    fn largest_input() {
        let n = 30_000;
        let mut a = NumArray::new(vec![100; n]);
        assert_eq!(a.sum_range(0, n as i32 - 1), 3_000_000);
        a.update(0, -100);
        assert_eq!(a.sum_range(0, n as i32 - 1), 2_999_800);
        assert_eq!(a.sum_range(1, 1), 100);
    }
}
