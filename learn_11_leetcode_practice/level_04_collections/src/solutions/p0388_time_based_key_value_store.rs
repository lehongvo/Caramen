//! # Problem 0388: Time Based Key-Value Store
//!
//! - **Difficulty:** Medium
//! - **Topics:** nested collections `HashMap<String, BTreeMap<i32, String>>`, `range(..=t).next_back()`, `and_then` on `Option`
//! - **Source:** [LeetCode 981. Time Based Key-Value Store](https://leetcode.com/problems/time-based-key-value-store/)
//!
//! ## Problem
//!
//! Build a key-value store that remembers **every** value a key has had, together with the time it
//! was set. Implement the `TimeMap` struct:
//!
//! - `TimeMap::new() -> Self`: an empty store;
//! - `set(&mut self, key: String, value: String, timestamp: i32)`: record that `key` has `value`
//!   from `timestamp` on;
//! - `get(&self, key: String, timestamp: i32) -> String`: return the value of `key` as it was at
//!   time `timestamp`, that is, the value set with the **largest** timestamp `<= timestamp`. Return
//!   `""` if there is none.
//!
//! On LeetCode the timestamps passed to `set` are strictly increasing; this version also works when
//! they are not (a later `set` with the same key and timestamp replaces the value). `set` takes
//! `&mut self` instead of the template's `&self`, as LeetCode's comment allows.
//!
//! ## Examples
//!
//! ```text
//! let mut tm = TimeMap::new();
//! tm.set("foo", "bar", 1);
//! tm.get("foo", 1) => "bar"
//! tm.get("foo", 3) => "bar"     (latest value at or before time 3)
//! tm.set("foo", "bar2", 4);
//! tm.get("foo", 4) => "bar2"
//! tm.get("foo", 5) => "bar2"
//! tm.get("foo", 0) => ""        (nothing set yet at time 0)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= key.len(), value.len() <= 100`, lowercase letters and digits
//! - `1 <= timestamp <= 10^7`
//! - At most `2 * 10^5` calls in total.
//!
//! ## Hints
//!
//! 1. Group the history by key: `HashMap<String, ...>`.
//! 2. For one key, keep `timestamp -> value` in a `BTreeMap`; "largest timestamp `<= t`" is the last
//!    entry of `range(..=t)`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** a `HashMap` finds the history of a key in `O(1)`, and each history is a `BTreeMap`
//! ordered by timestamp. `get` asks the history for the last entry at or before the requested time.
//!
//! **Complexity:** `O(log h)` per call for a key with `h` stored values, `O(total values)` space.
//! (Because LeetCode's timestamps increase, a `Vec<(i32, String)>` per key with `partition_point`
//! also works and uses less memory.)
//!
//! **Rust concepts:**
//! - `entry(key).or_default().insert(timestamp, value)` creates a key's history on first use; both
//!   `String`s are moved into the store.
//! - `self.store.get(&key)` returns `Option<&BTreeMap<..>>`; `and_then` chains the second lookup,
//!   which also returns an `Option`.
//! - `range(..=timestamp).next_back()` is the "floor" query of an ordered map.
//! - `map_or_else(String::new, |(_, v)| v.clone())` returns `""` or a copy of the stored value.
// @solution-end

use std::collections::{BTreeMap, HashMap};

/// Values of each key over time.
pub struct TimeMap {
    // @solution-start remove
    store: HashMap<String, BTreeMap<i32, String>>,
    // @solution-end
}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl TimeMap {
    pub fn new() -> Self {
        // @solution-start
        Self {
            store: HashMap::new(),
        }
        // @solution-end
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        // @solution-start
        self.store.entry(key).or_default().insert(timestamp, value);
        // @solution-end
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        // @solution-start
        self.store
            .get(&key)
            .and_then(|history| history.range(..=timestamp).next_back())
            .map_or_else(String::new, |(_, value)| value.clone())
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(x: &str) -> String {
        x.to_string()
    }

    #[test]
    fn example_sequence() {
        let mut tm = TimeMap::new();
        tm.set(s("foo"), s("bar"), 1);
        assert_eq!(tm.get(s("foo"), 1), "bar");
        assert_eq!(tm.get(s("foo"), 3), "bar");
        tm.set(s("foo"), s("bar2"), 4);
        assert_eq!(tm.get(s("foo"), 4), "bar2");
        assert_eq!(tm.get(s("foo"), 5), "bar2");
        assert_eq!(tm.get(s("foo"), 0), "");
    }

    #[test]
    fn unknown_key_and_independent_keys() {
        let mut tm = TimeMap::new();
        assert_eq!(tm.get(s("nope"), 10), "");
        tm.set(s("a"), s("1"), 5);
        tm.set(s("b"), s("2"), 6);
        assert_eq!(tm.get(s("a"), 6), "1");
        assert_eq!(tm.get(s("b"), 5), "");
    }

    #[test]
    fn floor_between_timestamps() {
        let mut tm = TimeMap::new();
        for (t, v) in [(10, "x"), (20, "y"), (30, "z")] {
            tm.set(s("k"), s(v), t);
        }
        assert_eq!(tm.get(s("k"), 9), "");
        assert_eq!(tm.get(s("k"), 19), "x");
        assert_eq!(tm.get(s("k"), 20), "y");
        assert_eq!(tm.get(s("k"), 10_000_000), "z");
    }

    #[test]
    fn many_values() {
        let mut tm = TimeMap::new();
        for t in 1..=100_000 {
            tm.set(format!("key{}", t % 3), format!("v{t}"), t * 10);
        }
        assert_eq!(tm.get(s("key1"), 15), "v1");
        assert_eq!(tm.get(s("key0"), 25), "");
        assert_eq!(tm.get(s("key0"), 1_000_000), "v99999");
        assert_eq!(tm.get(s("key2"), 1_000_000), "v99998");
    }
}
