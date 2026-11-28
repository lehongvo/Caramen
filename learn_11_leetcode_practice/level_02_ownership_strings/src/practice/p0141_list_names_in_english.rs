// ==================================================================
// PRACTICE 0141: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0141_list_names_in_english.md
// Run the tests:          cargo test practice::p0141
// ==================================================================

//! # Problem 0141: List Names in English
//!
//! - **Difficulty:** Easy
//! - **Topics:** slice patterns (`[a, b]`, `[rest @ .., last]`), `join`, `format!`, `&[&str]`
//! - **Source:** Original · [Slice patterns (Reference)](https://doc.rust-lang.org/reference/patterns.html#slice-patterns) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
//!
//! ## Problem
//!
//! Implement `list_names(names: &[&str]) -> String`, which writes a list of names the way an English
//! sentence would (with the "Oxford comma"):
//!
//! - no names: `""`
//! - one name: `"Ann"`
//! - two names: `"Ann and Ben"`
//! - three or more names: all names except the last separated by `", "`, then `", and "` and the
//!   last name, for example `"Ann, Ben, and Cy"`.
//!
//! ## Examples
//!
//! ```text
//! Input:  names = ["Ann", "Ben", "Cy"]
//! Output: "Ann, Ben, and Cy"
//!
//! Input:  names = ["Ann", "Ben"]
//! Output: "Ann and Ben"
//!
//! Input:  names = ["Ann"]
//! Output: "Ann"
//!
//! Input:  names = []
//! Output: ""
//! ```
//!
//! ## Constraints
//!
//! - `0 <= names.len() <= 10^4`
//!
//! ## Hints
//!
//! 1. A `match` on a slice can use patterns such as `[]`, `[one]`, `[first, second]`.
//! 2. `[rest @ .., last]` matches any non-empty slice and binds `rest: &[&str]` to everything but the
//!    last element.
//! 3. `rest.join(", ")` builds `"Ann, Ben"` from `["Ann", "Ben"]`.

pub fn list_names(names: &[&str]) -> String {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_three_names() {
        assert_eq!(list_names(&["Ann", "Ben", "Cy"]), "Ann, Ben, and Cy");
    }

    #[test]
    fn example_two_names() {
        assert_eq!(list_names(&["Ann", "Ben"]), "Ann and Ben");
    }

    #[test]
    fn example_one_name() {
        assert_eq!(list_names(&["Ann"]), "Ann");
    }

    #[test]
    fn example_no_names() {
        assert_eq!(list_names(&[]), "");
    }

    #[test]
    fn four_names() {
        assert_eq!(list_names(&["a", "b", "c", "d"]), "a, b, c, and d");
    }

    #[test]
    fn names_borrowed_from_owned_strings() {
        let owned = [String::from("Zoe"), String::from("Yan")];
        let borrowed: Vec<&str> = owned.iter().map(|s| s.as_str()).collect();
        assert_eq!(list_names(&borrowed), "Zoe and Yan");
    }

    #[test]
    fn many_names() {
        let names = vec!["x"; 10_000];
        let result = list_names(&names);
        assert!(result.starts_with("x, x, "));
        assert!(result.ends_with(", x, and x"));
        assert_eq!(result.len(), 9_999 * 3 + 4 + 1);
    }
}
