//! # Problem 0237: Design an Ordered Stream
//!
//! - **Difficulty:** Easy
//! - **Topics:** design struct, `Vec<Option<String>>`, `while let`, `Option::take`, `and_then`
//! - **Source:** [LeetCode 1656. Design an Ordered Stream](https://leetcode.com/problems/design-an-ordered-stream/)
//!
//! ## Problem
//!
//! `n` key-value pairs `(id_key, value)` arrive in **arbitrary order**. The keys are exactly the
//! integers `1` to `n`, each appearing once, and values are strings. The stream must hand the
//! values out **in key order**, as early as possible.
//!
//! Implement the `OrderedStream` struct:
//!
//! - `OrderedStream::new(n: i32) -> Self`: a stream for keys `1..=n`;
//! - `insert(&mut self, id_key: i32, value: String) -> Vec<String>`: store the pair, then return the
//!   longest run of values that can now be released: starting at the smallest key not yet
//!   released, take consecutive keys as long as they have arrived. Return an empty vector if the
//!   next key to release is still missing.
//!
//! Concatenating all the returned vectors gives the values sorted by key. This is a LeetCode
//! *design* problem, so there is no `impl Solution`; the template declares `insert(&self, ...)`
//! and invites you to change it to `&mut self`, as done here.
//!
//! ## Examples
//!
//! ```text
//! let mut s = OrderedStream::new(5);
//! s.insert(3, "ccccc") => []                         (key 1 has not arrived)
//! s.insert(1, "aaaaa") => ["aaaaa"]                  (key 2 is missing)
//! s.insert(2, "bbbbb") => ["bbbbb", "ccccc"]         (2 and the waiting 3)
//! s.insert(5, "eeeee") => []
//! s.insert(4, "ddddd") => ["ddddd", "eeeee"]
//! ```
//!
//! ## Constraints
//!
//! - `1 <= n <= 1000`, `1 <= id_key <= n`, every key is inserted exactly once.
//! - `value.len() == 5`, lowercase letters.
//!
//! ## Hints
//!
//! 1. Store slots as `Vec<Option<String>>` with index `id_key - 1`, plus a cursor `ptr` to the next
//!    key to release.
//! 2. `while let Some(v) = <take the value at ptr> { ... }` loops exactly as long as values are ready.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** keep the values in a vector of slots and a cursor. After storing a value, release
//! values from the cursor onward while the slot is filled, moving each value out with `take()`.
//!
//! **Complexity:** `O(n)` total over all inserts (each value is released once); `O(n)` space.
//!
//! **Rust concepts:**
//! - `self.slots.get_mut(self.ptr).and_then(Option::take)` is `None` both when the cursor is past
//!   the end and when the slot is empty, so one `while let` condition covers both stops.
//! - `take()` moves the `String` out of the slot without cloning; the slot becomes `None`, which is
//!   fine because a released key is never needed again.
//! - Converting the 1-based `i32` key to a 0-based `usize` index needs a cast; the constraints make
//!   it safe.
// @solution-end

/// Releases inserted values in key order.
pub struct OrderedStream {
    // @solution-start remove
    slots: Vec<Option<String>>,
    ptr: usize,
    // @solution-end
}

impl OrderedStream {
    /// A stream for keys `1..=n`.
    pub fn new(n: i32) -> Self {
        // @solution-start
        Self {
            slots: vec![None; n as usize],
            ptr: 0,
        }
        // @solution-end
    }

    /// Stores a pair and returns the values that are now ready, in key order.
    pub fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        // @solution-start
        // The constraints guarantee 1 <= id_key <= n.
        if let Some(slot) = self.slots.get_mut(id_key as usize - 1) {
            *slot = Some(value);
        }
        let mut ready = Vec::new();
        while let Some(v) = self.slots.get_mut(self.ptr).and_then(Option::take) {
            ready.push(v);
            self.ptr += 1;
        }
        ready
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut s = OrderedStream::new(5);
        assert!(s.insert(3, "ccccc".into()).is_empty());
        assert_eq!(s.insert(1, "aaaaa".into()), vec!["aaaaa"]);
        assert_eq!(s.insert(2, "bbbbb".into()), vec!["bbbbb", "ccccc"]);
        assert!(s.insert(5, "eeeee".into()).is_empty());
        assert_eq!(s.insert(4, "ddddd".into()), vec!["ddddd", "eeeee"]);
    }

    #[test]
    fn in_order_releases_one_by_one() {
        let mut s = OrderedStream::new(3);
        assert_eq!(s.insert(1, "a".into()), vec!["a"]);
        assert_eq!(s.insert(2, "b".into()), vec!["b"]);
        assert_eq!(s.insert(3, "c".into()), vec!["c"]);
    }

    #[test]
    fn single_key() {
        let mut s = OrderedStream::new(1);
        assert_eq!(s.insert(1, "zzzzz".into()), vec!["zzzzz"]);
    }

    #[test]
    fn reverse_order_releases_everything_at_the_end() {
        let n = 1000;
        let mut s = OrderedStream::new(n);
        for key in (2..=n).rev() {
            assert!(s.insert(key, format!("v{key}")).is_empty());
        }
        let all = s.insert(1, "v1".into());
        assert_eq!(all.len(), 1000);
        assert_eq!(all[0], "v1");
        assert_eq!(all[999], "v1000");
    }
}
