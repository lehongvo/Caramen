// ==================================================================
// PRACTICE 0338: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0338_design_hashset.md
// Run the tests:          cargo test practice::p0338
// ==================================================================

//! # Problem 0338: Design HashSet
//!
//! - **Difficulty:** Easy
//! - **Topics:** hashing by hand: buckets `Vec<Vec<i32>>`, a modulo hash, `contains`, `retain`
//! - **Source:** [LeetCode 705. Design HashSet](https://leetcode.com/problems/design-hashset/)
//!
//! ## Problem
//!
//! Build your own set of integers **without** using `HashSet`, `HashMap`, `BTreeSet` or any other
//! hash table from the standard library. Implement the `MyHashSet` struct:
//!
//! - `MyHashSet::new() -> Self` creates an empty set;
//! - `add(&mut self, key: i32)` inserts `key` (adding a key that is already present does nothing);
//! - `remove(&mut self, key: i32)` deletes `key` if it is present (otherwise does nothing);
//! - `contains(&self, key: i32) -> bool` tells whether `key` is in the set.
//!
//! LeetCode's template declares every method with `&self`; its comment allows changing it to
//! `&mut self` for methods that modify the object, which is what this version does.
//!
//! ## Examples
//!
//! ```text
//! let mut set = MyHashSet::new();
//! set.add(1);
//! set.add(2);
//! set.contains(1) => true
//! set.contains(3) => false
//! set.add(2);                 (already present)
//! set.contains(2) => true
//! set.remove(2);
//! set.contains(2) => false
//! ```
//!
//! ## Constraints
//!
//! - `0 <= key <= 10^6`
//! - At most `10^4` calls in total.
//!
//! ## Hints
//!
//! 1. A hash table is an array of **buckets**; a key goes into bucket `hash(key) % number_of_buckets`.
//! 2. Different keys can land in the same bucket, so each bucket is a small `Vec<i32>`.
//! 3. All three operations only look at the one bucket where the key belongs.

/// A set of integers implemented with buckets.
pub struct MyHashSet {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl MyHashSet {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn add(&mut self, key: i32) {
        todo!("Write your solution here")
    }

    pub fn remove(&mut self, key: i32) {
        todo!("Write your solution here")
    }

    pub fn contains(&self, key: i32) -> bool {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(2);
        assert!(set.contains(1));
        assert!(!set.contains(3));
        set.add(2);
        assert!(set.contains(2));
        set.remove(2);
        assert!(!set.contains(2));
    }

    #[test]
    fn remove_missing_key_is_harmless() {
        let mut set = MyHashSet::new();
        set.remove(5);
        assert!(!set.contains(5));
        set.add(0);
        set.remove(5);
        assert!(set.contains(0));
    }

    #[test]
    fn colliding_keys_are_kept_apart() {
        let mut set = MyHashSet::new();
        // 769 * k all land in the same bucket.
        for k in 0..10 {
            set.add(769 * k);
        }
        set.remove(769 * 3);
        for k in 0..10 {
            assert_eq!(set.contains(769 * k), k != 3);
        }
        assert!(!set.contains(1));
    }

    #[test]
    fn many_operations() {
        let mut set = MyHashSet::new();
        for k in (0..1_000_000).step_by(250) {
            set.add(k);
        }
        for k in (0..1_000_000).step_by(500) {
            set.remove(k);
        }
        assert!(set.contains(250));
        assert!(!set.contains(500));
        assert!(set.contains(999_750));
        assert!(!set.contains(1_000_000));
        assert!(!set.contains(251));
    }
}
