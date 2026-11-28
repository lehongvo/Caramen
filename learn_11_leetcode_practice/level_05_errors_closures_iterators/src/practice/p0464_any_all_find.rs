// ==================================================================
// PRACTICE 0464: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0464_any_all_find.md
// Run the tests:          cargo test practice::p0464
// ==================================================================

//! # Problem 0464: Questions About a List
//!
//! - **Difficulty:** Easy
//! - **Topics:** `any`, `all`, `find`, `find_map`, `count`, `is_sorted`, short-circuiting, vacuous truth
//! - **Source:** Original · [std `Iterator::any`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any) · [std `Iterator::find_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.find_map)
//!
//! ## Problem
//!
//! Many questions about a collection have a ready-made iterator method. They all stop as early as
//! possible: `any` stops at the first `true`, `all` at the first `false`, `find` at the first match.
//!
//! Implement:
//!
//! - `has_negative(nums: &[i32]) -> bool`.
//! - `all_even(nums: &[i32]) -> bool` (`true` for an empty slice: there is no odd number).
//! - `first_over(nums: &[i32], limit: i32) -> Option<i32>`: the first number strictly greater
//!   than `limit`.
//! - `first_number(words: &[&str]) -> Option<i32>`: the first word that parses as an `i32`,
//!   already parsed.
//! - `count_in_range(nums: &[i32], lo: i32, hi: i32) -> usize`: how many numbers are in `lo..=hi`.
//! - `is_non_decreasing(nums: &[i32]) -> bool`: each number is `>=` the previous one.
//!
//! ## Examples
//!
//! ```text
//! has_negative(&[3, -1, 4])            -> true
//! all_even(&[2, 4, 7])                 -> false
//! all_even(&[])                        -> true
//! first_over(&[1, 5, 9, 12], 6)        -> Some(9)
//! first_number(&["x", "12", "7"])      -> Some(12)
//! count_in_range(&[1, 5, 9, 12], 5, 9) -> 2
//! is_non_decreasing(&[1, 1, 2, 5])     -> true
//! ```
//!
//! ## Constraints
//!
//! - Slices have at most `10^5` elements.
//!
//! ## Hints
//!
//! 1. `find(|&&x| x > limit)` receives a `&&i32` because `iter()` yields `&i32` and `find` passes
//!    a reference to each item; `copied()` first makes it simpler.
//! 2. `find_map(f)` returns the first `Some` produced by `f`: perfect with `parse().ok()`.
//! 3. `(lo..=hi).contains(&x)` tests a range; `slice::is_sorted` (Rust 1.82) checks the order.

pub fn has_negative(nums: &[i32]) -> bool {
    todo!("Write your solution here")
}

pub fn all_even(nums: &[i32]) -> bool {
    todo!("Write your solution here")
}

pub fn first_over(nums: &[i32], limit: i32) -> Option<i32> {
    todo!("Write your solution here")
}

pub fn first_number(words: &[&str]) -> Option<i32> {
    todo!("Write your solution here")
}

pub fn count_in_range(nums: &[i32], lo: i32, hi: i32) -> usize {
    todo!("Write your solution here")
}

pub fn is_non_decreasing(nums: &[i32]) -> bool {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_and_all() {
        assert!(has_negative(&[3, -1, 4]));
        assert!(!has_negative(&[3, 0, 4]));
        assert!(!has_negative(&[]));
        assert!(!all_even(&[2, 4, 7]));
        assert!(all_even(&[2, -4, 0]));
        assert!(all_even(&[]));
    }

    #[test]
    fn find_first() {
        assert_eq!(first_over(&[1, 5, 9, 12], 6), Some(9));
        assert_eq!(first_over(&[1, 5], 5), None);
        assert_eq!(first_over(&[], 0), None);
    }

    #[test]
    fn find_map_parses_once() {
        assert_eq!(first_number(&["x", "12", "7"]), Some(12));
        assert_eq!(first_number(&["x", "-3"]), Some(-3));
        assert_eq!(first_number(&["x", "y"]), None);
        assert_eq!(first_number(&["99999999999", "4"]), Some(4));
    }

    #[test]
    fn ranges_and_order() {
        assert_eq!(count_in_range(&[1, 5, 9, 12], 5, 9), 2);
        assert_eq!(count_in_range(&[1, 5, 9, 12], 10, 3), 0);
        assert!(is_non_decreasing(&[1, 1, 2, 5]));
        assert!(!is_non_decreasing(&[1, 3, 2]));
        assert!(is_non_decreasing(&[]));
    }

    #[test]
    fn large_inputs() {
        let nums: Vec<i32> = (0..100_000).collect();
        assert!(is_non_decreasing(&nums));
        assert!(!has_negative(&nums));
        assert_eq!(count_in_range(&nums, 10, 19), 10);
        assert_eq!(first_over(&nums, 99_998), Some(99_999));
    }
}
