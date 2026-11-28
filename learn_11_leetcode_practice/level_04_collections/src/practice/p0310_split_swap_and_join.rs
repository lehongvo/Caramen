// ==================================================================
// PRACTICE 0310: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0310_split_swap_and_join.md
// Run the tests:          cargo test practice::p0310
// ==================================================================

//! # Problem 0310: Split, Swap and Join
//!
//! - **Difficulty:** Easy
//! - **Topics:** `slice::split_at`, `concat`, `join`, slices of `Vec`s
//! - **Source:** Original · [`slice::concat`](https://doc.rust-lang.org/std/primitive.slice.html#method.concat) · [`slice::join`](https://doc.rust-lang.org/std/primitive.slice.html#method.join)
//!
//! ## Problem
//!
//! Implement three helpers that cut and glue vectors:
//!
//! 1. `swap_halves(values: &[i32], at: usize) -> Vec<i32>` splits `values` into a left part
//!    (`at` elements) and a right part (the rest) and returns `right` followed by `left`. If `at`
//!    is larger than the length, treat it as the length (the result is then a plain copy).
//! 2. `flatten_rows(rows: &[Vec<i32>]) -> Vec<i32>` returns all rows glued together in order.
//! 3. `format_groups(groups: &[Vec<&str>]) -> String` joins the words of each group with a single
//!    space, and joins the groups with `" | "`.
//!
//! ## Examples
//!
//! ```text
//! swap_halves(&[1, 2, 3, 4, 5], 2)     => [3, 4, 5, 1, 2]
//! swap_halves(&[1, 2, 3], 10)          => [1, 2, 3]
//! flatten_rows(&[vec![1, 2], vec![], vec![3]]) => [1, 2, 3]
//! format_groups(&[vec!["red", "green"], vec!["blue"]]) => "red green | blue"
//! format_groups(&[])                   => ""
//! ```
//!
//! ## Constraints
//!
//! - `0 <= values.len() <= 10^4`, `at` can be any `usize`.
//! - Total number of elements in `rows` or `groups` is at most `10^4`.
//!
//! ## Hints
//!
//! 1. `split_at(mid)` returns `(&values[..mid], &values[mid..])` and **panics** if
//!    `mid > len`. Clamp `at` first.
//! 2. A slice of slices (or of `Vec`s) has `concat()` to glue them, and `join(separator)` to glue
//!    them with something in between.

/// Returns `values[at..]` followed by `values[..at]` (with `at` clamped to the length).
pub fn swap_halves(values: &[i32], at: usize) -> Vec<i32> {
    todo!("Write your solution here")
}

/// Glues all rows together in order.
pub fn flatten_rows(rows: &[Vec<i32>]) -> Vec<i32> {
    todo!("Write your solution here")
}

/// Joins the words of each group with spaces and the groups with `" | "`.
pub fn format_groups(groups: &[Vec<&str>]) -> String {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_halves_examples() {
        assert_eq!(swap_halves(&[1, 2, 3, 4, 5], 2), vec![3, 4, 5, 1, 2]);
        assert_eq!(swap_halves(&[1, 2, 3], 10), vec![1, 2, 3]);
        assert_eq!(swap_halves(&[1, 2, 3], 0), vec![1, 2, 3]);
        assert_eq!(swap_halves(&[1, 2, 3], 3), vec![1, 2, 3]);
        assert_eq!(swap_halves(&[], 1), Vec::<i32>::new());
    }

    #[test]
    fn flatten_rows_example() {
        assert_eq!(flatten_rows(&[vec![1, 2], vec![], vec![3]]), vec![1, 2, 3]);
        assert_eq!(flatten_rows(&[]), Vec::<i32>::new());
    }

    #[test]
    fn format_groups_examples() {
        assert_eq!(
            format_groups(&[vec!["red", "green"], vec!["blue"]]),
            "red green | blue"
        );
        assert_eq!(format_groups(&[]), "");
        assert_eq!(format_groups(&[vec!["solo"]]), "solo");
        assert_eq!(format_groups(&[vec![], vec!["x"]]), " | x");
    }

    #[test]
    fn large_swap_is_a_rotation() {
        let values: Vec<i32> = (0..10_000).collect();
        let mut expected = values.clone();
        expected.rotate_left(1234);
        assert_eq!(swap_halves(&values, 1234), expected);
    }
}
