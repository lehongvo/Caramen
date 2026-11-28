//! # Problem 0911: Normalize Whitespace with `Cow<str>`
//!
//! - **Difficulty:** Medium
//! - **Topics:** `std::borrow::Cow`, `Cow::Borrowed` vs `Cow::Owned`, avoiding allocations, lifetimes in return types
//! - **Source:** Original · [std: `Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
//!
//! ## Problem
//!
//! A text-cleaning function usually receives text that is **already clean**. Returning a new
//! `String` every time wastes an allocation; returning `Cow<'_, str>` lets you hand back the input
//! itself (`Cow::Borrowed`) when nothing changes and allocate (`Cow::Owned`) only when needed.
//!
//! Implement:
//!
//! - `normalize_whitespace(input: &str) -> Cow<'_, str>`: trim both ends, and replace every run of
//!   whitespace characters (`char::is_whitespace`: spaces, tabs, newlines, ...) by a **single
//!   space** `' '`. Return `Cow::Borrowed(input)` when the input is already in that form, i.e. it
//!   has no leading or trailing whitespace, no two whitespace characters in a row, and no whitespace
//!   other than `' '`.
//! - `ensure_suffix<'a>(text: &'a str, suffix: &str) -> Cow<'a, str>`: `text` unchanged (borrowed) if
//!   it already ends with `suffix`, otherwise `text + suffix` (owned).
//! - `allocations_needed(inputs: &[&str]) -> usize`: how many of the inputs would make
//!   `normalize_whitespace` allocate (return `Owned`).
//!
//! ## Examples
//!
//! ```text
//! normalize_whitespace("hello world")        => Borrowed("hello world")
//! normalize_whitespace("  hello \t\n world ") => Owned("hello world")
//! normalize_whitespace("a\tb")               => Owned("a b")      (a tab is not a plain space)
//! normalize_whitespace("   ")                => Owned("")
//! ensure_suffix("file.txt", ".txt")          => Borrowed("file.txt")
//! ensure_suffix("file", ".txt")              => Owned("file.txt")
//! allocations_needed(&["ok", " no", "fine too", "x  y"]) => 2
//! ```
//!
//! ## Constraints
//!
//! - `0 <= input.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. First decide **whether** anything must change with one pass over the characters (remember the
//!    previous character).
//! 2. If nothing changes, return `Cow::Borrowed(input)`. Otherwise build the new string with
//!    `input.split_whitespace()` joined by `" "`.
//! 3. A `Cow` can be compared with a `&str` directly, and `matches!(c, Cow::Borrowed(_))` tells you
//!    which variant you got.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `is_normalized` scans the characters once: it rejects a whitespace character that is not
//! `' '`, a space at either end, and a space right after another space. Only when it fails do we
//! allocate, using `split_whitespace` (which skips leading, trailing and repeated whitespace) and
//! `join(" ")`.
//!
//! **Complexity:** `O(n)` time; `O(1)` extra space when the input is borrowed, `O(n)` otherwise.
//!
//! **Rust concepts:**
//! - `Cow<'a, str>` is an enum: `Borrowed(&'a str)` or `Owned(String)`. It derefs to `&str`, so
//!   callers use it like any string slice.
//! - The lifetime in the return type ties a borrowed result to the input; elision fills it in for
//!   `normalize_whitespace`, while `ensure_suffix` needs an explicit `'a` because it has two
//!   reference parameters and only `text` may be returned.
//! - `Cow::Owned(format!(...))` and `Cow::from(string)` both build the owned variant.
// @solution-end

use std::borrow::Cow;

// @solution-start remove
fn is_normalized(input: &str) -> bool {
    if input.starts_with(' ') || input.ends_with(' ') {
        return false;
    }
    let mut previous_was_space = false;
    for c in input.chars() {
        if c.is_whitespace() {
            if c != ' ' || previous_was_space {
                return false;
            }
            previous_was_space = true;
        } else {
            previous_was_space = false;
        }
    }
    true
}
// @solution-end

pub fn normalize_whitespace(input: &str) -> Cow<'_, str> {
    // @solution-start
    if is_normalized(input) {
        Cow::Borrowed(input)
    } else {
        Cow::Owned(input.split_whitespace().collect::<Vec<_>>().join(" "))
    }
    // @solution-end
}

pub fn ensure_suffix<'a>(text: &'a str, suffix: &str) -> Cow<'a, str> {
    // @solution-start
    if text.ends_with(suffix) {
        Cow::Borrowed(text)
    } else {
        Cow::Owned(format!("{text}{suffix}"))
    }
    // @solution-end
}

pub fn allocations_needed(inputs: &[&str]) -> usize {
    // @solution-start
    inputs
        .iter()
        .filter(|input| matches!(normalize_whitespace(input), Cow::Owned(_)))
        .count()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    // Clippy prefers `&str` over `&Cow<str>`, but here we need to inspect the variant itself.
    #[allow(clippy::ptr_arg)]
    fn is_borrowed(c: &Cow<'_, str>) -> bool {
        matches!(c, Cow::Borrowed(_))
    }

    #[test]
    fn clean_input_is_borrowed() {
        let input = "hello world";
        let out = normalize_whitespace(input);
        assert!(is_borrowed(&out));
        assert_eq!(out, "hello world");
        // Borrowed really means "the same bytes": no copy was made.
        assert_eq!(out.as_ptr(), input.as_ptr());
        assert!(is_borrowed(&normalize_whitespace("")));
        assert!(is_borrowed(&normalize_whitespace("x")));
    }

    #[test]
    fn messy_input_is_owned_and_fixed() {
        let out = normalize_whitespace("  hello \t\n world ");
        assert!(!is_borrowed(&out));
        assert_eq!(out, "hello world");
        assert_eq!(normalize_whitespace("a\tb"), "a b");
        assert_eq!(normalize_whitespace("a  b"), "a b");
        assert_eq!(normalize_whitespace("   "), "");
        assert_eq!(normalize_whitespace("\u{a0}x"), "x");
    }

    #[test]
    fn ensure_suffix_borrows_when_possible() {
        let done = ensure_suffix("file.txt", ".txt");
        assert!(is_borrowed(&done));
        let added = ensure_suffix("file", ".txt");
        assert!(!is_borrowed(&added));
        assert_eq!(added, "file.txt");
        assert!(is_borrowed(&ensure_suffix("x", "")));
    }

    #[test]
    fn counting_allocations() {
        assert_eq!(allocations_needed(&["ok", " no", "fine too", "x  y"]), 2);
        assert_eq!(allocations_needed(&[]), 0);
    }

    #[test]
    fn long_inputs() {
        let clean = "word ".repeat(20_000);
        let clean = clean.trim_end();
        assert!(is_borrowed(&normalize_whitespace(clean)));
        let messy = "word \n\t ".repeat(20_000);
        let out = normalize_whitespace(&messy);
        assert_eq!(out.len(), clean.len());
        assert_eq!(out, clean);
    }
}
