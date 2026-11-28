// ==================================================================
// PRACTICE 0881: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0881_parallel_word_count.md
// Run the tests:          cargo test practice::p0881
// ==================================================================

//! # Problem 0881: Parallel Word Count
//!
//! - **Difficulty:** Medium
//! - **Topics:** map-reduce with scoped threads, a private `HashMap` per worker merged at the end,
//!   `BTreeMap` for a deterministic result
//! - **Source:** Original · [`thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html)
//!
//! ## Problem
//!
//! Implement `word_frequencies(lines: &[&str], workers: usize) -> BTreeMap<String, usize>`.
//!
//! A **word** is a maximal run of ASCII letters (`a-z`, `A-Z`); everything else (digits, spaces,
//! punctuation, other characters) separates words. Words are counted case-insensitively and stored
//! in lower case. Return how many times each word occurs in all lines together.
//!
//! Use a **map-reduce** design: split `lines` into at most `workers` contiguous chunks; each scoped
//! thread counts the words of its chunk into its **own** `HashMap` (no shared lock while counting);
//! then merge the partial maps into the final `BTreeMap`.
//!
//! ## Examples
//!
//! ```text
//! word_frequencies(&["The cat", "the CAT sat.", "dog"], 2)
//!   -> {"cat": 2, "dog": 1, "sat": 1, "the": 2}
//! word_frequencies(&["it's 42-ish"], 1) -> {"ish": 1, "it": 1, "s": 1}
//! word_frequencies(&[], 4)              -> {}
//! ```
//!
//! ## Constraints
//!
//! - `1 <= workers <= 64`; `0 <= lines.len() <= 10^5`
//!
//! ## Hints
//!
//! 1. `line.split(|c: char| !c.is_ascii_alphabetic()).filter(|w| !w.is_empty())` yields the words.
//! 2. Workers that share one `Mutex<HashMap>` would spend their time waiting for the lock.
//!    Counting locally and merging once per worker scales much better.
//! 3. A `BTreeMap` iterates in sorted key order, which makes the result easy to compare.

use std::collections::{BTreeMap, HashMap};
use std::thread;

pub fn word_frequencies(lines: &[&str], workers: usize) -> BTreeMap<String, usize> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map(pairs: &[(&str, usize)]) -> BTreeMap<String, usize> {
        pairs.iter().map(|&(w, c)| (w.to_string(), c)).collect()
    }

    #[test]
    fn example_case_insensitive() {
        let got = word_frequencies(&["The cat", "the CAT sat.", "dog"], 2);
        assert_eq!(got, map(&[("cat", 2), ("dog", 1), ("sat", 1), ("the", 2)]));
    }

    #[test]
    fn example_separators() {
        let got = word_frequencies(&["it's 42-ish"], 1);
        assert_eq!(got, map(&[("ish", 1), ("it", 1), ("s", 1)]));
    }

    #[test]
    fn example_no_lines() {
        assert!(word_frequencies(&[], 4).is_empty());
    }

    #[test]
    fn non_ascii_letters_split_words() {
        let got = word_frequencies(&["naïve über", "   ", ""], 3);
        assert_eq!(got, map(&[("ber", 1), ("na", 1), ("ve", 1)]));
    }

    #[test]
    fn large_input_same_for_any_worker_count() {
        let lines: Vec<String> = (0..20_000)
            .map(|i| {
                format!(
                    "alpha beta{} Gamma, {}",
                    i % 10,
                    if i % 2 == 0 { "even" } else { "odd" }
                )
            })
            .collect();
        let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
        let one = word_frequencies(&refs, 1);
        let many = word_frequencies(&refs, 16);
        assert_eq!(one, many);
        // "beta0" splits into "beta" because digits separate words.
        assert_eq!(
            many,
            map(&[
                ("alpha", 20_000),
                ("beta", 20_000),
                ("even", 10_000),
                ("gamma", 20_000),
                ("odd", 10_000)
            ])
        );
    }
}
