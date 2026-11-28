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
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** "separate chaining". Keep a fixed number of buckets (a prime such as `769` spreads
//! keys well) and use `key % BUCKETS` as the hash. Each operation computes the bucket index and
//! works on that small vector only: `add` pushes if absent, `remove` uses `retain`, `contains` uses
//! `contains`.
//!
//! **Complexity:** `O(n / BUCKETS)` average time per call (the bucket length), `O(BUCKETS + n)`
//! space.
//!
//! **Rust concepts:**
//! - `vec![Vec::new(); BUCKETS]` creates independent empty buckets (each `Vec` is cloned).
//! - A private helper method `bucket(&self, key) -> usize` keeps the hashing in one place.
//! - `rem_euclid` would make the hash safe for negative keys too; here keys are non-negative.
//! - The standard `HashSet` works the same way in spirit, but uses a strong hash function
//!   (SipHash by default), grows its bucket array as it fills up, and uses open addressing.
//! - LeetCode's template only has `new()`, which makes Clippy suggest a `Default` impl; the lint
//!   is allowed on this `impl` block to keep LeetCode's shape.
// @solution-end

// @solution-start remove
const BUCKETS: usize = 769;
// @solution-end

/// A set of integers implemented with buckets.
pub struct MyHashSet {
    // @solution-start remove
    buckets: Vec<Vec<i32>>,
    // @solution-end
}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl MyHashSet {
    pub fn new() -> Self {
        // @solution-start
        Self {
            buckets: vec![Vec::new(); BUCKETS],
        }
        // @solution-end
    }

    // @solution-start remove
    fn bucket(&self, key: i32) -> usize {
        key.rem_euclid(BUCKETS as i32) as usize
    }
    // @solution-end

    pub fn add(&mut self, key: i32) {
        // @solution-start
        let b = self.bucket(key);
        if !self.buckets[b].contains(&key) {
            self.buckets[b].push(key);
        }
        // @solution-end
    }

    pub fn remove(&mut self, key: i32) {
        // @solution-start
        let b = self.bucket(key);
        self.buckets[b].retain(|&k| k != key);
        // @solution-end
    }

    pub fn contains(&self, key: i32) -> bool {
        // @solution-start
        self.buckets[self.bucket(key)].contains(&key)
        // @solution-end
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
