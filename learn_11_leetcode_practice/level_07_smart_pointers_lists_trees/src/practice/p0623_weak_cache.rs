// ==================================================================
// PRACTICE 0623: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0623_weak_cache.md
// Run the tests:          cargo test practice::p0623
// ==================================================================

//! # Problem 0623: Weak Cache
//!
//! - **Difficulty:** Medium
//! - **Topics:** `Weak<T>`, `Rc::downgrade`, `Weak::upgrade`, `Rc::weak_count`, `HashMap::retain`
//! - **Source:** Original · [The Rust Book 15.6](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html) · [`std::rc::Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html)
//!
//! ## Problem
//!
//! A game loads assets (textures, sounds) by name. Loading is slow, so if an asset is already in
//! use somewhere, asking for it again must return the **same** shared value. But the cache must not
//! keep assets alive by itself: when the last user drops an asset, its memory is freed, and the
//! next request loads it again.
//!
//! A `Weak<T>` is exactly that: a non-owning handle that does not count as an owner. `upgrade()`
//! returns `Some(Rc<T>)` while at least one strong owner exists, and `None` afterwards.
//!
//! ```text
//! #[derive(Debug, PartialEq)]
//! pub struct Asset {
//!     pub name: String,
//!     pub data: String,          // "<data of NAME>"
//! }
//! ```
//!
//! Implement `AssetCache`:
//!
//! - `AssetCache::new() -> AssetCache`;
//! - `get(&mut self, name: &str) -> Rc<Asset>`: return the live asset if some user still holds it;
//!   otherwise "load" a new `Asset { name, data: format!("<data of {name}>") }`, remember it
//!   **weakly**, and return it;
//! - `loads(&self) -> usize`: how many assets have been loaded so far;
//! - `live_count(&self) -> usize`: how many remembered assets are still alive;
//! - `purge(&mut self) -> usize`: forget the entries whose asset is gone and return how many were
//!   removed.
//!
//! ## Examples
//!
//! ```text
//! let mut cache = AssetCache::new();
//! let a = cache.get("grass");
//! let b = cache.get("grass");     // Rc::ptr_eq(&a, &b), cache.loads() == 1
//! Rc::strong_count(&a) == 2, Rc::weak_count(&a) == 1
//! drop(a); drop(b);               // the asset is freed
//! cache.live_count() == 0
//! let c = cache.get("grass");     // loaded again: cache.loads() == 2
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` calls.
//!
//! ## Hints
//!
//! 1. Store `HashMap<String, Weak<Asset>>` plus a load counter.
//! 2. `self.entries.get(name).and_then(Weak::upgrade)` is `Some(rc)` exactly when the asset is
//!    still alive.
//! 3. `Rc::downgrade(&rc)` creates the `Weak` to store.

use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug, PartialEq)]
pub struct Asset {
    pub name: String,
    pub data: String,
}

#[derive(Default)]
pub struct AssetCache {}

impl AssetCache {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn get(&mut self, name: &str) -> Rc<Asset> {
        todo!("Write your solution here")
    }

    pub fn loads(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn live_count(&self) -> usize {
        todo!("Write your solution here")
    }

    pub fn purge(&mut self) -> usize {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_assets_are_shared() {
        let mut cache = AssetCache::new();
        let a = cache.get("grass");
        let b = cache.get("grass");
        assert!(Rc::ptr_eq(&a, &b));
        assert_eq!(cache.loads(), 1);
        assert_eq!(Rc::strong_count(&a), 2);
        assert_eq!(Rc::weak_count(&a), 1);
        assert_eq!(a.data, "<data of grass>");
    }

    #[test]
    fn dropped_assets_are_loaded_again() {
        let mut cache = AssetCache::new();
        let a = cache.get("grass");
        drop(a);
        assert_eq!(cache.live_count(), 0);
        let c = cache.get("grass");
        assert_eq!(cache.loads(), 2);
        assert_eq!(c.name, "grass");
        assert_eq!(cache.live_count(), 1);
    }

    #[test]
    fn the_cache_does_not_keep_assets_alive() {
        let mut cache = AssetCache::new();
        let sound = cache.get("boom");
        assert_eq!(Rc::strong_count(&sound), 1);
        let weak = Rc::downgrade(&sound);
        drop(sound);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn purge_removes_only_dead_entries() {
        let mut cache = AssetCache::new();
        let keep = cache.get("keep");
        drop(cache.get("gone1"));
        drop(cache.get("gone2"));
        assert_eq!(cache.live_count(), 1);
        assert_eq!(cache.purge(), 2);
        assert_eq!(cache.purge(), 0);
        assert!(Rc::ptr_eq(&keep, &cache.get("keep")));
        assert_eq!(cache.loads(), 3);
    }

    #[test]
    fn many_names() {
        let mut cache = AssetCache::new();
        let held: Vec<Rc<Asset>> = (0..1_000)
            .map(|i| cache.get(&format!("a{}", i % 100)))
            .collect();
        assert_eq!(cache.loads(), 100);
        assert_eq!(cache.live_count(), 100);
        assert_eq!(Rc::strong_count(&held[0]), 10);
        drop(held);
        assert_eq!(cache.live_count(), 0);
    }
}
