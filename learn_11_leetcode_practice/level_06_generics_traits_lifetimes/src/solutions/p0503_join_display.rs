//! # Problem 0503: Join Anything Displayable
//!
//! - **Difficulty:** Easy
//! - **Topics:** `where` clauses, `T: Display`, `std::fmt::Write`, generic functions over slices
//! - **Source:** Original · [The Rust Book 10.2](https://doc.rust-lang.org/book/ch10-02-traits.html#clearer-trait-bounds-with-where-clauses)
//!
//! ## Problem
//!
//! `[&str]::join` only works for string-like elements. Write generic versions that work for **any**
//! type that implements `std::fmt::Display`:
//!
//! - `join_display<T>(items: &[T], sep: &str) -> String where T: Display` formats every element with
//!   `{}` and puts `sep` between consecutive elements (not before the first or after the last).
//! - `join_with_brackets<T, U>(left: &[T], right: &[U]) -> String where T: Display, U: Display`
//!   returns `"[l1, l2, ...] | [r1, r2, ...]"`: each slice is joined with `", "` and wrapped in square
//!   brackets. The two slices may have **different** element types.
//!
//! Use `where` clauses for the bounds (that is the point of the exercise).
//!
//! ## Examples
//!
//! ```text
//! join_display(&[1, 2, 3], "-")             => "1-2-3"
//! join_display(&['a'], ", ")                => "a"
//! join_display::<i32>(&[], ", ")            => ""
//! join_with_brackets(&[1.5, 2.0], &["x"])   => "[1.5, 2] | [x]"
//! ```
//!
//! ## Constraints
//!
//! - `0 <= items.len() <= 10^4`
//!
//! ## Hints
//!
//! 1. A `where` clause goes between the return type and the opening `{` of the body.
//! 2. `write!(out, "{item}")` appends to a `String` if you `use std::fmt::Write;`.
//! 3. Reuse `join_display` inside `join_with_brackets`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** iterate with `enumerate`, push the separator before every element except the first,
//! then format the element into the output string. `join_with_brackets` calls `join_display` twice,
//! once for each type parameter.
//!
//! **Rust concepts:**
//! - `where T: Display` is equivalent to `<T: Display>`, but keeps the signature readable when there
//!   are several type parameters with several bounds.
//! - `T` and `U` are independent type parameters, so `left` and `right` can hold different types.
//! - `use std::fmt::Write` brings the `write!` support for `String` into scope; writing to a `String`
//!   cannot fail, so ignoring the `fmt::Result` with `let _ =` is fine. `1.5_f64` displays as `1.5`
//!   and `2.0_f64` as `2`.
// @solution-end

use std::fmt::{Display, Write};

/// Formats every element with `{}` and joins them with `sep`.
pub fn join_display<T>(items: &[T], sep: &str) -> String
where
    T: Display,
{
    // @solution-start
    let mut out = String::new();
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push_str(sep);
        }
        let _ = write!(out, "{item}");
    }
    out
    // @solution-end
}

/// Returns `"[left joined by ', '] | [right joined by ', ']"`.
pub fn join_with_brackets<T, U>(left: &[T], right: &[U]) -> String
where
    T: Display,
    U: Display,
{
    // @solution-start
    format!(
        "[{}] | [{}]",
        join_display(left, ", "),
        join_display(right, ", ")
    )
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joins_numbers_and_chars() {
        assert_eq!(join_display(&[1, 2, 3], "-"), "1-2-3");
        assert_eq!(join_display(&['a'], ", "), "a");
        assert_eq!(join_display(&["x", "y"], ""), "xy");
    }

    #[test]
    fn empty_slice_gives_empty_string() {
        assert_eq!(join_display::<i32>(&[], ", "), "");
        assert_eq!(join_with_brackets::<i32, char>(&[], &[]), "[] | []");
    }

    #[test]
    fn brackets_with_different_types() {
        assert_eq!(join_with_brackets(&[1.5, 2.0], &["x"]), "[1.5, 2] | [x]");
        assert_eq!(
            join_with_brackets(&[true, false], &[10_u8, 20]),
            "[true, false] | [10, 20]"
        );
    }

    #[test]
    fn long_separator_and_many_items() {
        let items: Vec<u32> = (1..=1000).collect();
        let joined = join_display(&items, " <> ");
        assert!(joined.starts_with("1 <> 2 <> 3"));
        assert!(joined.ends_with("999 <> 1000"));
        assert_eq!(joined.matches(" <> ").count(), 999);
    }
}
