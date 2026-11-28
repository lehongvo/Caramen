// ==================================================================
// PRACTICE 0536: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0536_case_insensitive_key.md
// Run the tests:          cargo test practice::p0536
// ==================================================================

//! # Problem 0536: A Case-Insensitive Key
//!
//! - **Difficulty:** Medium
//! - **Topics:** manual `PartialEq`, `Eq` and `Hash`, the `Hash`/`Eq` consistency rule, `PartialEq<&str>`, `HashSet`/`HashMap` with custom keys
//! - **Source:** Original · [std::hash::Hash](https://doc.rust-lang.org/std/hash/trait.Hash.html#hash-and-eq)
//!
//! ## Problem
//!
//! Usernames are compared **ignoring ASCII case**: `"Alice"`, `"ALICE"` and `"alice"` are the same
//! user, but each value remembers how it was first written.
//!
//! Implement `Username` (a newtype around `String`):
//!
//! - `Username::new(name: &str) -> Self` and `as_str(&self) -> &str` (the original spelling).
//! - `PartialEq` and `Eq`: two usernames are equal when they are equal ignoring ASCII case.
//! - `Hash`: must be **consistent** with `Eq`: if `a == b` then `hash(a) == hash(b)`. Hash the
//!   lowercase form of every byte.
//! - `PartialEq<&str> for Username`, so `username == "ALICE"` compiles and ignores case too.
//!
//! Then write two functions:
//!
//! - `count_unique(names: &[&str]) -> usize`: the number of distinct users (use a
//!   `HashSet<Username>`).
//! - `logins_per_user(names: &[&str]) -> Vec<(String, usize)>`: how many times each user appears,
//!   reporting each user under the spelling of its **first** appearance, in order of first appearance.
//!
//! ## Examples
//!
//! ```text
//! Username::new("Alice") == Username::new("aLiCe")      => true
//! Username::new("Alice") == "ALICE"                     => true
//! count_unique(&["Alice", "bob", "ALICE", "Bob", "eve"]) => 3
//! logins_per_user(&["Alice", "bob", "ALICE", "Bob", "eve"])
//!     => [("Alice", 2), ("bob", 2), ("eve", 1)]
//! ```
//!
//! ## Constraints
//!
//! - `0 <= names.len() <= 10^5`
//! - Only ASCII letters are folded; other characters must match exactly.
//!
//! ## Hints
//!
//! 1. `str::eq_ignore_ascii_case` compares two strings ignoring ASCII case.
//! 2. In `Hash::hash`, feed `state` the lowercase bytes: `b.to_ascii_lowercase().hash(state)`. Using
//!    the derived `Hash` would hash `"Alice"` and `"alice"` differently and break the `HashSet`.
//! 3. For `logins_per_user`, a `HashMap<Username, usize>` maps each user to its index in the output
//!    vector.

use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

/// A username that compares and hashes case-insensitively.
#[derive(Debug, Clone)]
pub struct Username(String);

impl Username {
    pub fn new(name: &str) -> Self {
        todo!("Write your solution here")
    }

    pub fn as_str(&self) -> &str {
        todo!("Write your solution here")
    }
}

impl PartialEq for Username {
    fn eq(&self, other: &Self) -> bool {
        todo!("Write your solution here")
    }
}

impl Eq for Username {}

impl Hash for Username {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!("Write your solution here")
    }
}

impl PartialEq<&str> for Username {
    fn eq(&self, other: &&str) -> bool {
        todo!("Write your solution here")
    }
}

/// The number of distinct users.
pub fn count_unique(names: &[&str]) -> usize {
    todo!("Write your solution here")
}

/// Logins per user, under the first spelling, in order of first appearance.
pub fn logins_per_user(names: &[&str]) -> Vec<(String, usize)> {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::DefaultHasher;

    fn hash_of<T: Hash>(value: &T) -> u64 {
        let mut h = DefaultHasher::new();
        value.hash(&mut h);
        h.finish()
    }

    #[test]
    fn equality_ignores_ascii_case() {
        assert_eq!(Username::new("Alice"), Username::new("aLiCe"));
        assert_ne!(Username::new("Alice"), Username::new("Alicia"));
        assert!(Username::new("Alice") == "ALICE");
        assert!(Username::new("bob") != "rob");
        assert_eq!(Username::new("MiXeD").as_str(), "MiXeD");
    }

    #[test]
    fn equal_values_have_equal_hashes() {
        assert_eq!(
            hash_of(&Username::new("Alice")),
            hash_of(&Username::new("ALICE"))
        );
        assert_eq!(
            hash_of(&Username::new("x-1")),
            hash_of(&Username::new("X-1"))
        );
    }

    #[test]
    fn counting_users() {
        let names = ["Alice", "bob", "ALICE", "Bob", "eve"];
        assert_eq!(count_unique(&names), 3);
        assert_eq!(
            logins_per_user(&names),
            vec![
                ("Alice".to_string(), 2),
                ("bob".to_string(), 2),
                ("eve".to_string(), 1)
            ]
        );
        assert_eq!(count_unique(&[]), 0);
        assert!(logins_per_user(&[]).is_empty());
    }

    #[test]
    fn non_ascii_characters_must_match_exactly() {
        // Only ASCII letters are folded: 'E' with an accent and 'e' with an accent differ.
        assert_ne!(Username::new("\u{c9}va"), Username::new("\u{e9}va"));
        assert_eq!(count_unique(&["\u{c9}va", "\u{e9}va", "\u{e9}VA"]), 2);
    }

    #[test]
    fn many_names() {
        let owned: Vec<String> = (0..100_000)
            .map(|i| {
                let base = format!("user{}", i % 1000);
                if i % 2 == 0 {
                    base.to_uppercase()
                } else {
                    base
                }
            })
            .collect();
        let names: Vec<&str> = owned.iter().map(String::as_str).collect();
        assert_eq!(count_unique(&names), 1000);
        let logins = logins_per_user(&names);
        assert_eq!(logins.len(), 1000);
        assert_eq!(logins[0], ("USER0".to_string(), 100));
        assert_eq!(logins[1], ("user1".to_string(), 100));
    }
}
