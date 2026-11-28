//! # Problem 0906: Collection Literals: `hashmap!` and `hashset!`
//!
//! - **Difficulty:** Medium
//! - **Topics:** `macro_rules!` repetitions, `$k:expr => $v:expr`, absolute paths `::std::...`, pre-allocation
//! - **Source:** Original · [Rust by Example: Repeat](https://doc.rust-lang.org/rust-by-example/macros/repeat.html) · [Reference: Macros By Example](https://doc.rust-lang.org/reference/macros-by-example.html#repetitions)
//!
//! ## Problem
//!
//! `vec![...]` builds a `Vec` from a literal list; the standard library has no such macro for maps
//! and sets. Write three:
//!
//! - `hashmap! { k1 => v1, k2 => v2, ... }` builds a `std::collections::HashMap`.
//! - `btreemap! { k1 => v1, ... }` builds a `std::collections::BTreeMap`.
//! - `hashset![e1, e2, ...]` builds a `std::collections::HashSet`.
//!
//! Rules:
//!
//! - An empty invocation (`hashmap! {}`) gives an empty collection; a trailing comma is accepted.
//! - Pairs are inserted from left to right, so a repeated key keeps the **last** value.
//! - Every key and value expression is evaluated exactly once, from left to right.
//! - `hashmap!` and `hashset!` create the collection with `with_capacity(n)` where `n` is the number
//!   of entries written in the macro call (count them at compile time, without evaluating anything).
//! - The macros must work where the caller has **not** imported `HashMap`, `BTreeMap` or `HashSet`.
//!
//! Then use them in `fn roman_values() -> BTreeMap<char, u32>` (the 7 Roman numeral letters
//! `I V X L C D M` with their values) and `fn vowels() -> HashSet<char>` (`a e i o u`).
//!
//! ## Examples
//!
//! ```text
//! let ages = hashmap! { "ann" => 31, "bob" => 27, };   // HashMap<&str, i32> with 2 entries
//! let dup  = hashmap! { 1 => "a", 1 => "b" };          // {1: "b"}
//! let set  = hashset![3, 1, 3];                        // {1, 3}
//! let tree = btreemap! { 'b' => 2, 'a' => 1 };         // iterates as ('a', 1), ('b', 2)
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. The matcher `$($key:expr => $value:expr),* $(,)?` accepts zero or more pairs.
//! 2. Write the type as `::std::collections::HashMap` inside the expansion: paths in a macro are
//!    resolved where the macro is **used**.
//! 3. To count entries without evaluating them, map each one to `()` and take the slice length
//!    (problem 0905).
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** each macro expands to a block that creates the collection, inserts every entry in order
//! and evaluates to the collection. The capacity is the length of an array of `()` values, one per
//! entry, which the compiler computes without touching the entry expressions.
//!
//! **Rust concepts:**
//! - `$( map.insert($key, $value); )*` repeats a statement once per captured pair.
//! - `=>` and `,` are allowed after an `expr` fragment (the "follow-set" rules restrict what may come
//!   after `expr`, `ty`, `path`...).
//! - The empty invocation gets its own arm: otherwise the expansion would declare `let mut map`
//!   and never mutate it, and the compiler would warn about an unneeded `mut`.
//! - Absolute paths starting with `::std` keep working even if the caller shadows `std` or has not
//!   imported anything; local variables in the expansion (`map`, `set`) are hygienic.
//! - The element types are inferred from the inserted values or from the caller's annotation.
// @solution-end

// @solution-start remove
macro_rules! unit {
    ($t:tt) => {
        ()
    };
}
// @solution-end

// Only invoked by the tests below (in a real crate it would be `#[macro_export]`ed).
#[allow(unused_macros)]
macro_rules! hashmap {
    // @solution-start replace: ($($t:tt)*) => { todo!() };
    () => {
        ::std::collections::HashMap::new()
    };
    ($($key:expr => $value:expr),* $(,)?) => {{
        let capacity = <[()]>::len(&[$(unit!($key)),*]);
        let mut map = ::std::collections::HashMap::with_capacity(capacity);
        $( map.insert($key, $value); )*
        map
    }};
    // @solution-end
}

macro_rules! btreemap {
    // @solution-start replace: ($($t:tt)*) => { todo!() };
    () => {
        ::std::collections::BTreeMap::new()
    };
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = ::std::collections::BTreeMap::new();
        $( map.insert($key, $value); )*
        map
    }};
    // @solution-end
}

macro_rules! hashset {
    // @solution-start replace: ($($t:tt)*) => { todo!() };
    () => {
        ::std::collections::HashSet::new()
    };
    ($($element:expr),* $(,)?) => {{
        let capacity = <[()]>::len(&[$(unit!($element)),*]);
        let mut set = ::std::collections::HashSet::with_capacity(capacity);
        $( set.insert($element); )*
        set
    }};
    // @solution-end
}

use std::collections::{BTreeMap, HashSet};

/// The seven Roman numeral letters and their values.
pub fn roman_values() -> BTreeMap<char, u32> {
    // @solution-start
    btreemap! {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
    }
    // @solution-end
}

/// The five lowercase vowels.
pub fn vowels() -> HashSet<char> {
    // @solution-start
    hashset!['a', 'e', 'i', 'o', 'u']
    // @solution-end
}

#[cfg(test)]
mod tests {
    // Note: `HashMap` is deliberately NOT imported here; the macros must not need it.
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn hashmap_with_trailing_comma_and_duplicates() {
        let ages: std::collections::HashMap<&str, i32> = hashmap! { "ann" => 31, "bob" => 27, };
        assert_eq!(ages.len(), 2);
        assert_eq!(ages["ann"], 31);
        assert!(ages.capacity() >= 2);
        let dup: std::collections::HashMap<i32, &str> = hashmap! { 1 => "a", 1 => "b" };
        assert_eq!(dup.len(), 1);
        assert_eq!(dup[&1], "b");
    }

    #[test]
    fn empty_invocations() {
        let map: std::collections::HashMap<String, u8> = hashmap! {};
        let set: HashSet<u8> = hashset![];
        let tree: BTreeMap<u8, u8> = btreemap! {};
        assert!(map.is_empty() && set.is_empty() && tree.is_empty());
    }

    #[test]
    fn sets_and_ordered_maps() {
        let set: HashSet<i32> = hashset![3, 1, 3];
        assert_eq!(set.len(), 2);
        assert!(set.contains(&1) && set.contains(&3));
        let tree: BTreeMap<char, i32> = btreemap! { 'b' => 2, 'a' => 1 };
        assert_eq!(
            tree.into_iter().collect::<Vec<_>>(),
            vec![('a', 1), ('b', 2)]
        );
    }

    #[test]
    fn entries_are_evaluated_once_in_order() {
        let log = RefCell::new(Vec::new());
        let note = |s: &'static str| {
            log.borrow_mut().push(s);
            s
        };
        let map: std::collections::HashMap<&str, &str> =
            hashmap! { note("k1") => note("v1"), note("k2") => note("v2") };
        assert_eq!(map.len(), 2);
        assert_eq!(*log.borrow(), vec!["k1", "v1", "k2", "v2"]);
    }

    #[test]
    fn roman_values_and_vowels() {
        let roman = roman_values();
        assert_eq!(roman.len(), 7);
        assert_eq!(roman.values().sum::<u32>(), 1666);
        assert_eq!(roman.keys().collect::<String>(), "CDILMVX");
        let v = vowels();
        assert_eq!(v.len(), 5);
        assert_eq!("education".chars().filter(|c| v.contains(c)).count(), 5);
    }
}
