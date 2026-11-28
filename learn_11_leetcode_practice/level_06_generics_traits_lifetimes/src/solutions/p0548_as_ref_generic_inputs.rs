//! # Problem 0548: Flexible Inputs with `AsRef`
//!
//! - **Difficulty:** Easy
//! - **Topics:** `AsRef<str>`, `AsRef<[T]>`, `IntoIterator<Item = S>` bounds, accepting owned and borrowed inputs with one function
//! - **Source:** Original · [std::convert::AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html)
//!
//! ## Problem
//!
//! A function that takes `&str` cannot be called with a `Vec<String>` element without `as_str()`, and one
//! that takes `String` forces callers to allocate. `AsRef` accepts all of them: `S: AsRef<str>` is
//! satisfied by `&str`, `String`, `&String`, `Box<str>`, ...
//!
//! Implement:
//!
//! - `count_vowels<S: AsRef<str>>(text: S) -> usize`: the number of ASCII vowels (`aeiou`, either case).
//! - `total_chars<I, S>(items: I) -> usize where I: IntoIterator<Item = S>, S: AsRef<str>`: the total
//!   number of characters (`char`s) of all items. Works for `Vec<String>`, `[&str; N]`, `&[String]`,
//!   iterators of `String`s, ...
//! - `longest<S: AsRef<str>>(words: &[S]) -> Option<&str>`: the longest word (by characters, first on
//!   ties), returned as a `&str` borrowed from `words`.
//! - `checksum<B: AsRef<[u8]>>(data: B) -> u32`: the wrapping sum of all bytes. Works for `&[u8]`,
//!   `Vec<u8>`, `[u8; N]`, `&str` and `String` (strings expose their UTF-8 bytes through `AsRef<[u8]>`).
//!
//! ## Examples
//!
//! ```text
//! count_vowels("Programming")                 => 3
//! count_vowels(String::from("AEIOU xyz"))     => 5
//! total_chars(vec!["ab".to_string(), "cde".to_string()]) => 5
//! total_chars(["caf\u{e9}", "!"])             => 5
//! longest(&["hi", "hello", "hey"])            => Some("hello")
//! checksum(b"abc")                            => 294
//! checksum("abc")                             => 294
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Inside the function, call `.as_ref()` once to get a `&str` (or `&[u8]`).
//! 2. In `longest`, `words[i].as_ref()` borrows from the slice, so it can be returned with the slice's
//!    lifetime (elision does the rest).
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** every function converts its generic input to a borrowed view (`&str` or `&[u8]`) with
//! `as_ref()` and then works on that view.
//!
//! **Rust concepts:**
//! - `AsRef<T>` is a cheap reference-to-reference conversion; unlike `Into`, it never takes ownership
//!   or allocates.
//! - `IntoIterator<Item = S>` accepts collections and iterators alike; with `S: AsRef<str>` the items
//!   can be owned `String`s or borrowed `&str`s.
//! - The returned `&str` in `longest` has the lifetime of `words`; since `S` could be `String`, the
//!   borrow points into the `String`s stored in the slice.
// @solution-end

/// The number of ASCII vowels.
pub fn count_vowels<S: AsRef<str>>(text: S) -> usize {
    // @solution-start
    text.as_ref()
        .chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
    // @solution-end
}

/// The total number of characters of all items.
pub fn total_chars<I, S>(items: I) -> usize
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    // @solution-start
    items.into_iter().map(|s| s.as_ref().chars().count()).sum()
    // @solution-end
}

/// The longest word (first on ties).
pub fn longest<S: AsRef<str>>(words: &[S]) -> Option<&str> {
    // @solution-start
    let mut best: Option<&str> = None;
    for word in words {
        let w = word.as_ref();
        if best.is_none_or(|b| w.chars().count() > b.chars().count()) {
            best = Some(w);
        }
    }
    best
    // @solution-end
}

/// The wrapping sum of all bytes.
pub fn checksum<B: AsRef<[u8]>>(data: B) -> u32 {
    // @solution-start
    data.as_ref()
        .iter()
        .fold(0_u32, |acc, &b| acc.wrapping_add(u32::from(b)))
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vowels_from_many_string_types() {
        assert_eq!(count_vowels("Programming"), 3);
        assert_eq!(count_vowels(String::from("AEIOU xyz")), 5);
        let owned = String::from("queue");
        assert_eq!(count_vowels(&owned), 4);
        let boxed: Box<str> = "rhythm".into();
        assert_eq!(count_vowels(boxed), 0);
    }

    #[test]
    fn total_chars_from_collections_and_iterators() {
        assert_eq!(total_chars(vec!["ab".to_string(), "cde".to_string()]), 5);
        assert_eq!(total_chars(["caf\u{e9}", "!"]), 5);
        let words = vec![String::from("x"), String::from("yz")];
        assert_eq!(total_chars(&words), 3);
        assert_eq!(total_chars((1..=3).map(|n| "a".repeat(n))), 6);
        assert_eq!(total_chars(Vec::<&str>::new()), 0);
    }

    #[test]
    fn longest_borrows_from_the_slice() {
        assert_eq!(longest(&["hi", "hello", "hey"]), Some("hello"));
        let owned = vec!["tie".to_string(), "two".to_string()];
        assert_eq!(longest(&owned), Some("tie"));
        assert_eq!(longest::<&str>(&[]), None);
        assert_eq!(longest(&["\u{e9}\u{e9}\u{e9}", "abcd"]), Some("abcd"));
    }

    #[test]
    fn checksums_from_bytes_and_strings() {
        assert_eq!(checksum(b"abc"), 294);
        assert_eq!(checksum("abc"), 294);
        assert_eq!(checksum(String::from("abc")), 294);
        assert_eq!(checksum(vec![255_u8; 3]), 765);
        assert_eq!(checksum([0_u8; 0]), 0);
        // 255 * 17_000_000 does not fit in u32, so the sum wraps around.
        let big = vec![255_u8; 17_000_000];
        assert_eq!(
            checksum(&big),
            (255_u64 * 17_000_000 % (1_u64 << 32)) as u32
        );
    }
}
