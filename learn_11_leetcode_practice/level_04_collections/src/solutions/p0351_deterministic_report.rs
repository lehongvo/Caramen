//! # Problem 0351: A Deterministic Report from a HashMap
//!
//! - **Difficulty:** Easy
//! - **Topics:** unspecified `HashMap` iteration order, collect + sort, `sort_by` with `then_with`, collecting into a `BTreeMap`
//! - **Source:** Original · [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) · [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
//!
//! ## Problem
//!
//! Iterating over a `std::collections::HashMap` visits its entries in an **unspecified** order.
//! In practice the order depends on a random seed chosen when the map is created, so it can change
//! from one run of the program to the next. Any output that must be stable (reports, tests, files)
//! has to impose an order explicitly.
//!
//! Given `scores: &HashMap<String, u32>` (player name -> score), implement:
//!
//! 1. `by_name(scores) -> Vec<(String, u32)>`: all entries sorted by name.
//! 2. `ranking(scores) -> Vec<String>`: the names sorted by score, **highest first**; equal scores
//!    are ordered by name.
//! 3. `to_sorted_map(scores) -> BTreeMap<String, u32>`: the same entries in a `BTreeMap`, which
//!    always iterates in key order.
//! 4. `report(scores) -> String`: one line `"<name>: <score>"` per player, sorted by name, lines
//!    separated by `'\n'` (no trailing newline; an empty map gives `""`).
//!
//! ## Examples
//!
//! ```text
//! scores = {"mia": 7, "ali": 9, "zoe": 7}
//! by_name       => [("ali", 9), ("mia", 7), ("zoe", 7)]
//! ranking       => ["ali", "mia", "zoe"]
//! to_sorted_map => {"ali": 9, "mia": 7, "zoe": 7}   (iterates in this order)
//! report        => "ali: 9\nmia: 7\nzoe: 7"
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` players; names are unique (they are map keys).
//!
//! ## Hints
//!
//! 1. `scores.iter()` yields `(&String, &u32)`; clone/copy them into a `Vec` and sort the `Vec`.
//! 2. Tuples sort by their first element first, so a `Vec<(String, u32)>` sorts by name.
//! 3. `collect()` can build a `BTreeMap` directly from an iterator of pairs.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** never rely on the map's own order. Copy what you need into a `Vec` and sort it with
//! the order the output requires, or move the data into a `BTreeMap`, whose iteration order is the
//! key order by definition.
//!
//! **Rust concepts:**
//! - `HashMap::new()` uses `RandomState`, which seeds the hasher randomly per map to protect against
//!   hash-flooding attacks. That randomness is why iteration order is not reproducible.
//! - Names are unique, so `sort_unstable` is fine in `by_name` and `ranking`: no two elements are
//!   equal under the comparison.
//! - `b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))` sorts scores descending and names ascending.
//! - `format!("{name}: {score}")` + `join("\n")` builds the report; a `BTreeMap` would give the same
//!   order without an explicit sort.
// @solution-end

use std::collections::{BTreeMap, HashMap};

/// All entries sorted by name.
pub fn by_name(scores: &HashMap<String, u32>) -> Vec<(String, u32)> {
    // @solution-start
    let mut entries: Vec<(String, u32)> = scores
        .iter()
        .map(|(name, &score)| (name.clone(), score))
        .collect();
    entries.sort_unstable();
    entries
    // @solution-end
}

/// Names by score (highest first), then by name.
pub fn ranking(scores: &HashMap<String, u32>) -> Vec<String> {
    // @solution-start
    let mut entries: Vec<(&String, &u32)> = scores.iter().collect();
    entries.sort_unstable_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    entries.into_iter().map(|(name, _)| name.clone()).collect()
    // @solution-end
}

/// The same entries in a `BTreeMap`.
pub fn to_sorted_map(scores: &HashMap<String, u32>) -> BTreeMap<String, u32> {
    // @solution-start
    scores
        .iter()
        .map(|(name, &score)| (name.clone(), score))
        .collect()
    // @solution-end
}

/// One `"name: score"` line per player, sorted by name.
pub fn report(scores: &HashMap<String, u32>) -> String {
    // @solution-start
    by_name(scores)
        .iter()
        .map(|(name, score)| format!("{name}: {score}"))
        .collect::<Vec<String>>()
        .join("\n")
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scores(pairs: &[(&str, u32)]) -> HashMap<String, u32> {
        pairs.iter().map(|&(n, s)| (n.to_string(), s)).collect()
    }

    fn example() -> HashMap<String, u32> {
        scores(&[("mia", 7), ("ali", 9), ("zoe", 7)])
    }

    #[test]
    fn by_name_example() {
        assert_eq!(
            by_name(&example()),
            vec![
                ("ali".to_string(), 9),
                ("mia".to_string(), 7),
                ("zoe".to_string(), 7)
            ]
        );
    }

    #[test]
    fn ranking_example() {
        assert_eq!(ranking(&example()), vec!["ali", "mia", "zoe"]);
        assert_eq!(
            ranking(&scores(&[("b", 1), ("a", 1), ("c", 2)])),
            vec!["c", "a", "b"]
        );
    }

    #[test]
    fn sorted_map_iterates_in_key_order() {
        let map = to_sorted_map(&example());
        let keys: Vec<&str> = map.keys().map(String::as_str).collect();
        assert_eq!(keys, vec!["ali", "mia", "zoe"]);
        assert_eq!(map["ali"], 9);
    }

    #[test]
    fn report_example_and_empty() {
        assert_eq!(report(&example()), "ali: 9\nmia: 7\nzoe: 7");
        assert_eq!(report(&HashMap::new()), "");
    }

    #[test]
    fn same_output_for_maps_built_in_different_orders() {
        let forward: HashMap<String, u32> =
            (0..1000).map(|i| (format!("p{i:03}"), i % 50)).collect();
        let backward: HashMap<String, u32> = (0..1000)
            .rev()
            .map(|i| (format!("p{i:03}"), i % 50))
            .collect();
        assert_eq!(report(&forward), report(&backward));
        assert_eq!(ranking(&forward), ranking(&backward));
        assert_eq!(ranking(&forward)[0], "p049");
        assert_eq!(ranking(&forward)[999], "p950");
    }
}
