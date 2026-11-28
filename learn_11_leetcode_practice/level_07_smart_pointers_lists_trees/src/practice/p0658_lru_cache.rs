// ==================================================================
// PRACTICE 0658: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0658_lru_cache.md
// Run the tests:          cargo test practice::p0658
// ==================================================================

//! # Problem 0658: LRU Cache
//!
//! - **Difficulty:** Medium
//! - **Topics:** `HashMap<i32, Rc<RefCell<Node>>>`, doubly linked list with `Weak` back links, unlinking and re-linking shared nodes, iterative `Drop`
//! - **Source:** [LeetCode 146. LRU Cache](https://leetcode.com/problems/lru-cache/)
//!
//! ## Problem
//!
//! Design a key-value cache with a fixed `capacity` that evicts the **least recently used** entry
//! when it is full:
//!
//! - `LRUCache::new(capacity: i32) -> Self`;
//! - `get(&mut self, key: i32) -> i32`: the value for `key`, or `-1` if it is absent. A successful
//!   `get` marks the key as the most recently used;
//! - `put(&mut self, key: i32, value: i32)`: insert or update the key and mark it as the most
//!   recently used. If inserting a **new** key makes the cache exceed `capacity`, first remove the
//!   least recently used key.
//!
//! Both operations must run in `O(1)` average time. (LeetCode's template shows `&self`; its
//! comment says to switch to `&mut self` when needed, as done here.)
//!
//! ## Examples
//!
//! ```text
//! let mut cache = LRUCache::new(2);
//! cache.put(1, 1);            // {1=1}
//! cache.put(2, 2);            // {1=1, 2=2}
//! cache.get(1)    -> 1        // 1 is now the most recent
//! cache.put(3, 3);            // evicts key 2
//! cache.get(2)    -> -1
//! cache.put(4, 4);            // evicts key 1
//! cache.get(1)    -> -1
//! cache.get(3)    -> 3
//! cache.get(4)    -> 4
//! ```
//!
//! ## Constraints
//!
//! - `1 <= capacity <= 3000`, `0 <= key <= 10^4`, `0 <= value <= 10^5`
//! - At most `2 * 10^5` calls in total.
//!
//! ## Hints
//!
//! 1. A `HashMap` finds a key in `O(1)`; a doubly linked list ordered by recency moves an entry to
//!    the front and removes the back in `O(1)`. Use both, pointing at the same nodes.
//! 2. Nodes are shared by the map and the list, so they are `Rc<RefCell<Node>>`. Forward links are
//!    `Rc`; backward links are `Weak` to avoid reference cycles (problem 0626).
//! 3. Write two helpers: `detach(node)` (unlink it from wherever it is) and `push_front(node)`.
//!    `get` and `put` are then a few lines each.

pub struct LRUCache {}

impl LRUCache {
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

impl Drop for LRUCache {
    fn drop(&mut self) {
        // TODO (optional): unlink the nodes with a loop so a long list is not dropped recursively.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn example_sequence() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn updating_a_key_refreshes_it() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(1, 10);
        cache.put(3, 3);
        assert_eq!(cache.get(1), 10);
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(3), 3);
    }

    #[test]
    fn capacity_one() {
        let mut cache = LRUCache::new(1);
        assert_eq!(cache.get(5), -1);
        cache.put(5, 50);
        assert_eq!(cache.get(5), 50);
        cache.put(6, 60);
        assert_eq!(cache.get(5), -1);
        assert_eq!(cache.get(6), 60);
        cache.put(6, 61);
        assert_eq!(cache.get(6), 61);
    }

    #[test]
    fn matches_a_simple_model() {
        // Model: a VecDeque of (key, value), front = most recent. O(n) per call, but obviously right.
        let capacity = 50;
        let mut cache = LRUCache::new(capacity as i32);
        let mut model: VecDeque<(i32, i32)> = VecDeque::new();
        for step in 0..20_000 {
            let key = (step * 7_919) % 120;
            if step % 3 == 0 {
                let expected = match model.iter().position(|&(k, _)| k == key) {
                    Some(i) => {
                        let entry = model.remove(i).unwrap();
                        model.push_front(entry);
                        entry.1
                    }
                    None => -1,
                };
                assert_eq!(cache.get(key), expected, "step {step}");
            } else {
                if let Some(i) = model.iter().position(|&(k, _)| k == key) {
                    model.remove(i);
                } else if model.len() == capacity {
                    model.pop_back();
                }
                model.push_front((key, step));
                cache.put(key, step);
            }
        }
    }

    #[test]
    fn large_capacity_drops_cleanly() {
        let mut cache = LRUCache::new(3_000);
        for key in 0..10_000 {
            cache.put(key, key * 2);
        }
        assert_eq!(cache.get(6_999), -1);
        assert_eq!(cache.get(7_000), 14_000);
        drop(cache);
    }
}
