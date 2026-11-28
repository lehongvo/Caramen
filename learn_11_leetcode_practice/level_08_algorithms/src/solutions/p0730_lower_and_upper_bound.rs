//! # Problem 0730: Lower and Upper Bound
//!
//! - **Difficulty:** Easy
//! - **Topics:** `slice::partition_point`, `slice::binary_search` and its `Result`, generic `T: Ord`, `Vec::insert`
//! - **Source:** Original · [`partition_point`](https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point) · [`binary_search`](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search)
//!
//! ## Problem
//!
//! Binary search is easiest to get right when it answers one question: "where does a condition
//! that is `true` for a prefix of the slice become `false`?" The standard library provides exactly
//! that as `slice::partition_point`. Use it (no hand-written loops) to implement four generic
//! helpers on a slice `sorted` that is sorted in non-decreasing order:
//!
//! - `lower_bound(sorted, target)`: the first index `i` with `sorted[i] >= target`
//!   (`sorted.len()` if there is none). It is also the leftmost position where `target` could be
//!   inserted while keeping the slice sorted.
//! - `upper_bound(sorted, target)`: the first index `i` with `sorted[i] > target`
//!   (`sorted.len()` if there is none).
//! - `count_in_range(sorted, low, high)`: how many elements `x` satisfy `low <= x <= high`
//!   (`0` when `low > high`).
//! - `insert_sorted(v, item)`: insert `item` into the sorted vector `v` **after** every element
//!   equal to it, keep `v` sorted, and return the index where `item` now sits.
//!
//! ## Examples
//!
//! ```text
//! sorted = [1, 2, 2, 2, 5]
//! lower_bound(sorted, 2) = 1      upper_bound(sorted, 2) = 4
//! lower_bound(sorted, 3) = 4      upper_bound(sorted, 3) = 4   (3 is absent: both are equal)
//! lower_bound(sorted, 9) = 5      (past the end)
//!
//! count_in_range([1, 2, 2, 2, 5, 7, 9], 2, 7) = 5   (2, 2, 2, 5, 7)
//!
//! v = [1, 3, 3, 5]; insert_sorted(v, 3) returns 3 and v becomes [1, 3, 3, 3, 5]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= sorted.len() <= 10^5`; the input slices are sorted in non-decreasing order.
//! - Every helper must run in `O(log n)` comparisons (`insert_sorted` may also shift elements).
//!
//! ## Hints
//!
//! 1. `sorted.partition_point(|x| x < target)` counts the elements smaller than `target`.
//! 2. `count_in_range` is the difference of two bounds. Which ones?
//! 3. `slice::binary_search` returns `Ok(index)` of **some** equal element, which is ambiguous with
//!    duplicates. That is why these helpers use `partition_point` instead.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** binary search as a boundary: `partition_point(pred)` assumes `pred` is `true` for
//! a prefix of the slice and `false` afterwards, and returns the length of that prefix in
//! `O(log n)`. With `pred = x < target`, the prefix is everything smaller than `target`, so the
//! result is the lower bound. With `pred = x <= target`, it is the upper bound. The elements in
//! `[low, high]` are exactly the indices `lower_bound(low)..upper_bound(high)`, so their count is a
//! subtraction (saturating, to return `0` when `low > high`). Inserting at the upper bound keeps
//! equal elements in insertion order.
//!
//! `binary_search(&target)` is the right tool for "is it there?": it returns
//! `Ok(index_of_an_equal_element)` or `Err(insertion_point)`. With duplicates, `Ok` can point at
//! any of them, so it cannot give the first or last occurrence.
//!
//! **Complexity:** `O(log n)` per bound; `insert_sorted` is `O(log n)` comparisons plus `O(n)` to
//! shift the tail of the vector.
//!
//! **Rust concepts:**
//! - `partition_point` takes a closure `FnMut(&T) -> bool`; here `x` is `&T`, compared with
//!   `target: &T` directly because references to `Ord` values compare their targets.
//! - `usize::saturating_sub` returns `0` instead of underflowing.
//! - `Vec::insert(index, item)` shifts later elements one position to the right.
// @solution-end

pub fn lower_bound<T: Ord>(sorted: &[T], target: &T) -> usize {
    // @solution-start
    sorted.partition_point(|x| x < target)
    // @solution-end
}

pub fn upper_bound<T: Ord>(sorted: &[T], target: &T) -> usize {
    // @solution-start
    sorted.partition_point(|x| x <= target)
    // @solution-end
}

pub fn count_in_range<T: Ord>(sorted: &[T], low: &T, high: &T) -> usize {
    // @solution-start
    upper_bound(sorted, high).saturating_sub(lower_bound(sorted, low))
    // @solution-end
}

pub fn insert_sorted<T: Ord>(v: &mut Vec<T>, item: T) -> usize {
    // @solution-start
    let index = upper_bound(v, &item);
    v.insert(index, item);
    index
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: [i32; 5] = [1, 2, 2, 2, 5];

    #[test]
    fn example_bounds_of_a_duplicate() {
        assert_eq!(lower_bound(&SAMPLE, &2), 1);
        assert_eq!(upper_bound(&SAMPLE, &2), 4);
    }

    #[test]
    fn example_bounds_of_an_absent_value() {
        assert_eq!(lower_bound(&SAMPLE, &3), 4);
        assert_eq!(upper_bound(&SAMPLE, &3), 4);
        assert_eq!(lower_bound(&SAMPLE, &0), 0);
        assert_eq!(lower_bound(&SAMPLE, &9), 5);
        assert_eq!(upper_bound(&SAMPLE, &9), 5);
    }

    #[test]
    fn empty_slice() {
        let empty: [i32; 0] = [];
        assert_eq!(lower_bound(&empty, &1), 0);
        assert_eq!(upper_bound(&empty, &1), 0);
        assert_eq!(count_in_range(&empty, &0, &10), 0);
    }

    #[test]
    fn example_count_in_range() {
        let sorted = [1, 2, 2, 2, 5, 7, 9];
        assert_eq!(count_in_range(&sorted, &2, &7), 5);
        assert_eq!(count_in_range(&sorted, &3, &4), 0);
        assert_eq!(count_in_range(&sorted, &8, &1), 0); // low > high
        assert_eq!(count_in_range(&sorted, &0, &100), 7);
    }

    #[test]
    fn example_insert_after_equal_elements() {
        let mut v = vec![1, 3, 3, 5];
        assert_eq!(insert_sorted(&mut v, 3), 3);
        assert_eq!(v, vec![1, 3, 3, 3, 5]);
        assert_eq!(insert_sorted(&mut v, 0), 0);
        assert_eq!(insert_sorted(&mut v, 9), 6);
        assert_eq!(v, vec![0, 1, 3, 3, 3, 5, 9]);
    }

    #[test]
    fn works_with_strings() {
        let fruits = ["apple", "banana", "cherry"].map(String::from);
        assert_eq!(lower_bound(&fruits, &"blueberry".to_string()), 2);
        assert_eq!(upper_bound(&fruits, &"banana".to_string()), 2);
    }

    #[test]
    fn large_slice_counts() {
        // Each value v in 0..1000 appears 100 times.
        let sorted: Vec<u32> = (0..100_000).map(|i| i / 100).collect();
        assert_eq!(lower_bound(&sorted, &500), 50_000);
        assert_eq!(upper_bound(&sorted, &500), 50_100);
        assert_eq!(count_in_range(&sorted, &10, &19), 1_000);
    }
}
