//! # Problem 0888: Read-Mostly Config Store
//!
//! - **Difficulty:** Medium
//! - **Topics:** `std::sync::RwLock` (many readers or one writer), `read()` / `write()` guards,
//!   recovering from poisoning, running a closure under a read lock
//! - **Source:** Original · [`RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html)
//!
//! ## Problem
//!
//! A configuration is read constantly and changed rarely. With a `Mutex`, readers would block each
//! other for no reason. A `RwLock` allows **any number of readers at the same time**, or **one
//! writer** alone.
//!
//! Implement `ConfigStore`, a string-to-string map shared by many threads through `&self`:
//!
//! - `new() -> ConfigStore` (also `Default`): empty;
//! - `get(&self, key: &str) -> Option<String>`: a copy of the value (read lock);
//! - `set(&self, key: &str, value: &str) -> Option<String>`: insert or replace, returning the old
//!   value (write lock);
//! - `remove(&self, key: &str) -> Option<String>`: remove, returning the old value (write lock);
//! - `len(&self) -> usize` and `is_empty(&self) -> bool` (read lock);
//! - `snapshot(&self) -> BTreeMap<String, String>`: a sorted copy of everything (read lock);
//! - `with_read<R>(&self, f: impl FnOnce(&HashMap<String, String>) -> R) -> R`: run `f` while
//!   holding a read lock, and return its result.
//!
//! Several threads must be able to be inside `with_read` **at the same time**. If a thread panicked
//! while holding the lock, keep working with the data anyway.
//!
//! ## Examples
//!
//! ```text
//! let store = ConfigStore::new();
//! store.set("mode", "fast") -> None
//! store.set("mode", "safe") -> Some("fast")
//! store.get("mode")         -> Some("safe")
//! store.remove("mode")      -> Some("safe")
//! store.is_empty()          -> true
//! ```
//!
//! ## Constraints
//!
//! - Up to 16 threads; keys and values are short strings.
//!
//! ## Hints
//!
//! 1. `self.entries.read()` returns `LockResult<RwLockReadGuard<..>>`; `write()` returns a write
//!    guard. Recover from poisoning with `unwrap_or_else(PoisonError::into_inner)`.
//! 2. Return owned `String`s from `get`: a reference into the map cannot outlive the guard.
//! 3. Keep each guard only as long as needed; never call `set` while holding a read guard from the
//!    same thread (that would deadlock).
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** wrap a `HashMap<String, String>` in a `RwLock`. Two private helpers return a read or a
//! write guard with poison recovery; every public method takes the right kind of guard for the
//! shortest time and clones what it returns.
//!
//! **Rust concepts:**
//! - `RwLock<T>` is `Sync` when `T: Send + Sync`, because several readers may access `&T`
//!   concurrently (a `Mutex` only needs `T: Send`).
//! - Guards implement `Deref`/`DerefMut`, so `guard.get(key)` calls `HashMap::get` directly.
//! - The order in which waiting readers and writers get the lock depends on the operating system;
//!   code must not rely on it.
// @solution-end

use std::collections::{BTreeMap, HashMap};
use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// A thread-safe configuration map optimized for many readers.
pub struct ConfigStore {
    // @solution-start remove
    entries: RwLock<HashMap<String, String>>,
    // @solution-end
}

impl ConfigStore {
    pub fn new() -> Self {
        // @solution-start
        Self {
            entries: RwLock::new(HashMap::new()),
        }
        // @solution-end
    }

    // @solution-start remove
    fn read(&self) -> RwLockReadGuard<'_, HashMap<String, String>> {
        self.entries.read().unwrap_or_else(PoisonError::into_inner)
    }

    fn write(&self) -> RwLockWriteGuard<'_, HashMap<String, String>> {
        self.entries.write().unwrap_or_else(PoisonError::into_inner)
    }
    // @solution-end

    pub fn get(&self, key: &str) -> Option<String> {
        // @solution-start
        self.read().get(key).cloned()
        // @solution-end
    }

    pub fn set(&self, key: &str, value: &str) -> Option<String> {
        // @solution-start
        self.write().insert(key.to_string(), value.to_string())
        // @solution-end
    }

    pub fn remove(&self, key: &str) -> Option<String> {
        // @solution-start
        self.write().remove(key)
        // @solution-end
    }

    pub fn len(&self) -> usize {
        // @solution-start
        self.read().len()
        // @solution-end
    }

    pub fn is_empty(&self) -> bool {
        // @solution-start
        self.read().is_empty()
        // @solution-end
    }

    pub fn snapshot(&self) -> BTreeMap<String, String> {
        // @solution-start
        self.read()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
        // @solution-end
    }

    pub fn with_read<R>(&self, f: impl FnOnce(&HashMap<String, String>) -> R) -> R {
        // @solution-start
        f(&self.read())
        // @solution-end
    }
}

impl Default for ConfigStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Barrier;
    use std::thread;

    #[test]
    fn example_set_get_remove() {
        let store = ConfigStore::new();
        assert_eq!(store.set("mode", "fast"), None);
        assert_eq!(store.set("mode", "safe"), Some("fast".to_string()));
        assert_eq!(store.get("mode"), Some("safe".to_string()));
        assert_eq!(store.remove("mode"), Some("safe".to_string()));
        assert!(store.is_empty());
    }

    #[test]
    fn missing_keys_and_snapshot_order() {
        let store = ConfigStore::default();
        assert_eq!(store.get("nope"), None);
        assert_eq!(store.remove("nope"), None);
        store.set("b", "2");
        store.set("a", "1");
        let snap: Vec<(String, String)> = store.snapshot().into_iter().collect();
        assert_eq!(
            snap,
            vec![("a".into(), "1".into()), ("b".into(), "2".into())]
        );
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn readers_share_the_lock() {
        // Four readers wait for each other *inside* `with_read`. With an exclusive lock this test
        // would hang; with a read lock all four get in together.
        let store = ConfigStore::new();
        store.set("k", "v");
        let barrier = Barrier::new(4);
        thread::scope(|s| {
            for _ in 0..4 {
                s.spawn(|| {
                    let len = store.with_read(|map| {
                        barrier.wait();
                        map.len()
                    });
                    assert_eq!(len, 1);
                });
            }
        });
    }

    #[test]
    fn concurrent_writers_and_readers() {
        let store = ConfigStore::new();
        thread::scope(|s| {
            for w in 0..8 {
                let store = &store;
                s.spawn(move || {
                    for i in 0..200 {
                        store.set(&format!("w{w}-k{i}"), &format!("{}", w * 1000 + i));
                    }
                });
            }
            for _ in 0..8 {
                s.spawn(|| {
                    for _ in 0..200 {
                        if let Some(v) = store.get("w3-k10") {
                            assert_eq!(v, "3010");
                        }
                        let _ = store.len();
                    }
                });
            }
        });
        assert_eq!(store.len(), 1600);
        assert_eq!(store.get("w7-k199"), Some("7199".to_string()));
    }
}
