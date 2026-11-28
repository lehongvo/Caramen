// ==================================================================
// PRACTICE 0416: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0416_collect_results_into_vec.md
// Run the tests:          cargo test practice::p0416
// ==================================================================

//! # Problem 0416: All Numbers or the First Error
//!
//! - **Difficulty:** Easy
//! - **Topics:** `collect::<Result<Vec<_>, _>>()`, `Sum` for `Result`, `Iterator::position`, `enumerate`, `map_err`
//! - **Source:** Original · [Rust By Example: Iterating over Results](https://doc.rust-lang.org/rust-by-example/error/iter_result.html)
//!
//! ## Problem
//!
//! Mapping a parser over a list gives an iterator of `Result`s. Very often you want **all** the
//! values, or else the first error. `collect` can do exactly that: collecting an iterator of
//! `Result<T, E>` into a `Result<Vec<T>, E>` stops at the first `Err` and returns it. In the same
//! way, `sum()` can add up an iterator of `Result<i32, E>` into a `Result<i32, E>`.
//!
//! Implement:
//!
//! - `parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError>`: every item parsed as an `i32`
//!   (no trimming), or the error of the first invalid item.
//! - `sum_all(items: &[&str]) -> Result<i32, ParseIntError>`: the sum of all items, or the first
//!   error. The sum of an empty list is `0`.
//! - `first_bad_index(items: &[&str]) -> Option<usize>`: the 0-based index of the first item that is
//!   not a valid `i32`, or `None` if all are valid.
//! - `parse_all_explained(items: &[&str]) -> Result<Vec<i32>, String>`: like `parse_all`, but the
//!   error is ``"item N (`TEXT`) is not a number"`` where `N` is the **1-based** position.
//!
//! ## Examples
//!
//! ```text
//! parse_all(&["1", "-2", "30"])            -> Ok([1, -2, 30])
//! parse_all(&["1", "x", ""])               -> Err(InvalidDigit)   (the "x"; "" is never parsed)
//! sum_all(&["4", "5"])                     -> Ok(9)
//! sum_all(&[])                             -> Ok(0)
//! first_bad_index(&["1", "2", "three"])    -> Some(2)
//! parse_all_explained(&["7", "seven"])     -> Err("item 2 (`seven`) is not a number")
//! ```
//!
//! ## Constraints
//!
//! - `0 <= items.len() <= 10^4`; valid numbers satisfy `|value| <= 10^4`, so sums fit in an `i32`.
//!
//! ## Hints
//!
//! 1. `items.iter().map(|s| s.parse::<i32>()).collect()` already returns the right type when the
//!    function's return type is `Result<Vec<i32>, ParseIntError>`.
//! 2. Replace `collect()` with `sum()` for `sum_all`.
//! 3. For the explained version, `enumerate()` gives you the index to put in the message.

use std::num::ParseIntError;

pub fn parse_all(items: &[&str]) -> Result<Vec<i32>, ParseIntError> {
    todo!("Write your solution here")
}

pub fn sum_all(items: &[&str]) -> Result<i32, ParseIntError> {
    todo!("Write your solution here")
}

pub fn first_bad_index(items: &[&str]) -> Option<usize> {
    todo!("Write your solution here")
}

pub fn parse_all_explained(items: &[&str]) -> Result<Vec<i32>, String> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn parse_all_success_and_failure() {
        assert_eq!(parse_all(&["1", "-2", "30"]), Ok(vec![1, -2, 30]));
        assert_eq!(parse_all(&[]), Ok(vec![]));
        let err = parse_all(&["1", "x", ""]).unwrap_err();
        assert_eq!(err.kind(), &IntErrorKind::InvalidDigit);
    }

    #[test]
    fn sum_all_cases() {
        assert_eq!(sum_all(&["4", "5"]), Ok(9));
        assert_eq!(sum_all(&[]), Ok(0));
        assert_eq!(
            sum_all(&["4", "", "x"]).unwrap_err().kind(),
            &IntErrorKind::Empty
        );
    }

    #[test]
    fn first_bad_index_cases() {
        assert_eq!(first_bad_index(&["1", "2", "three"]), Some(2));
        assert_eq!(first_bad_index(&["1", "2"]), None);
        assert_eq!(first_bad_index(&[" 1"]), Some(0));
    }

    #[test]
    fn explained_errors_use_one_based_positions() {
        assert_eq!(
            parse_all_explained(&["7", "seven"]),
            Err("item 2 (`seven`) is not a number".to_string())
        );
        assert_eq!(
            parse_all_explained(&["", "1"]),
            Err("item 1 (``) is not a number".to_string())
        );
        assert_eq!(parse_all_explained(&["7", "8"]), Ok(vec![7, 8]));
    }

    #[test]
    fn large_valid_input() {
        let owned: Vec<String> = (1..=10_000).map(|i| (i % 7).to_string()).collect();
        let items: Vec<&str> = owned.iter().map(String::as_str).collect();
        let expected: i32 = (1..=10_000).map(|i| i % 7).sum();
        assert_eq!(sum_all(&items), Ok(expected));
        assert_eq!(parse_all(&items).map(|v| v.len()), Ok(10_000));
    }
}
