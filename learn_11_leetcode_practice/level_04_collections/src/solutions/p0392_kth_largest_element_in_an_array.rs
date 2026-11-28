//! # Problem 0392: Kth Largest Element in an Array
//!
//! - **Difficulty:** Medium
//! - **Topics:** a min-heap of size `k` with `Reverse`, `BinaryHeap::peek`, `select_nth_unstable` (quickselect)
//! - **Source:** [LeetCode 215. Kth Largest Element in an Array](https://leetcode.com/problems/kth-largest-element-in-an-array/)
//!
//! ## Problem
//!
//! Given a vector `nums` and an integer `k`, return the `k`-th largest element **in sorted order**
//! (not the `k`-th distinct element: in `[3, 3, 2]` the 2nd largest is `3`).
//!
//! LeetCode asks whether you can solve it without sorting the whole vector.
//!
//! Implement `Solution::find_kth_largest(nums: Vec<i32>, k: i32) -> i32`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [3, 2, 1, 5, 6, 4], k = 2
//! Output: 5
//!
//! Input:  nums = [3, 2, 3, 1, 2, 4, 5, 5, 6], k = 4
//! Output: 4      (sorted descending: 6 5 5 4 3 3 2 2 1)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= k <= nums.len() <= 10^5`
//! - `-10^4 <= nums[i] <= 10^4`
//!
//! ## Hints
//!
//! 1. Keep the `k` largest values seen so far in a min-heap; its top is the `k`-th largest.
//! 2. Alternative: `select_nth_unstable(i)` moves the element that belongs at index `i` of the
//!    sorted order into place in `O(n)` average time.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** scan the values, pushing each into a min-heap (`BinaryHeap<Reverse<i32>>`) and popping
//! the smallest whenever the heap holds more than `k`. At the end the heap contains the `k` largest
//! values and its top, the smallest of them, is the answer.
//!
//! **Complexity:** `O(n log k)` time, `O(k)` space.
//!
//! The quickselect alternative is a one-liner on a mutable vector:
//! `*nums.select_nth_unstable(nums.len() - k).1` (average `O(n)`, in place).
//!
//! **Rust concepts:**
//! - The bounded min-heap is the same "top-k" pattern as in "Kth Largest Element in a Stream", now
//!   used once over a fixed input.
//! - `peek()` returns `Option<&Reverse<i32>>`; `map_or(0, |r| r.0)` reads the wrapped value (`.0` is
//!   the field of the tuple struct `Reverse`).
//! - `select_nth_unstable` returns `(&mut [T], &mut T, &mut [T])`: the elements before, the element
//!   at the index, and the elements after.
// @solution-end

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        // @solution-start
        let k = k as usize;
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k + 1);
        for x in nums {
            heap.push(Reverse(x));
            if heap.len() > k {
                heap.pop();
            }
        }
        heap.peek().map_or(0, |r| r.0)
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }

    #[test]
    fn example_two_duplicates() {
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
    }

    #[test]
    fn k_equals_one_and_length() {
        assert_eq!(Solution::find_kth_largest(vec![-1, -5, -3], 1), -1);
        assert_eq!(Solution::find_kth_largest(vec![-1, -5, -3], 3), -5);
        assert_eq!(Solution::find_kth_largest(vec![7], 1), 7);
    }

    #[test]
    fn large_input() {
        let nums: Vec<i32> = (0..100_000)
            .map(|i| (i * 7_919) % 20_001 - 10_000)
            .collect();
        let mut sorted = nums.clone();
        sorted.sort_unstable_by(|a, b| b.cmp(a));
        for k in [1, 500, 50_000, 100_000] {
            assert_eq!(
                Solution::find_kth_largest(nums.clone(), k as i32),
                sorted[k - 1]
            );
        }
    }
}
