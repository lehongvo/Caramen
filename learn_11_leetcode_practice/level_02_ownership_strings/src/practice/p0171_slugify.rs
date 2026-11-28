// ==================================================================
// PRACTICE 0171: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0171_slugify.md
// Run the tests:          cargo test practice::p0171
// ==================================================================

//! # Problem 0171: Slugify a Title
//!
//! - **Difficulty:** Medium
//! - **Topics:** choosing `&str` in and `String` out, a "pending separator" flag, `is_ascii_alphanumeric`, validating with `bytes().all`
//! - **Source:** Original · [`char::is_ascii_alphanumeric`](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_alphanumeric) · [The Rust Book 8.2](https://doc.rust-lang.org/book/ch08-02-strings.html)
//!
//! ## Problem
//!
//! Blogs turn article titles into URL *slugs*, such as `"hello-world"` for `"Hello, World!"`.
//! Implement two functions:
//!
//! 1. `slugify(title: &str) -> String` builds the slug:
//!    - ASCII letters are kept and converted to lowercase; ASCII digits are kept;
//!    - every run of other characters (spaces, punctuation, non-ASCII characters, ...) between two
//!      kept characters becomes a **single** `'-'`;
//!    - the slug never starts or ends with `'-'`; a title with no ASCII letter or digit gives `""`.
//! 2. `is_slug(text: &str) -> bool` returns `true` if `text` is a valid slug: non-empty, made only of
//!    `a`-`z`, `0`-`9` and `'-'`, not starting or ending with `'-'`, and without `"--"`.
//!
//! The parameter types are `&str` because both functions only read their input; `slugify` returns a
//! `String` because it produces new text, while `is_slug` just answers a question.
//!
//! ## Examples
//!
//! ```text
//! Input:  slugify("Hello, World!")
//! Output: "hello-world"
//!
//! Input:  slugify("  Rust 1.87 -- Released!  ")
//! Output: "rust-1-87-released"
//!
//! Input:  slugify("Grüße aus Köln")
//! Output: "gr-e-aus-k-ln"      (non-ASCII letters are separators)
//!
//! Input:  slugify("¿?")
//! Output: ""
//!
//! Input:  is_slug("rust-1-87")   -> true
//! Input:  is_slug("rust--lang")  -> false
//! Input:  is_slug("-rust")       -> false
//! ```
//!
//! ## Constraints
//!
//! - `0 <= title.len(), text.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. Do not push a `'-'` as soon as you see a separator: remember that one is *pending* and push it
//!    only when the next kept character arrives (and only if the slug is not empty yet).
//! 2. That rule alone avoids leading, trailing and doubled dashes.
//! 3. For `is_slug`, combine `starts_with`, `ends_with`, `contains("--")` and `bytes().all(...)`.

pub fn slugify(title: &str) -> String {
    todo!("Write your solution here")
}

pub fn is_slug(text: &str) -> bool {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_hello_world() {
        assert_eq!(slugify("Hello, World!"), "hello-world");
    }

    #[test]
    fn example_digits_and_runs_of_separators() {
        assert_eq!(slugify("  Rust 1.87 -- Released!  "), "rust-1-87-released");
    }

    #[test]
    fn example_non_ascii_letters_are_separators() {
        assert_eq!(slugify("Grüße aus Köln"), "gr-e-aus-k-ln");
    }

    #[test]
    fn example_nothing_to_keep() {
        assert_eq!(slugify("¿?"), "");
        assert_eq!(slugify(""), "");
    }

    #[test]
    fn example_is_slug() {
        assert!(is_slug("rust-1-87"));
        assert!(!is_slug("rust--lang"));
        assert!(!is_slug("-rust"));
    }

    #[test]
    fn is_slug_rejects_bad_characters_and_edges() {
        assert!(!is_slug(""));
        assert!(!is_slug("rust-"));
        assert!(!is_slug("Rust"));
        assert!(!is_slug("rust lang"));
        assert!(!is_slug("über"));
        assert!(is_slug("a"));
        assert!(is_slug("2024"));
    }

    #[test]
    fn every_non_empty_slug_is_valid() {
        for title in ["A -- B", "...x...", "C++ & Rust", "日本 2 go", "--a--b--"] {
            let slug = slugify(title);
            assert!(is_slug(&slug), "{title:?} gave {slug:?}");
        }
    }

    #[test]
    fn large_title() {
        let title = "Word! ".repeat(10_000);
        let slug = slugify(&title);
        assert_eq!(slug.len(), 10_000 * 5 - 1);
        assert!(is_slug(&slug));
    }
}
