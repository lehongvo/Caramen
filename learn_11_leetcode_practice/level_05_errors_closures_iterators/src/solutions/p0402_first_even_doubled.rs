//! # Problem 0402: First Even Number, Doubled
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Iterator::find`, `Option::map`, `Option::filter`, `Option::map_or`
//! - **Source:** Original · [std `Option`](https://doc.rust-lang.org/std/option/enum.Option.html) · [Rust By Example: map](https://doc.rust-lang.org/rust-by-example/error/option_unwrap/map.html)
//!
//! ## Problem
//!
//! Searching a list may find nothing, so a search naturally returns an `Option`. Instead of
//! unpacking that `Option` with `match` every time, you can transform it with *combinators*:
//! methods that run only when there is a value.
//!
//! Implement:
//!
//! - `first_even(nums: &[i32]) -> Option<i32>`: the first even number in `nums` (scanning from
//!   the left), or `None` if there is none. Negative numbers can be even too.
//! - `first_even_doubled(nums: &[i32]) -> Option<i32>`: that first even number multiplied by 2.
//! - `first_even_if_at_least(nums: &[i32], min: i32) -> Option<i32>`: the first even number, but
//!   only if it is `>= min`. **Only the first even number is considered**: if it is too small,
//!   the answer is `None` even when a later even number would be large enough.
//! - `describe_first_even(nums: &[i32]) -> String`: `"first even: N"`, or `"no even number"`.
//!
//! ## Examples
//!
//! ```text
//! first_even(&[3, 7, 8, 10])                  -> Some(8)
//! first_even(&[1, 3, 5])                      -> None
//! first_even_doubled(&[5, -4, 6])             -> Some(-8)
//! first_even_if_at_least(&[1, 4, 100], 10)    -> None      (4 is the first even and 4 < 10)
//! first_even_if_at_least(&[1, 40, 2], 10)     -> Some(40)
//! describe_first_even(&[9, 2])                -> "first even: 2"
//! describe_first_even(&[])                    -> "no even number"
//! ```
//!
//! ## Constraints
//!
//! - `0 <= nums.len() <= 10^4`
//! - `-10^6 <= nums[i], min <= 10^6`
//!
//! ## Hints
//!
//! 1. `nums.iter().copied().find(|n| ...)` returns the first matching element as `Option<i32>`.
//! 2. `option.map(|n| ...)` changes the value inside `Some` and leaves `None` alone.
//! 3. `option.filter(|n| ...)` turns `Some(n)` into `None` when the condition is false.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** write the search once in `first_even`, then build the other three answers by
//! transforming its `Option` with one combinator each: `map` for doubling, `filter` for the
//! size check and `map_or` for turning both cases into a `String`.
//!
//! **Rust concepts:**
//! - `iter()` yields `&i32`; `copied()` turns them into `i32` so the closures work with plain numbers.
//! - `find` stops at the first match: the rest of the slice is never looked at.
//! - `Option::filter` keeps the value only when the predicate returns `true`. It does not search
//!   further; that is why `first_even_if_at_least(&[1, 4, 100], 10)` is `None`.
//! - `map_or(default, f)` is `map(f).unwrap_or(default)` in one call.
// @solution-end

pub fn first_even(nums: &[i32]) -> Option<i32> {
    // @solution-start
    nums.iter().copied().find(|n| n % 2 == 0)
    // @solution-end
}

pub fn first_even_doubled(nums: &[i32]) -> Option<i32> {
    // @solution-start
    first_even(nums).map(|n| n * 2)
    // @solution-end
}

pub fn first_even_if_at_least(nums: &[i32], min: i32) -> Option<i32> {
    // @solution-start
    first_even(nums).filter(|&n| n >= min)
    // @solution-end
}

pub fn describe_first_even(nums: &[i32]) -> String {
    // @solution-start
    first_even(nums).map_or(String::from("no even number"), |n| {
        format!("first even: {n}")
    })
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_even_finds_the_leftmost_even() {
        assert_eq!(first_even(&[3, 7, 8, 10]), Some(8));
        assert_eq!(first_even(&[-3, -2, 4]), Some(-2));
        assert_eq!(first_even(&[0]), Some(0));
    }

    #[test]
    fn first_even_none_cases() {
        assert_eq!(first_even(&[1, 3, 5]), None);
        assert_eq!(first_even(&[]), None);
    }

    #[test]
    fn doubled_maps_only_some() {
        assert_eq!(first_even_doubled(&[5, -4, 6]), Some(-8));
        assert_eq!(first_even_doubled(&[1_000_000]), Some(2_000_000));
        assert_eq!(first_even_doubled(&[1, 3]), None);
    }

    #[test]
    fn filter_checks_only_the_first_even() {
        assert_eq!(first_even_if_at_least(&[1, 4, 100], 10), None);
        assert_eq!(first_even_if_at_least(&[1, 40, 2], 10), Some(40));
        assert_eq!(first_even_if_at_least(&[10], 10), Some(10));
        assert_eq!(first_even_if_at_least(&[7, 9], -100), None);
    }

    #[test]
    fn describe_both_variants() {
        assert_eq!(describe_first_even(&[9, 2]), "first even: 2");
        assert_eq!(describe_first_even(&[]), "no even number");
        assert_eq!(describe_first_even(&[1, -6]), "first even: -6");
    }
}
