//! # Problem 0561: The Longer of Two Strings
//!
//! - **Difficulty:** Easy
//! - **Topics:** lifetime parameters in function signatures, `'a` relating inputs and outputs, outputs tied to only one input
//! - **Source:** Original · [The Rust Book 10.3](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#generic-lifetimes-in-functions)
//!
//! ## Problem
//!
//! When a function returns a reference, the compiler must know **which input** the reference may point
//! into. Lifetime parameters say so. Implement:
//!
//! - `longest<'a>(x: &'a str, y: &'a str) -> &'a str`: the longer string (by bytes), `x` on a tie. The
//!   result may borrow from either input, so both share `'a`.
//! - `prefix_before<'a>(text: &'a str, delimiter: &str) -> &'a str`: the part of `text` before the
//!   first occurrence of `delimiter` (all of `text` when it does not occur). The result never borrows
//!   from `delimiter`, so `delimiter` keeps an elided lifetime of its **own** (as if you had written a
//!   separate `'b`), and callers may drop the delimiter's owner while still using the result.
//! - `longest_word<'a>(words: &[&'a str]) -> Option<&'a str>`: the longest of many words (first on
//!   ties). The result borrows from the **strings**, not from the slice that lists them, so the slice
//!   may be dropped while the result is still used.
//!
//! ## Examples
//!
//! ```text
//! longest("long string is long", "xyz")    => "long string is long"
//! longest("ab", "cd")                      => "ab"
//! prefix_before("key=value", "=")         => "key"
//! prefix_before("no delimiter", ";")      => "no delimiter"
//! longest_word(&["a", "abc", "ab"])        => Some("abc")
//! longest_word(&[])                        => None
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Without lifetimes, `fn longest(x: &str, y: &str) -> &str` does not compile (error E0106): the
//!    compiler cannot tell whether the result comes from `x` or `y`.
//! 2. `'a` does not change how long anything lives; it only describes a relationship that the borrow
//!    checker then verifies at every call site.
//! 3. A string literal has type `&'static str`, which can be used where any shorter `&'a str` is
//!    expected.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the bodies are short; the signatures carry the meaning. `longest` ties both inputs to the
//! output. `prefix_before` ties only `text`. `longest_word` ties the output to the element lifetime `'a`,
//! leaving the slice borrow anonymous.
//!
//! **Rust concepts:**
//! - With `longest<'a>`, the returned reference is valid for the *shorter* of the two input lifetimes;
//!   using it after either input is dropped is rejected.
//! - Naming only the lifetime the output depends on (and leaving the others elided, which makes each
//!   of them a separate anonymous lifetime) makes the function easier to call. Writing
//!   `<'a, 'b>(text: &'a str, delimiter: &'b str)` would mean the same; Clippy suggests the shorter form.
//! - In `&[&'a str]` there are two lifetimes: the anonymous one of the slice borrow and `'a` of the
//!   strings. The output names only `'a`.
// @solution-end

/// The longer of two strings (`x` on a tie).
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // @solution-start
    if y.len() > x.len() { y } else { x }
    // @solution-end
}

/// The part of `text` before the first `delimiter`. Never borrows from `delimiter`.
pub fn prefix_before<'a>(text: &'a str, delimiter: &str) -> &'a str {
    // @solution-start
    match text.find(delimiter) {
        Some(i) => &text[..i],
        None => text,
    }
    // @solution-end
}

/// The longest word (first on ties); the result borrows from the strings, not the slice.
pub fn longest_word<'a>(words: &[&'a str]) -> Option<&'a str> {
    // @solution-start
    let mut best: Option<&'a str> = None;
    for &w in words {
        if best.is_none_or(|b| w.len() > b.len()) {
            best = Some(w);
        }
    }
    best
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_examples() {
        assert_eq!(longest("long string is long", "xyz"), "long string is long");
        assert_eq!(longest("ab", "cd"), "ab");
        assert_eq!(longest("", "x"), "x");
    }

    #[test]
    fn longest_within_a_scope() {
        // The Book's example: the result is used while both strings are alive.
        let string1 = String::from("long string is long");
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            assert_eq!(result, "long string is long");
        } // using `result` after this point would not compile: `string2` is gone
    }

    #[test]
    fn prefix_before_does_not_borrow_the_delimiter() {
        let line = String::from("key=value");
        let key;
        {
            let delimiter = String::from("=");
            key = prefix_before(&line, &delimiter);
        } // `delimiter` is gone, `key` only borrows from `line`
        assert_eq!(key, "key");
        assert_eq!(prefix_before("no delimiter", ";"), "no delimiter");
        assert_eq!(prefix_before("a::b::c", "::"), "a");
        assert_eq!(prefix_before("", "x"), "");
    }

    #[test]
    fn longest_word_outlives_the_slice() {
        let text = String::from("a abc ab abd");
        let best;
        {
            let words: Vec<&str> = text.split(' ').collect();
            best = longest_word(&words);
        } // `words` is dropped; `best` borrows from `text`
        assert_eq!(best, Some("abc"));
        assert_eq!(longest_word(&[]), None);
    }
}
