// ==================================================================
// PRACTICE 0470: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0470_flatten_nested_data.md
// Run the tests:          cargo test practice::p0470
// ==================================================================

//! # Problem 0470: Flatten Nested Data
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Iterator::flatten`, `flat_map`, `Option` and `Result` as iterators, `split_whitespace`, `filter_map`
//! - **Source:** Original · [std `Iterator::flatten`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten) · [std `Option`: iterating over `Option`](https://doc.rust-lang.org/std/option/index.html#iterating-over-option)
//!
//! ## Problem
//!
//! `flatten` removes one level of nesting from an iterator whose items are themselves iterable.
//! That includes vectors of vectors, but also `Option` (an iterator of zero or one item) and
//! `Result` (iterating over a `Result` yields the `Ok` value, if any). `flat_map(f)` is
//! `map(f).flatten()`.
//!
//! Implement:
//!
//! - `flatten_grid(grid: Vec<Vec<i32>>) -> Vec<i32>`: all values, row by row.
//! - `present_values(values: &[Option<i32>]) -> Vec<i32>`: the `Some` values, in order.
//! - `sum_present(values: &[Option<i32>]) -> i32`: the sum of the `Some` values.
//! - `ok_values(results: Vec<Result<i32, String>>) -> Vec<i32>`: the `Ok` values, in order.
//! - `all_words(lines: &[&str]) -> Vec<String>`: every whitespace-separated word of every line.
//! - `all_numbers(lines: &[&str]) -> Vec<i32>`: every comma-separated field (trimmed) of every line
//!   that parses as an `i32`; other fields are skipped.
//!
//! ## Examples
//!
//! ```text
//! flatten_grid(vec![vec![1, 2], vec![], vec![3]])           -> [1, 2, 3]
//! present_values(&[Some(4), None, Some(-1)])                -> [4, -1]
//! sum_present(&[Some(4), None, Some(-1)])                   -> 3
//! ok_values(vec![Ok(1), Err("bad".into()), Ok(3)])          -> [1, 3]
//! all_words(&["the quick", "", "  brown fox "])             -> ["the", "quick", "brown", "fox"]
//! all_numbers(&["1, 2, x", "40,  ,-5"])                     -> [1, 2, 40, -5]
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` items in total.
//!
//! ## Hints
//!
//! 1. `grid.into_iter().flatten()` moves the inner vectors' values out.
//! 2. `values.iter().flatten()` yields `&i32` for every `Some`; add `.copied()`.
//! 3. `lines.iter().flat_map(|l| l.split(','))` gives all fields of all lines.

pub fn flatten_grid(grid: Vec<Vec<i32>>) -> Vec<i32> {
    todo!("Write your solution here")
}

pub fn present_values(values: &[Option<i32>]) -> Vec<i32> {
    todo!("Write your solution here")
}

pub fn sum_present(values: &[Option<i32>]) -> i32 {
    todo!("Write your solution here")
}

pub fn ok_values(results: Vec<Result<i32, String>>) -> Vec<i32> {
    todo!("Write your solution here")
}

pub fn all_words(lines: &[&str]) -> Vec<String> {
    todo!("Write your solution here")
}

pub fn all_numbers(lines: &[&str]) -> Vec<i32> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grids() {
        assert_eq!(
            flatten_grid(vec![vec![1, 2], vec![], vec![3]]),
            vec![1, 2, 3]
        );
        assert_eq!(flatten_grid(vec![]), Vec::<i32>::new());
        assert_eq!(flatten_grid(vec![vec![], vec![]]), Vec::<i32>::new());
    }

    #[test]
    fn options() {
        assert_eq!(present_values(&[Some(4), None, Some(-1)]), vec![4, -1]);
        assert_eq!(sum_present(&[Some(4), None, Some(-1)]), 3);
        assert_eq!(sum_present(&[None, None]), 0);
    }

    #[test]
    fn results() {
        assert_eq!(ok_values(vec![Ok(1), Err("bad".into()), Ok(3)]), vec![1, 3]);
        assert_eq!(ok_values(vec![Err("x".into())]), Vec::<i32>::new());
    }

    #[test]
    fn words_and_numbers() {
        assert_eq!(
            all_words(&["the quick", "", "  brown fox "]),
            vec!["the", "quick", "brown", "fox"]
        );
        assert_eq!(all_numbers(&["1, 2, x", "40,  ,-5"]), vec![1, 2, 40, -5]);
        assert_eq!(all_numbers(&[]), Vec::<i32>::new());
    }

    #[test]
    fn large_grid() {
        let grid: Vec<Vec<i32>> = (0..100)
            .map(|r| (0..100).map(|c| r * 100 + c).collect())
            .collect();
        let flat = flatten_grid(grid);
        assert_eq!(flat.len(), 10_000);
        assert!(flat.iter().enumerate().all(|(i, &v)| v == i as i32));
    }
}
