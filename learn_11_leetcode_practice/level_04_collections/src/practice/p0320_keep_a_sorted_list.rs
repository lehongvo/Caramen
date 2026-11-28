// ==================================================================
// PRACTICE 0320: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0320_keep_a_sorted_list.md
// Run the tests:          cargo test practice::p0320
// ==================================================================

//! # Problem 0320: Keep a Sorted List
//!
//! - **Difficulty:** Easy
//! - **Topics:** `slice::partition_point`, `binary_search`, `Vec::insert`/`remove` on sorted data
//! - **Source:** Original · [`slice::partition_point`](https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point) · [`slice::binary_search`](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search)
//!
//! ## Problem
//!
//! A vector `list` is kept sorted in ascending order (duplicates allowed). Implement three
//! operations that each run a binary search instead of scanning the whole vector:
//!
//! 1. `insert_sorted(list: &mut Vec<i32>, value: i32) -> usize` inserts `value` so that the list
//!    stays sorted, placing it **after** any elements equal to it, and returns the index where it
//!    was inserted.
//! 2. `remove_one(list: &mut Vec<i32>, value: i32) -> bool` removes one copy of `value` if there is
//!    one and returns `true`; otherwise leaves the list unchanged and returns `false`.
//! 3. `count_in_range(list: &[i32], low: i32, high: i32) -> usize` returns how many elements `x`
//!    satisfy `low <= x <= high` (`0` if `low > high`).
//!
//! ## Examples
//!
//! ```text
//! list = [1, 3, 3, 7]
//! insert_sorted(list, 3)    => 3, list becomes [1, 3, 3, 3, 7]
//! insert_sorted(list, 0)    => 0, list becomes [0, 1, 3, 3, 3, 7]
//! remove_one(list, 3)       => true, list becomes [0, 1, 3, 3, 7]
//! remove_one(list, 5)       => false
//! count_in_range(list, 1, 3) => 3     (1, 3, 3)
//! count_in_range(list, 4, 6) => 0
//! ```
//!
//! ## Constraints
//!
//! - `0 <= list.len() <= 10^5`; `list` is sorted ascending before every call.
//!
//! ## Hints
//!
//! 1. `partition_point(|&x| pred(x))` returns the first index where `pred` becomes `false`
//!    (the predicate must be `true` for a prefix and `false` after it).
//! 2. `partition_point(|&x| x <= value)` is "the index just after the last element `<= value`".
//! 3. The count of elements in `[low, high]` is the difference of two partition points.

/// Inserts `value` after any equal elements and returns its index.
pub fn insert_sorted(list: &mut Vec<i32>, value: i32) -> usize {
    todo!("Write your solution here")
}

/// Removes one copy of `value`; returns whether something was removed.
pub fn remove_one(list: &mut Vec<i32>, value: i32) -> bool {
    todo!("Write your solution here")
}

/// Counts the elements `x` with `low <= x <= high`.
pub fn count_in_range(list: &[i32], low: i32, high: i32) -> usize {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut list = vec![1, 3, 3, 7];
        assert_eq!(insert_sorted(&mut list, 3), 3);
        assert_eq!(list, vec![1, 3, 3, 3, 7]);
        assert_eq!(insert_sorted(&mut list, 0), 0);
        assert_eq!(list, vec![0, 1, 3, 3, 3, 7]);
        assert!(remove_one(&mut list, 3));
        assert_eq!(list, vec![0, 1, 3, 3, 7]);
        assert!(!remove_one(&mut list, 5));
        assert_eq!(list, vec![0, 1, 3, 3, 7]);
        assert_eq!(count_in_range(&list, 1, 3), 3);
        assert_eq!(count_in_range(&list, 4, 6), 0);
    }

    #[test]
    fn empty_list() {
        let mut list = Vec::new();
        assert!(!remove_one(&mut list, 1));
        assert_eq!(count_in_range(&list, -5, 5), 0);
        assert_eq!(insert_sorted(&mut list, 9), 0);
        assert_eq!(list, vec![9]);
    }

    #[test]
    fn reversed_range_counts_zero() {
        assert_eq!(count_in_range(&[1, 2, 3, 4], 3, 2), 0);
        assert_eq!(count_in_range(&[1, 2, 3, 4], i32::MIN, i32::MAX), 4);
    }

    #[test]
    fn many_inserts_stay_sorted() {
        let mut list = Vec::new();
        for i in 0..2000 {
            insert_sorted(&mut list, (i * 7919) % 500);
        }
        assert!(list.windows(2).all(|w| w[0] <= w[1]));
        assert_eq!(count_in_range(&list, 0, 499), 2000);
        assert_eq!(count_in_range(&list, 100, 199), 400);
        for _ in 0..4 {
            assert!(remove_one(&mut list, 42));
        }
        assert!(!remove_one(&mut list, 42));
    }
}
