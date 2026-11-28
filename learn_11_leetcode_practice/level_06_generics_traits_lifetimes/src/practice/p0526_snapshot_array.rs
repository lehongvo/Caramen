// ==================================================================
// PRACTICE 0526: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0526_snapshot_array.md
// Run the tests:          cargo test practice::p0526
// ==================================================================

//! # Problem 0526: Snapshot Array
//!
//! - **Difficulty:** Medium
//! - **Topics:** design struct, per-index history `Vec<Vec<(i32, i32)>>`, `partition_point`, `&self` queries
//! - **Source:** [LeetCode 1146. Snapshot Array](https://leetcode.com/problems/snapshot-array/)
//!
//! ## Problem
//!
//! An array of `length` integers, all starting at `0`, that can take **snapshots** of itself.
//!
//! - `SnapshotArray::new(length: i32) -> Self`
//! - `set(&mut self, index: i32, val: i32)`: sets the element at `index` to `val`.
//! - `snap(&mut self) -> i32`: takes a snapshot and returns its id. Ids are `0, 1, 2, ...` (the number
//!   of earlier `snap` calls).
//! - `get(&self, index: i32, snap_id: i32) -> i32`: the value that `index` had when snapshot
//!   `snap_id` was taken.
//!
//! Copying the whole array on every `snap` is too slow: store, for each index, only the changes.
//! This is a LeetCode *design* problem, so there is no `impl Solution`: paste the struct and its
//! `impl SnapshotArray` block into LeetCode.
//!
//! ## Examples
//!
//! ```text
//! let mut a = SnapshotArray::new(3);
//! a.set(0, 5);
//! a.snap()    => 0
//! a.set(0, 6);
//! a.get(0, 0) => 5     (index 0 was 5 when snapshot 0 was taken)
//! a.get(1, 0) => 0     (index 1 was never set)
//! ```
//!
//! ## Constraints
//!
//! - `1 <= length <= 5 * 10^4`
//! - `0 <= index < length`, `0 <= val <= 10^9`
//! - `0 <= snap_id <` the number of `snap` calls so far
//! - At most `5 * 10^4` calls to `set`, `snap` and `get` in total.
//!
//! ## Hints
//!
//! 1. For each index keep a list of `(snap_id, value)` pairs, sorted by `snap_id`: "from snapshot
//!    `snap_id` on, the value is `value`".
//! 2. Several `set`s between two snapshots only need the last one: overwrite the last pair when its
//!    `snap_id` equals the current one.
//! 3. To answer `get`, binary search for the last pair whose `snap_id <= snap_id`.

/// An array that remembers its values at every snapshot.
pub struct SnapshotArray {}

impl SnapshotArray {
    pub fn new(length: i32) -> Self {
        todo!("Write your solution here")
    }

    pub fn set(&mut self, index: i32, val: i32) {
        todo!("Write your solution here")
    }

    pub fn snap(&mut self) -> i32 {
        todo!("Write your solution here")
    }

    pub fn get(&self, index: i32, snap_id: i32) -> i32 {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut a = SnapshotArray::new(3);
        a.set(0, 5);
        assert_eq!(a.snap(), 0);
        a.set(0, 6);
        assert_eq!(a.get(0, 0), 5);
        assert_eq!(a.get(1, 0), 0);
    }

    #[test]
    fn unchanged_index_keeps_old_value() {
        let mut a = SnapshotArray::new(2);
        a.set(1, 7);
        assert_eq!(a.snap(), 0);
        assert_eq!(a.snap(), 1);
        a.set(1, 8);
        assert_eq!(a.snap(), 2);
        assert_eq!(a.get(1, 0), 7);
        assert_eq!(a.get(1, 1), 7);
        assert_eq!(a.get(1, 2), 8);
        assert_eq!(a.get(0, 2), 0);
    }

    #[test]
    fn several_sets_between_snapshots() {
        let mut a = SnapshotArray::new(1);
        a.set(0, 1);
        a.set(0, 2);
        a.set(0, 3);
        assert_eq!(a.snap(), 0);
        a.set(0, 4);
        a.set(0, 0);
        assert_eq!(a.snap(), 1);
        assert_eq!(a.get(0, 0), 3);
        assert_eq!(a.get(0, 1), 0);
    }

    #[test]
    fn many_snapshots_match_a_model() {
        let length = 50;
        let mut a = SnapshotArray::new(length);
        let mut model = vec![0; length as usize];
        let mut snapshots: Vec<Vec<i32>> = Vec::new();
        for step in 0..20_000 {
            if step % 5 == 4 {
                assert_eq!(a.snap(), snapshots.len() as i32);
                snapshots.push(model.clone());
            } else {
                let index = (step * 7) % length;
                a.set(index, step);
                model[index as usize] = step;
            }
        }
        for (id, snapshot) in snapshots.iter().enumerate().step_by(37) {
            for index in 0..length {
                assert_eq!(a.get(index, id as i32), snapshot[index as usize]);
            }
        }
    }
}
