// ==================================================================
// PRACTICE 0763: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0763_count_inversions.md
// Run the tests:          cargo test practice::p0763
// ==================================================================

//! # Problem 0763: Count Inversions
//!
//! - **Difficulty:** Medium
//! - **Topics:** merge sort with counting (divide and conquer), `u64` results, `to_vec`
//! - **Source:** Original · [`slice::split_at`](https://doc.rust-lang.org/std/primitive.slice.html#method.split_at)
//!
//! ## Problem
//!
//! An **inversion** in a sequence is a pair of positions `i < j` with `nums[i] > nums[j]`: two
//! elements in the wrong order. The number of inversions measures how far a sequence is from being
//! sorted (it is also the number of swaps bubble sort would make).
//!
//! Write `count_inversions(nums: &[i32]) -> u64` that returns the number of inversions of `nums`
//! in `O(n log n)` time. Do not modify the input (it is borrowed immutably). Equal elements do not
//! form an inversion.
//!
//! ## Examples
//!
//! ```text
//! count_inversions(&[2, 4, 1, 3, 5]) = 3     (pairs (2, 1), (4, 1), (4, 3))
//! count_inversions(&[1, 2, 3])       = 0
//! count_inversions(&[3, 2, 1])       = 3
//! count_inversions(&[5, 5, 1])       = 2     (each 5 with the 1; the two 5s are not inverted)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= nums.len() <= 10^5`
//! - Any `i32` values.
//! - A reversed array of `10^5` elements has `4_999_950_000` inversions, more than `u32::MAX`.
//!
//! ## Hints
//!
//! 1. Checking every pair is `O(n^2)`: about `5 * 10^9` comparisons for the largest input.
//! 2. Split the sequence in two halves. Inversions are inside the left half, inside the right half,
//!    or across the halves. The first two kinds are counted recursively.
//! 3. While merging two **sorted** halves, when you take an element from the right half, how many
//!    elements of the left half are greater than it?

pub fn count_inversions(nums: &[i32]) -> u64 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute_force(nums: &[i32]) -> u64 {
        nums.iter()
            .enumerate()
            .map(|(i, &a)| nums[i + 1..].iter().filter(|&&b| a > b).count() as u64)
            .sum()
    }

    #[test]
    fn example_three_inversions() {
        assert_eq!(count_inversions(&[2, 4, 1, 3, 5]), 3);
    }

    #[test]
    fn example_sorted_and_reversed() {
        assert_eq!(count_inversions(&[1, 2, 3]), 0);
        assert_eq!(count_inversions(&[3, 2, 1]), 3);
    }

    #[test]
    fn example_equal_values() {
        assert_eq!(count_inversions(&[5, 5, 1]), 2);
        assert_eq!(count_inversions(&[2, 2, 2]), 0);
    }

    #[test]
    fn empty_and_single() {
        assert_eq!(count_inversions(&[]), 0);
        assert_eq!(count_inversions(&[i32::MIN]), 0);
    }

    #[test]
    fn input_is_not_modified() {
        let nums = vec![3, 1, 2];
        assert_eq!(count_inversions(&nums), 2);
        assert_eq!(nums, vec![3, 1, 2]);
    }

    #[test]
    fn matches_brute_force() {
        let nums: Vec<i32> = (0..400).map(|i: i32| (i * 7_919) % 101 - 50).collect();
        assert_eq!(count_inversions(&nums), brute_force(&nums));
    }

    #[test]
    fn large_reversed_input_exceeds_u32() {
        let nums: Vec<i32> = (0..100_000).rev().collect();
        assert_eq!(count_inversions(&nums), 4_999_950_000);
    }
}
