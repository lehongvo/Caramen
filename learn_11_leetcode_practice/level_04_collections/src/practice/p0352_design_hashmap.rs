// ==================================================================
// PRACTICE 0352: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0352_design_hashmap.md
// Run the tests:          cargo test practice::p0352
// ==================================================================

//! # Problem 0352: Design HashMap
//!
//! - **Difficulty:** Easy
//! - **Topics:** buckets `Vec<Vec<(i32, i32)>>`, `iter_mut().find`, `iter().find`, `retain`
//! - **Source:** [LeetCode 706. Design HashMap](https://leetcode.com/problems/design-hashmap/)
//!
//! ## Problem
//!
//! Build your own map from integer keys to integer values **without** using `HashMap`, `BTreeMap`
//! or any other built-in map. Implement the `MyHashMap` struct:
//!
//! - `MyHashMap::new() -> Self` creates an empty map;
//! - `put(&mut self, key: i32, value: i32)` stores `value` for `key`, replacing the old value if
//!   the key is already present;
//! - `get(&self, key: i32) -> i32` returns the value for `key`, or `-1` if the key is absent;
//! - `remove(&mut self, key: i32)` deletes the key and its value, if present.
//!
//! As in "Design HashSet", the mutating methods take `&mut self` (LeetCode's template comment
//! allows replacing `&self`).
//!
//! ## Examples
//!
//! ```text
//! let mut map = MyHashMap::new();
//! map.put(1, 1);
//! map.put(2, 2);
//! map.get(1) => 1
//! map.get(3) => -1
//! map.put(2, 1);       (update)
//! map.get(2) => 1
//! map.remove(2);
//! map.get(2) => -1
//! ```
//!
//! ## Constraints
//!
//! - `0 <= key, value <= 10^6`
//! - At most `10^4` calls in total.
//!
//! ## Hints
//!
//! 1. Reuse the bucket idea of the set, but store `(key, value)` pairs in each bucket.
//! 2. `bucket.iter_mut().find(|(k, _)| *k == key)` returns a mutable reference to the pair, so
//!    `put` can update the value in place.

/// A map from integers to integers implemented with buckets.
pub struct MyHashMap {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl MyHashMap {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn put(&mut self, key: i32, value: i32) {
        todo!("Write your solution here")
    }

    pub fn get(&self, key: i32) -> i32 {
        todo!("Write your solution here")
    }

    pub fn remove(&mut self, key: i32) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        map.put(2, 2);
        assert_eq!(map.get(1), 1);
        assert_eq!(map.get(3), -1);
        map.put(2, 1);
        assert_eq!(map.get(2), 1);
        map.remove(2);
        assert_eq!(map.get(2), -1);
    }

    #[test]
    fn zero_key_and_zero_value() {
        let mut map = MyHashMap::new();
        assert_eq!(map.get(0), -1);
        map.put(0, 0);
        assert_eq!(map.get(0), 0);
        map.remove(0);
        map.remove(0);
        assert_eq!(map.get(0), -1);
    }

    #[test]
    fn colliding_keys() {
        let mut map = MyHashMap::new();
        for k in 0..10 {
            map.put(769 * k, k);
        }
        map.put(769 * 4, 400);
        map.remove(769 * 7);
        assert_eq!(map.get(769 * 4), 400);
        assert_eq!(map.get(769 * 7), -1);
        assert_eq!(map.get(769 * 9), 9);
    }

    #[test]
    fn many_keys() {
        let mut map = MyHashMap::new();
        for k in 0..5_000 {
            map.put(k * 200, k);
        }
        for k in 0..5_000 {
            if k % 2 == 0 {
                map.remove(k * 200);
            }
        }
        assert_eq!(map.get(999_800), 4_999);
        assert_eq!(map.get(999_600), -1);
        assert_eq!(map.get(1), -1);
    }
}
