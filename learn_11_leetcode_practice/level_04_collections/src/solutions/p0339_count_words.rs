//! # Problem 0339: Count Words
//!
//! - **Difficulty:** Easy
//! - **Topics:** `HashMap::new`, `entry(key).or_insert(0)`, `*count += 1`, `get` + `copied`
//! - **Source:** Original · [The Rust Book 8.3](https://doc.rust-lang.org/book/ch08-03-hash-maps.html) · [`HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry)
//!
//! ## Problem
//!
//! Implement three small text statistics with `std::collections::HashMap`:
//!
//! 1. `word_counts(text: &str) -> HashMap<String, usize>` splits `text` on whitespace, converts
//!    every word to lowercase, and maps each distinct word to the number of times it occurs.
//! 2. `count_of(counts: &HashMap<String, usize>, word: &str) -> usize` returns the count stored
//!    for `word`, or `0` if the word is not in the map. It must not modify the map.
//! 3. `char_counts(text: &str) -> HashMap<char, usize>` counts every character that is **not**
//!    whitespace (case-sensitive).
//!
//! ## Examples
//!
//! ```text
//! let counts = word_counts("the cat saw The dog");
//! counts.len()             => 4
//! count_of(&counts, "the") => 2
//! count_of(&counts, "cat") => 1
//! count_of(&counts, "cow") => 0
//!
//! char_counts("a ba") => {'a': 2, 'b': 1}
//! ```
//!
//! A `HashMap` has no defined order, so the examples only look up individual keys.
//!
//! ## Constraints
//!
//! - `0 <= text.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `map.entry(key).or_insert(0)` returns a `&mut usize` to the value for `key`, inserting `0`
//!    first if the key is missing. Increment it with `*count += 1`.
//! 2. `map.get(word)` returns `Option<&usize>`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the classic word count from The Rust Book. For every word, `entry` finds or creates
//! the slot for that word, and the returned mutable reference is incremented in place.
//!
//! **Rust concepts:**
//! - The **entry API** does the lookup once: `entry(key)` returns an `Entry` (occupied or vacant),
//!   and `or_insert(default)` gives `&mut V` in both cases.
//! - The key must be owned (`String`) because the map outlives `text`; `to_lowercase()` already
//!   returns a new `String`.
//! - `get` accepts a `&str` for a `HashMap<String, _>` because `String: Borrow<str>`; no `String`
//!   needs to be allocated just to look something up.
//! - `copied().unwrap_or(0)` turns `Option<&usize>` into a plain `usize`.
// @solution-end

use std::collections::HashMap;

/// Maps each lowercase word to its number of occurrences.
pub fn word_counts(text: &str) -> HashMap<String, usize> {
    // @solution-start
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }
    counts
    // @solution-end
}

/// Returns the count for `word`, or 0.
pub fn count_of(counts: &HashMap<String, usize>, word: &str) -> usize {
    // @solution-start
    counts.get(word).copied().unwrap_or(0)
    // @solution-end
}

/// Counts every non-whitespace character.
pub fn char_counts(text: &str) -> HashMap<char, usize> {
    // @solution-start
    let mut counts = HashMap::new();
    for c in text.chars().filter(|c| !c.is_whitespace()) {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_counts_example() {
        let counts = word_counts("the cat saw The dog");
        assert_eq!(counts.len(), 4);
        assert_eq!(count_of(&counts, "the"), 2);
        assert_eq!(count_of(&counts, "cat"), 1);
        assert_eq!(count_of(&counts, "saw"), 1);
        assert_eq!(count_of(&counts, "dog"), 1);
        assert_eq!(count_of(&counts, "cow"), 0);
        assert_eq!(count_of(&counts, "The"), 0);
    }

    #[test]
    fn whitespace_only_text() {
        assert!(word_counts("  \n\t ").is_empty());
        assert!(char_counts("  \n\t ").is_empty());
    }

    #[test]
    fn char_counts_example() {
        let counts = char_counts("a ba");
        let expected: HashMap<char, usize> = [('a', 2), ('b', 1)].into_iter().collect();
        assert_eq!(counts, expected);
        let upper = char_counts("Aa");
        assert_eq!(upper.get(&'A'), Some(&1));
        assert_eq!(upper.get(&'a'), Some(&1));
    }

    #[test]
    fn large_text() {
        let text = "Rust rust RUST is fun ".repeat(10_000);
        let counts = word_counts(&text);
        assert_eq!(counts.len(), 3);
        assert_eq!(count_of(&counts, "rust"), 30_000);
        assert_eq!(count_of(&counts, "fun"), 10_000);
        assert_eq!(char_counts(&text).get(&'u'), Some(&30_000));
    }
}
