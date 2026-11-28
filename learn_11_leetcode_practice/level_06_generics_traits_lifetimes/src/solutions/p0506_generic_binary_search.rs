//! # Problem 0506: Generic Lower and Upper Bound
//!
//! - **Difficulty:** Medium
//! - **Topics:** generic binary search, `T: Ord`, `Result<usize, usize>`, half-open ranges
//! - **Source:** Original · [slice::binary_search](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search)
//!
//! ## Problem
//!
//! Write a small binary-search toolkit that works for any totally ordered type `T: Ord`. In every
//! function `items` is sorted in non-decreasing order and may contain duplicates. Do **not** use
//! `binary_search`, `partition_point` or any other search method from the standard library.
//!
//! - `lower_bound(items, target) -> usize`: the first index `i` with `items[i] >= target`
//!   (`items.len()` if there is none).
//! - `upper_bound(items, target) -> usize`: the first index `i` with `items[i] > target`
//!   (`items.len()` if there is none).
//! - `search_sorted(items, target) -> Result<usize, usize>`: `Ok(i)` where `i` is the index of the
//!   **first** occurrence of `target`, or `Err(i)` where `i` is the position at which `target` could
//!   be inserted to keep the slice sorted.
//! - `count_in_range(items, lo, hi) -> usize`: how many elements `x` satisfy `lo <= x <= hi`
//!   (`0` when `lo > hi`).
//!
//! Every function takes `items: &[T]` and the values by reference (`target: &T`, `lo: &T`, `hi: &T`).
//!
//! ## Examples
//!
//! ```text
//! items = [1, 3, 3, 3, 7, 9]
//! lower_bound(&items, &3)          => 1
//! upper_bound(&items, &3)          => 4
//! search_sorted(&items, &3)        => Ok(1)
//! search_sorted(&items, &8)        => Err(5)
//! count_in_range(&items, &2, &7)   => 4      (3, 3, 3, 7)
//! search_sorted(&["ant", "bee"], &"cat") => Err(2)
//! ```
//!
//! ## Constraints
//!
//! - `0 <= items.len() <= 10^6`
//! - Each query must run in `O(log n)` comparisons.
//!
//! ## Hints
//!
//! 1. Keep a half-open window `[lo, hi)` that is known to contain the answer, starting with
//!    `[0, len)`. Compute `mid = lo + (hi - lo) / 2`.
//! 2. `lower_bound` and `upper_bound` differ only in the comparison (`<` vs `<=`).
//! 3. The other two functions are one-liners on top of the bounds.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `lower_bound` keeps the invariant "everything before `lo` is `< target` and everything at
//! or after `hi` is `>= target`". Each step halves the window. `upper_bound` is the same with `<=`.
//! `search_sorted` checks the element at `lower_bound`, and `count_in_range` is
//! `upper_bound(hi) - lower_bound(lo)`.
//!
//! **Complexity:** `O(log n)` comparisons per query, `O(1)` extra space.
//!
//! **Rust concepts:**
//! - The functions only need `T: Ord` because they only compare; they never copy or clone elements.
//! - Taking `target: &T` means callers can search for a `String` without giving up ownership.
//! - `Result<usize, usize>` mirrors `slice::binary_search`: `Err` is not a failure here, it carries
//!   the insertion point.
//! - `lo + (hi - lo) / 2` avoids overflow, and `usize` indices never go negative with this
//!   half-open formulation.
// @solution-end

/// First index `i` with `items[i] >= target`.
pub fn lower_bound<T: Ord>(items: &[T], target: &T) -> usize {
    // @solution-start
    let (mut lo, mut hi) = (0, items.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if items[mid] < *target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
    // @solution-end
}

/// First index `i` with `items[i] > target`.
pub fn upper_bound<T: Ord>(items: &[T], target: &T) -> usize {
    // @solution-start
    let (mut lo, mut hi) = (0, items.len());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if items[mid] <= *target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
    // @solution-end
}

/// `Ok(first index of target)` or `Err(insertion point)`.
pub fn search_sorted<T: Ord>(items: &[T], target: &T) -> Result<usize, usize> {
    // @solution-start
    let i = lower_bound(items, target);
    match items.get(i) {
        Some(x) if x == target => Ok(i),
        _ => Err(i),
    }
    // @solution-end
}

/// Number of elements `x` with `lo <= x <= hi`.
pub fn count_in_range<T: Ord>(items: &[T], lo: &T, hi: &T) -> usize {
    // @solution-start
    if lo > hi {
        return 0;
    }
    upper_bound(items, hi) - lower_bound(items, lo)
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_bounds() {
        let items = [1, 3, 3, 3, 7, 9];
        assert_eq!(lower_bound(&items, &3), 1);
        assert_eq!(upper_bound(&items, &3), 4);
        assert_eq!(lower_bound(&items, &0), 0);
        assert_eq!(upper_bound(&items, &9), 6);
    }

    #[test]
    fn example_search_and_count() {
        let items = [1, 3, 3, 3, 7, 9];
        assert_eq!(search_sorted(&items, &3), Ok(1));
        assert_eq!(search_sorted(&items, &8), Err(5));
        assert_eq!(search_sorted(&items, &0), Err(0));
        assert_eq!(count_in_range(&items, &2, &7), 4);
        assert_eq!(count_in_range(&items, &7, &2), 0);
    }

    #[test]
    fn works_for_strings() {
        assert_eq!(search_sorted(&["ant", "bee"], &"cat"), Err(2));
        let owned = vec!["a".to_string(), "b".to_string(), "b".to_string()];
        assert_eq!(search_sorted(&owned, &"b".to_string()), Ok(1));
        assert_eq!(
            count_in_range(&owned, &"a".to_string(), &"az".to_string()),
            1
        );
    }

    #[test]
    fn empty_slice() {
        let empty: [i32; 0] = [];
        assert_eq!(lower_bound(&empty, &5), 0);
        assert_eq!(upper_bound(&empty, &5), 0);
        assert_eq!(search_sorted(&empty, &5), Err(0));
        assert_eq!(count_in_range(&empty, &0, &10), 0);
    }

    #[test]
    fn large_input_matches_std() {
        let items: Vec<u64> = (0..1_000_000_u64).map(|x| x / 4 * 2).collect();
        for target in [0_u64, 1, 2, 1_000, 499_998, 499_999, 600_000] {
            assert_eq!(
                lower_bound(&items, &target),
                items.partition_point(|x| *x < target)
            );
            assert_eq!(
                upper_bound(&items, &target),
                items.partition_point(|x| *x <= target)
            );
        }
        assert_eq!(count_in_range(&items, &10, &20), 24);
    }
}
