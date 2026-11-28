//! # Problem 0762: Sort an Array
//!
//! - **Difficulty:** Medium
//! - **Topics:** merge sort, recursion on sub-slices, one reusable buffer, `split_at`, `copy_from_slice`
//! - **Source:** [LeetCode 912. Sort an Array](https://leetcode.com/problems/sort-an-array/)
//!
//! ## Problem
//!
//! Sort the vector `nums` in ascending order and return it, **without** using any built-in sort
//! (`sort`, `sort_unstable`, `BinaryHeap`, ...). Your algorithm must run in `O(n log n)` time and
//! use as little extra memory as reasonably possible. Implement **merge sort**.
//!
//! Implement `Solution::sort_array(nums: Vec<i32>) -> Vec<i32>`.
//!
//! ## Examples
//!
//! ```text
//! Input:  nums = [5, 2, 3, 1]
//! Output: [1, 2, 3, 5]
//!
//! Input:  nums = [5, 1, 1, 2, 0, 0]
//! Output: [0, 0, 1, 1, 2, 5]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= nums.len() <= 5 * 10^4`
//! - `-5 * 10^4 <= nums[i] <= 5 * 10^4`
//!
//! ## Hints
//!
//! 1. Merge sort: sort the left half, sort the right half, then merge the two sorted halves.
//! 2. Merging needs scratch space. Allocate **one** buffer of length `n` up front and pass the
//!    matching part of it down the recursion, instead of allocating new vectors at every level.
//! 3. A slice of length 0 or 1 is already sorted: that is the base case.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** top-down merge sort (divide and conquer): `merge_sort(v, buf)` sorts the slice `v`
//! using `buf` (same length) as scratch space. It splits `v` at `mid`, sorts both halves
//! recursively (each with the matching half of `buf`), then merges: repeatedly take the smaller
//! front element of the two sorted halves into `buf`, and finally copy `buf` back into `v`.
//! Taking from the left half on ties (`<=`) makes the sort **stable**.
//!
//! The recursion depth is `log2(n)` (about 16 for `5 * 10^4`), so there is no risk of overflowing
//! the stack.
//!
//! **Complexity:** `O(n log n)` time, `O(n)` extra space (the single buffer) plus `O(log n)` for
//! the recursion.
//!
//! **Rust concepts:**
//! - Recursing on `&mut v[..mid]` and `&mut v[mid..]` hands out two non-overlapping mutable
//!   sub-slices, one after the other; the borrow checker is satisfied because each borrow ends
//!   before the next one starts.
//! - `v.split_at(mid)` gives the two halves as shared slices for the merge, and
//!   [`copy_from_slice`](https://doc.rust-lang.org/std/primitive.slice.html#method.copy_from_slice)
//!   copies the merged result back (both slices must have the same length).
// @solution-end

pub struct Solution;

// @solution-start remove
/// Sorts `v` in place; `buf` is scratch space of the same length.
fn merge_sort(v: &mut [i32], buf: &mut [i32]) {
    let n = v.len();
    if n <= 1 {
        return;
    }
    let mid = n / 2;
    merge_sort(&mut v[..mid], &mut buf[..mid]);
    merge_sort(&mut v[mid..], &mut buf[mid..]);
    let (left, right) = v.split_at(mid);
    let (mut i, mut j) = (0, 0);
    for slot in buf.iter_mut() {
        // Take from the left half when the right one is exhausted or on ties (stable).
        if j == right.len() || (i < left.len() && left[i] <= right[j]) {
            *slot = left[i];
            i += 1;
        } else {
            *slot = right[j];
            j += 1;
        }
    }
    v.copy_from_slice(buf);
}
// @solution-end

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        // @solution-start
        let mut nums = nums;
        let mut buf = vec![0; nums.len()];
        merge_sort(&mut nums, &mut buf);
        nums
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A small deterministic pseudo-random generator (linear congruential), so tests need no crate.
    fn pseudo_random(n: usize, seed: u64) -> Vec<i32> {
        let mut state = seed;
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1);
                ((state >> 33) % 100_001) as i32 - 50_000
            })
            .collect()
    }

    #[test]
    fn example_four_values() {
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
    }

    #[test]
    fn example_duplicates() {
        assert_eq!(
            Solution::sort_array(vec![5, 1, 1, 2, 0, 0]),
            vec![0, 0, 1, 1, 2, 5]
        );
    }

    #[test]
    fn single_value() {
        assert_eq!(Solution::sort_array(vec![1]), vec![1]);
    }

    #[test]
    fn negative_and_extreme_values() {
        assert_eq!(
            Solution::sort_array(vec![50_000, -50_000, 0, -1, 1]),
            vec![-50_000, -1, 0, 1, 50_000]
        );
    }

    #[test]
    fn sorted_and_reversed_inputs() {
        let sorted: Vec<i32> = (0..1000).collect();
        assert_eq!(Solution::sort_array(sorted.clone()), sorted);
        assert_eq!(
            Solution::sort_array(sorted.iter().rev().copied().collect()),
            sorted
        );
    }

    #[test]
    fn large_random_input_matches_std_sort() {
        let nums = pseudo_random(50_000, 42);
        let mut expected = nums.clone();
        expected.sort_unstable();
        assert_eq!(Solution::sort_array(nums), expected);
    }
}
