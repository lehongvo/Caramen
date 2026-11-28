// ==================================================================
// PRACTICE 0039: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0039_min_and_max_of_a_slice.md
// Run the tests:          cargo test practice::p0039
// ==================================================================

//! # Problem 0039: Min and Max of a Slice
//!
//! - **Difficulty:** Easy
//! - **Topics:** slices `&[i32]`, reading by index and by reference, tuples inside `Option`, `abs_diff`
//! - **Source:** Original · [The Rust Book 4.3: slices](https://doc.rust-lang.org/book/ch04-03-slices.html)
//!
//! ## Problem
//!
//! Write two functions over a slice of integers:
//!
//! 1. `min_max(nums: &[i32]) -> Option<(i32, i32)>` returns `Some((smallest, largest))`, or `None`
//!    when the slice is empty. Use a single pass over the data.
//! 2. `spread(nums: &[i32]) -> u32` returns `largest - smallest` as a `u32`, or `0` for an empty
//!    slice. It must work for **every** input, including `[i32::MIN, i32::MAX]`, whose spread
//!    (`4294967295`) does not fit in an `i32`.
//!
//! ## Examples
//!
//! ```text
//! min_max(&[3, -1, 7, 2])         -> Some((-1, 7))
//! min_max(&[5])                   -> Some((5, 5))
//! min_max(&[])                    -> None
//!
//! spread(&[3, -1, 7, 2])          -> 8
//! spread(&[i32::MIN, i32::MAX])   -> 4294967295
//! spread(&[])                     -> 0
//! ```
//!
//! ## Constraints
//!
//! - `0 <= nums.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. Start both the minimum and the maximum at the **first** element `nums[0]` (after checking that it exists).
//! 2. A `&[i32]` can be iterated with `for &x in nums` (by reference, copying each value) or by
//!    index with `for i in 1..nums.len()`.
//! 3. `a.abs_diff(b)` returns the distance between two `i32` values as a `u32`, without overflow.

pub fn min_max(nums: &[i32]) -> Option<(i32, i32)> {
    todo!("Write your solution here")
}

pub fn spread(nums: &[i32]) -> u32 {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_max_examples() {
        assert_eq!(min_max(&[3, -1, 7, 2]), Some((-1, 7)));
        assert_eq!(min_max(&[5]), Some((5, 5)));
        assert_eq!(min_max(&[]), None);
    }

    #[test]
    fn works_with_arrays_and_vectors() {
        let array = [4, 4, 4];
        let vector = vec![10, -10, 0];
        assert_eq!(min_max(&array), Some((4, 4)));
        assert_eq!(min_max(&vector), Some((-10, 10)));
        assert_eq!(min_max(&vector[2..]), Some((0, 0)));
    }

    #[test]
    fn spread_examples() {
        assert_eq!(spread(&[3, -1, 7, 2]), 8);
        assert_eq!(spread(&[]), 0);
        assert_eq!(spread(&[42]), 0);
    }

    #[test]
    fn spread_extremes_do_not_overflow() {
        assert_eq!(spread(&[i32::MIN, i32::MAX]), 4_294_967_295);
        assert_eq!(spread(&[i32::MAX, 0, i32::MIN]), u32::MAX);
    }

    #[test]
    fn large_input() {
        let nums: Vec<i32> = (0..100_000)
            .map(|i| (i * 7919) % 100_003 - 50_000)
            .collect();
        let (lo, hi) = min_max(&nums).expect("non-empty");
        assert_eq!(lo, *nums.iter().min().expect("non-empty"));
        assert_eq!(hi, *nums.iter().max().expect("non-empty"));
    }
}
