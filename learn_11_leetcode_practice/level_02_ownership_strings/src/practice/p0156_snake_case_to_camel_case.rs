// ==================================================================
// PRACTICE 0156: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0156_snake_case_to_camel_case.md
// Run the tests:          cargo test practice::p0156
// ==================================================================

//! # Problem 0156: snake_case to camelCase
//!
//! - **Difficulty:** Easy
//! - **Topics:** `split('_')`, skipping empty pieces, `to_ascii_uppercase` / `to_ascii_lowercase`, `push`
//! - **Source:** Original · [`str::split`](https://doc.rust-lang.org/std/primitive.str.html#method.split) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
//!
//! ## Problem
//!
//! Convert an identifier written in `snake_case` to `camelCase`. Implement
//! `snake_to_camel(name: &str) -> String`:
//!
//! 1. Split `name` at every underscore `'_'` and ignore empty pieces (so leading, trailing and
//!    repeated underscores disappear).
//! 2. The first piece is written entirely in lowercase.
//! 3. Every following piece is written with an uppercase first letter and the rest in lowercase.
//! 4. Concatenate the pieces without separators.
//!
//! The input contains only ASCII letters, digits and underscores. Digits are kept as they are.
//!
//! ## Examples
//!
//! ```text
//! Input:  name = "hello_world"
//! Output: "helloWorld"
//!
//! Input:  name = "HTTP_status_CODE"
//! Output: "httpStatusCode"
//!
//! Input:  name = "__private__value_"
//! Output: "privateValue"
//!
//! Input:  name = "max_2_items"
//! Output: "max2Items"
//! ```
//!
//! ## Constraints
//!
//! - `0 <= name.len() <= 10^4`
//! - `name` contains only ASCII letters, digits and `'_'`.
//!
//! ## Hints
//!
//! 1. `name.split('_').filter(|piece| !piece.is_empty())` yields only the real pieces.
//! 2. Combine it with `enumerate()` to know whether you are on the first piece.
//! 3. Push characters one by one with `c.to_ascii_uppercase()` or `c.to_ascii_lowercase()`.

pub fn snake_to_camel(name: &str) -> String {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_words() {
        assert_eq!(snake_to_camel("hello_world"), "helloWorld");
    }

    #[test]
    fn example_mixed_case_input() {
        assert_eq!(snake_to_camel("HTTP_status_CODE"), "httpStatusCode");
    }

    #[test]
    fn example_extra_underscores() {
        assert_eq!(snake_to_camel("__private__value_"), "privateValue");
    }

    #[test]
    fn example_digits() {
        assert_eq!(snake_to_camel("max_2_items"), "max2Items");
    }

    #[test]
    fn empty_and_only_underscores() {
        assert_eq!(snake_to_camel(""), "");
        assert_eq!(snake_to_camel("___"), "");
    }

    #[test]
    fn single_piece() {
        assert_eq!(snake_to_camel("Value"), "value");
    }

    #[test]
    fn many_pieces() {
        let name = vec!["ab"; 3_000].join("_");
        let expected = format!("ab{}", "Ab".repeat(2_999));
        assert_eq!(snake_to_camel(&name), expected);
    }
}
