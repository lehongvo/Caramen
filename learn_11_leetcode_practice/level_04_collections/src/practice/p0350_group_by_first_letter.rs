// ==================================================================
// PRACTICE 0350: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0350_group_by_first_letter.md
// Run the tests:          cargo test practice::p0350
// ==================================================================

//! # Problem 0350: Group by First Letter
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Entry::or_default`, `and_modify(...).or_insert(...)`, `or_insert_with`, `HashMap<char, Vec<String>>`
//! - **Source:** Original · [`hash_map::Entry`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html)
//!
//! ## Problem
//!
//! Practise the three most useful methods of the entry API:
//!
//! 1. `group_by_initial(words: &[&str]) -> HashMap<char, Vec<String>>` groups the words by their
//!    first character converted to ASCII lowercase. Inside a group, words keep their input order.
//!    Empty strings are skipped.
//! 2. `tally_votes(votes: &[&str]) -> HashMap<String, u32>` counts the votes for each candidate,
//!    using `and_modify` to increment existing counts and `or_insert` to start new ones at `1`.
//! 3. `longest_per_initial(words: &[&str]) -> HashMap<char, String>` keeps, for every initial
//!    (lowercased as in 1.), the longest word with that initial; on equal lengths the word that
//!    came **first** wins. Empty strings are skipped.
//!
//! ## Examples
//!
//! ```text
//! words = ["apple", "Bean", "avocado", "banana", "Apricot"]
//! group_by_initial    => {'a': ["apple", "avocado", "Apricot"], 'b': ["Bean", "banana"]}
//! longest_per_initial => {'a': "avocado", 'b': "banana"}
//!
//! votes = ["ann", "bob", "ann"]
//! tally_votes => {"ann": 2, "bob": 1}
//! ```
//!
//! (The order of the keys when printing a `HashMap` is unspecified.)
//!
//! ## Constraints
//!
//! - At most `10^4` words or votes.
//!
//! ## Hints
//!
//! 1. `map.entry(k).or_default()` inserts `V::default()` (an empty `Vec` here) if the key is
//!    missing and returns `&mut V`.
//! 2. `map.entry(k).and_modify(|v| ...).or_insert(x)` runs the closure if the key exists, and
//!    inserts `x` otherwise.
//! 3. `or_insert_with(|| ...)` builds the default lazily, only when it is needed.

use std::collections::HashMap;

/// Groups words by their lowercase first character, keeping input order inside each group.
pub fn group_by_initial(words: &[&str]) -> HashMap<char, Vec<String>> {
    todo!("Write your solution here")
}

/// Counts the votes for each candidate.
pub fn tally_votes(votes: &[&str]) -> HashMap<String, u32> {
    todo!("Write your solution here")
}

/// Keeps the longest word for each lowercase initial (the first one on ties).
pub fn longest_per_initial(words: &[&str]) -> HashMap<char, String> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(words: &[&str]) -> Vec<String> {
        words.iter().map(|s| s.to_string()).collect()
    }

    const WORDS: [&str; 5] = ["apple", "Bean", "avocado", "banana", "Apricot"];

    #[test]
    fn group_by_initial_example() {
        let groups = group_by_initial(&WORDS);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[&'a'], strings(&["apple", "avocado", "Apricot"]));
        assert_eq!(groups[&'b'], strings(&["Bean", "banana"]));
    }

    #[test]
    fn longest_per_initial_example_and_ties() {
        let best = longest_per_initial(&WORDS);
        let expected: HashMap<char, String> =
            [('a', "avocado".to_string()), ('b', "banana".to_string())]
                .into_iter()
                .collect();
        assert_eq!(best, expected);
        let ties = longest_per_initial(&["cat", "cow", "Cup", "dog"]);
        assert_eq!(ties[&'c'], "cat");
        assert_eq!(ties[&'d'], "dog");
    }

    #[test]
    fn tally_votes_example() {
        let tally = tally_votes(&["ann", "bob", "ann"]);
        let expected: HashMap<String, u32> = [("ann".to_string(), 2), ("bob".to_string(), 1)]
            .into_iter()
            .collect();
        assert_eq!(tally, expected);
    }

    #[test]
    fn empty_inputs_and_empty_words() {
        assert!(group_by_initial(&[]).is_empty());
        assert!(group_by_initial(&["", ""]).is_empty());
        assert!(longest_per_initial(&[""]).is_empty());
        assert!(tally_votes(&[]).is_empty());
        assert_eq!(tally_votes(&[""])[""], 1);
    }

    #[test]
    fn many_votes() {
        let names = ["x", "y", "z"];
        let votes: Vec<&str> = (0..10_000).map(|i| names[(i * i) % 3]).collect();
        let tally = tally_votes(&votes);
        assert_eq!(tally.values().sum::<u32>(), 10_000);
        // i*i % 3 is 0 when 3 divides i, and 1 otherwise: "z" never gets a vote.
        assert_eq!(tally.get("z"), None);
        assert_eq!(tally["x"], 3_334);
        assert_eq!(tally["y"], 6_666);
    }
}
