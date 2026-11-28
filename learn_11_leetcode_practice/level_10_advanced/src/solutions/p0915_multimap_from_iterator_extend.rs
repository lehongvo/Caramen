//! # Problem 0915: `FromIterator` and `Extend` for a MultiMap
//!
//! - **Difficulty:** Hard
//! - **Topics:** `FromIterator`, `Extend` (owned and `&`-items), `IntoIterator` for `T`/`&T`, `Borrow`, naming iterator types with `fn` pointers
//! - **Source:** Original · [std: `FromIterator`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html) · [std: `Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html) · [std: `Borrow`](https://doc.rust-lang.org/std/borrow/trait.Borrow.html)
//!
//! ## Problem
//!
//! A **multimap** maps each key to a list of values. Make yours feel like a standard collection:
//! `collect()` into it, `extend()` it, and loop over it with `for`.
//!
//! `MultiMap<K, V>` wraps a `BTreeMap<K, Vec<V>>` (keys in sorted order, values in insertion order)
//! and a running count of values. With `K: Ord`:
//!
//! - `new()` (and `Default`), `insert(&mut self, key: K, value: V)`.
//! - `get<Q>(&self, key: &Q) -> &[V]` where `K: Borrow<Q>, Q: Ord + ?Sized`: the values of `key`, or
//!   an **empty slice** if it is absent. Thanks to `Borrow`, a `MultiMap<String, _>` can be queried
//!   with a `&str`.
//! - `remove<Q>(&mut self, key: &Q) -> Vec<V>` (same bounds): removes and returns all values of `key`
//!   (empty `Vec` if absent).
//! - `len()` (the total number of **values**), `key_count()`, `is_empty()`.
//!
//! Trait implementations:
//!
//! - `FromIterator<(K, V)>`, so `pairs.into_iter().collect::<MultiMap<_, _>>()` works.
//! - `Extend<(K, V)>`, and `Extend<(&'a K, &'a V)>` when `K: Copy, V: Copy` (like std's maps).
//! - `IntoIterator for MultiMap<K, V>` with `Item = (K, Vec<V>)`, keys in order.
//! - `IntoIterator for &'a MultiMap<K, V>` with `Item = (&'a K, &'a [V])`, keys in order. Use
//!   `type IntoIter = std::iter::Map<btree_map::Iter<'a, K, Vec<V>>, ToSlice<'a, K, V>>;` where the
//!   given alias `ToSlice<'a, K, V>` is the function pointer type `fn((&'a K, &'a Vec<V>)) -> (&'a K, &'a [V])`.
//!
//! Finally, `group_by_length(words: &[&str]) -> MultiMap<usize, String>` collects every word under
//! its length in bytes.
//!
//! ## Examples
//!
//! ```text
//! let m: MultiMap<&str, i32> = [("a", 1), ("b", 2), ("a", 3)].into_iter().collect();
//! m.get("a")     => [1, 3]
//! m.get("zzz")   => []
//! m.len()        => 3,  m.key_count() => 2
//! group_by_length(&["hi", "yo", "hey"]).get(&2)  => ["hi", "yo"]
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `FromIterator::from_iter` can create an empty map and call your own `extend`.
//! 2. `Extend::extend` takes `I: IntoIterator<Item = (K, V)>`; loop over it and `insert`.
//! 3. A non-capturing closure coerces to a `fn` pointer, which is what lets you *name* the `Map` type.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** `insert` pushes onto `entry(key).or_default()` and bumps the counter; every trait impl
//! is a thin layer over it. `from_iter` builds an empty map and extends it; the `&`-item `Extend`
//! copies the pair and reuses the owned one. The borrowing `IntoIterator` maps each
//! `(&K, &Vec<V>)` to `(&K, &[V])` with a plain function, so the iterator type can be written out.
//!
//! **Complexity:** `insert`, `get` and `remove` are `O(log k)` for `k` keys; iteration is linear.
//!
//! **Rust concepts:**
//! - `collect()` calls `FromIterator::from_iter`; implementing it is all that is needed to make a
//!   type a `collect` target.
//! - `Extend<(&'a K, &'a V)>` lets callers extend from an iterator of references without cloning by
//!   hand.
//! - `K: Borrow<Q>` with `Q: ?Sized` is the same trick `HashMap::get` uses so `String` keys can be
//!   looked up with `&str`.
//! - Closures have unnameable types, but a non-capturing closure (or a `fn` item) coerces to a `fn`
//!   pointer type, which can appear in an associated type.
// @solution-end

use std::borrow::Borrow;
use std::collections::{BTreeMap, btree_map};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiMap<K, V> {
    map: BTreeMap<K, Vec<V>>,
    len: usize,
}

impl<K: Ord, V> Default for MultiMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> MultiMap<K, V> {
    pub fn new() -> Self {
        // @solution-start
        MultiMap {
            map: BTreeMap::new(),
            len: 0,
        }
        // @solution-end
    }

    pub fn insert(&mut self, key: K, value: V) {
        // @solution-start
        self.map.entry(key).or_default().push(value);
        self.len += 1;
        // @solution-end
    }

