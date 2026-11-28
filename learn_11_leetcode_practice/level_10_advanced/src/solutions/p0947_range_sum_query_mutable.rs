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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the Fenwick tree splits every prefix into `O(log n)` blocks whose sizes are powers of
//! two given by the binary representation of the position. Building adds every element once
//! (`O(n log n)`); `update` adds the difference between the new and old value along the "parent"
//! chain; `sum_range(l, r)` is `prefix(r + 1) - prefix(l)`.
//!
//! **Complexity:** `O(log n)` per `update` and `sum_range`; `O(n)` space.
//!
//! **Rust concepts:**
//! - `i & i.wrapping_neg()` isolates the lowest set bit of a `usize` (two's complement), the step
//!   size of the Fenwick tree.
//! - LeetCode passes indices as `i32`; convert them once with `as usize` (they are non-negative by
//!   the constraints).
//! - Keeping the helper methods private hides the tree layout from users of `NumArray`.
// @solution-end

pub struct NumArray {
    // @solution-start remove
    values: Vec<i32>,
    tree: Vec<i32>,
    // @solution-end
}

// @solution-start remove
impl NumArray {
    /// Adds `delta` at 0-based position `index`.
    fn add(&mut self, index: usize, delta: i32) {
        let mut i = index + 1;
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    /// Sum of the first `count` elements.
    fn prefix(&self, count: usize) -> i32 {
        let mut i = count;
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        sum
    }
}
// @solution-end

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        // @solution-start
        let mut array = NumArray {
            values: vec![0; nums.len()],
            tree: vec![0; nums.len() + 1],
        };
        for (i, &value) in nums.iter().enumerate() {
            array.update(i as i32, value);
        }
        array
        // @solution-end
    }

    pub fn update(&mut self, index: i32, val: i32) {
        // @solution-start
        let index = index as usize;
        let delta = val - self.values[index];
        self.values[index] = val;
        self.add(index, delta);
        // @solution-end
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        // @solution-start
        self.prefix(right as usize + 1) - self.prefix(left as usize)
        // @solution-end
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
