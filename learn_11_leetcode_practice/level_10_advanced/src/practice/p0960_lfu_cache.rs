// ==================================================================
// PRACTICE 0960: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0960_lfu_cache.md
// Run the tests:          cargo test practice::p0960
// ==================================================================

//! # Problem 0960: LFU Cache
//!
//! - **Difficulty:** Hard
//! - **Topics:** `HashMap` + `BTreeSet<(freq, tick, key)>` as an ordered eviction index, `pop_first`, logical clocks, keeping two structures in sync
//! - **Source:** [LeetCode 460. LFU Cache](https://leetcode.com/problems/lfu-cache/)
//!
//! ## Problem
//!
//! A **least frequently used** cache of fixed capacity. Each key has a use counter, incremented by
//! every successful `get` and every `put` of that key (a new key starts at 1). When the cache is full
//! and a new key must be inserted, evict the key with the **smallest** counter; among those, evict
//! the **least recently used** one.
//!
//! Implement the `LFUCache` struct:
//!
//! - `LFUCache::new(capacity: i32) -> Self`.
//! - `get(&mut self, key: i32) -> i32`: the value of `key`, or `-1` if it is not cached.
//! - `put(&mut self, key: i32, value: i32)`: updates the value of an existing key, or inserts a new
//!   key (evicting first if the cache is full).
//!
//! The intended complexity is `O(1)` average per operation; `O(log n)` is accepted here.
//!
//! ## Examples
//!
//! ```text
//! let mut c = LFUCache::new(2);
//! c.put(1, 1); c.put(2, 2);
//! c.get(1)     => 1         (counters: 1 -> 2, 2 -> 1)
//! c.put(3, 3)              (evicts 2, the least frequently used)
//! c.get(2)     => -1
//! c.get(3)     => 3         (counters: 1 -> 2, 3 -> 2)
//! c.put(4, 4)              (1 and 3 tie; 1 was used less recently: evict 1)
//! c.get(1)     => -1
//! c.get(3)     => 3
//! c.get(4)     => 4
//! ```
//!
//! ## Constraints
//!
//! - `1 <= capacity <= 10^4`, `0 <= key <= 10^5`, `0 <= value <= 10^9`
//! - At most `2 * 10^5` calls in total.
//!
//! ## Hints
//!
//! 1. Give every use a timestamp from an increasing counter (`tick`).
//! 2. Keep `entries: HashMap<key, (value, freq, tick)>` and an ordered set of `(freq, tick, key)`:
//!    its first element is exactly the key to evict.
//! 3. On every use, remove the old `(freq, tick, key)` from the set and insert the updated one.

use std::collections::{BTreeSet, HashMap};

pub struct LFUCache {}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        todo!("Write your solution here")
    }

    pub fn get(&mut self, key: i32) -> i32 {
        todo!("Write your solution here")
    }

    pub fn put(&mut self, key: i32, value: i32) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut c = LFUCache::new(2);
        c.put(1, 1);
        c.put(2, 2);
        assert_eq!(c.get(1), 1);
        c.put(3, 3);
        assert_eq!(c.get(2), -1);
        assert_eq!(c.get(3), 3);
        c.put(4, 4);
        assert_eq!(c.get(1), -1);
        assert_eq!(c.get(3), 3);
        assert_eq!(c.get(4), 4);
    }

    #[test]
    fn updating_a_key_counts_as_a_use() {
        let mut c = LFUCache::new(2);
        c.put(1, 10);
        c.put(2, 20);
        c.put(1, 11); // key 1: frequency 2
        c.put(3, 30); // evicts 2
        assert_eq!((c.get(1), c.get(2), c.get(3)), (11, -1, 30));
    }

    #[test]
    fn capacity_one() {
        let mut c = LFUCache::new(1);
        c.put(1, 1);
        assert_eq!(c.get(1), 1);
        c.put(2, 2);
        assert_eq!((c.get(1), c.get(2)), (-1, 2));
    }

    /// A slow but obviously correct model of the cache.
    struct Model {
        capacity: usize,
        tick: u64,
        items: Vec<(i32, i32, u32, u64)>, // key, value, freq, last use
    }

    impl Model {
        fn get(&mut self, key: i32) -> i32 {
            self.tick += 1;
            let tick = self.tick;
            match self.items.iter_mut().find(|item| item.0 == key) {
                Some(item) => {
                    item.2 += 1;
                    item.3 = tick;
                    item.1
                }
                None => -1,
            }
        }

        fn put(&mut self, key: i32, value: i32) {
            self.tick += 1;
            let tick = self.tick;
            if let Some(item) = self.items.iter_mut().find(|item| item.0 == key) {
                *item = (key, value, item.2 + 1, tick);
                return;
            }
            if self.items.len() == self.capacity {
                let victim = (0..self.items.len())
                    .min_by_key(|&i| (self.items[i].2, self.items[i].3))
                    .unwrap();
                self.items.remove(victim);
            }
            self.items.push((key, value, 1, tick));
        }
    }

    #[test]
    fn matches_a_slow_model() {
        let mut c = LFUCache::new(5);
        let mut model = Model {
            capacity: 5,
            tick: 0,
            items: Vec::new(),
        };
        for step in 0..5_000_i32 {
            let key = (step * 7) % 13;
            if step % 3 == 0 {
                c.put(key, step);
                model.put(key, step);
            } else {
                assert_eq!(c.get(key), model.get(key), "step {step}");
            }
        }
    }
}