    pub fn get<Q>(&self, key: &Q) -> &[V]
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        // @solution-start
        self.map.get(key).map(Vec::as_slice).unwrap_or(&[])
        // @solution-end
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Vec<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        // @solution-start
        let values = self.map.remove(key).unwrap_or_default();
        self.len -= values.len();
        values
        // @solution-end
    }

    pub fn len(&self) -> usize {
        // @solution-start
        self.len
        // @solution-end
    }

    pub fn key_count(&self) -> usize {
        // @solution-start
        self.map.len()
        // @solution-end
    }

    pub fn is_empty(&self) -> bool {
        // @solution-start
        self.len == 0
        // @solution-end
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for MultiMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        // @solution-start
        let mut map = MultiMap::new();
        map.extend(iter);
        map
        // @solution-end
    }
}

impl<K: Ord, V> Extend<(K, V)> for MultiMap<K, V> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        // @solution-start
        for (key, value) in iter {
            self.insert(key, value);
        }
        // @solution-end
    }
}

impl<'a, K: Ord + Copy, V: Copy> Extend<(&'a K, &'a V)> for MultiMap<K, V> {
    fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: I) {
        // @solution-start
        self.extend(iter.into_iter().map(|(&key, &value)| (key, value)));
        // @solution-end
    }
}

impl<K, V> IntoIterator for MultiMap<K, V> {
    type Item = (K, Vec<V>);
    type IntoIter = btree_map::IntoIter<K, Vec<V>>;

    fn into_iter(self) -> Self::IntoIter {
        // @solution-start
        self.map.into_iter()
        // @solution-end
    }
}

/// The function pointer type used by the borrowing iterator: `(&K, &Vec<V>) -> (&K, &[V])`.
pub type ToSlice<'a, K, V> = fn((&'a K, &'a Vec<V>)) -> (&'a K, &'a [V]);

impl<'a, K, V> IntoIterator for &'a MultiMap<K, V> {
    type Item = (&'a K, &'a [V]);
    type IntoIter = std::iter::Map<btree_map::Iter<'a, K, Vec<V>>, ToSlice<'a, K, V>>;

    fn into_iter(self) -> Self::IntoIter {
        // @solution-start
        // The annotation coerces the non-capturing closure to the `fn` pointer type named above.
        let to_slice: ToSlice<'a, K, V> = |(key, values)| (key, values.as_slice());
        self.map.iter().map(to_slice)
        // @solution-end
    }
}

pub fn group_by_length(words: &[&str]) -> MultiMap<usize, String> {
    // @solution-start
    words
        .iter()
        .map(|word| (word.len(), word.to_string()))
        .collect()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collect_and_query() {
        let m: MultiMap<&str, i32> = [("a", 1), ("b", 2), ("a", 3)].into_iter().collect();
        assert_eq!(m.get("a"), &[1, 3]);
        assert_eq!(m.get("b"), &[2]);
        assert!(m.get("zzz").is_empty());
        assert_eq!((m.len(), m.key_count(), m.is_empty()), (3, 2, false));
    }

    #[test]
    fn string_keys_are_queried_with_str() {
        let mut m: MultiMap<String, u8> = MultiMap::default();
        m.insert("x".to_string(), 1);
        m.insert("x".to_string(), 2);
        assert_eq!(m.get("x"), &[1, 2]);
        assert_eq!(m.remove("x"), vec![1, 2]);
        assert_eq!(m.remove("x"), Vec::<u8>::new());
        assert!(m.is_empty());
        assert_eq!(m.key_count(), 0);
    }

    #[test]
    fn extend_with_owned_and_borrowed_pairs() {
        let mut m = MultiMap::new();
        m.extend(vec![(1, 'a'), (2, 'b')]);
        let more = [(1, 'c'), (3, 'd')];
        m.extend(more.iter().map(|(k, v)| (k, v)));
        assert_eq!(m.get(&1), &['a', 'c']);
        assert_eq!(m.len(), 4);
        let owned: Vec<(i32, Vec<char>)> = m.into_iter().collect();
        assert_eq!(
            owned,
            vec![(1, vec!['a', 'c']), (2, vec!['b']), (3, vec!['d'])]
        );
    }

    #[test]
    fn borrowing_for_loop() {
        let m: MultiMap<char, u32> = [('b', 2), ('a', 1), ('b', 3)].into_iter().collect();
        let mut seen = Vec::new();
        for (key, values) in &m {
            seen.push(format!("{key}:{values:?}"));
        }
        assert_eq!(seen, vec!["a:[1]", "b:[2, 3]"]);
    }

    #[test]
    fn group_words_by_length() {
        let groups = group_by_length(&["hi", "yo", "hey", "", "rust", "go"]);
        assert_eq!(groups.get(&2), &["hi", "yo", "go"]);
        assert_eq!(groups.get(&0), &[""]);
        assert_eq!(groups.key_count(), 4);
        let many: Vec<String> = (0..5_000).map(|i| i.to_string()).collect();
        let refs: Vec<&str> = many.iter().map(String::as_str).collect();
        let by_len = group_by_length(&refs);
        assert_eq!(by_len.get(&4).len(), 4_000);
        assert_eq!(by_len.len(), 5_000);
    }
}
