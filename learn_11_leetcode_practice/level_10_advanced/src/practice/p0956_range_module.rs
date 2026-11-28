// ==================================================================
// PRACTICE 0956: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0956_range_module.md
// Run the tests:          cargo test practice::p0956
// ==================================================================

//! # Problem 0956: Range Module
//!
//! - **Difficulty:** Hard
//! - **Topics:** half-open intervals in a `BTreeMap`, merging on insert, splitting on removal, collecting keys before mutating
//! - **Source:** [LeetCode 715. Range Module](https://leetcode.com/problems/range-module/)
//!
//! ## Problem
//!
//! Track a set of real numbers as half-open ranges `[left, right)`.
//!
//! Implement the `RangeModule` struct:
//!
//! - `RangeModule::new() -> Self`: nothing is tracked.
//! - `add_range(&mut self, left: i32, right: i32)`: start tracking every number in `[left, right)`.
//! - `query_range(&self, left: i32, right: i32) -> bool`: `true` if **every** number in
//!   `[left, right)` is currently tracked.
//! - `remove_range(&mut self, left: i32, right: i32)`: stop tracking every number in `[left, right)`.
//!
//! ## Examples
//!
//! ```text
//! add_range(10, 20)
//! remove_range(14, 16)
//! query_range(10, 14)   => true
//! query_range(13, 15)   => false   (14..15 was removed)
//! query_range(16, 17)   => true
//! ```
//!
//! ## Constraints
//!
//! - `1 <= left < right <= 10^9`
//! - At most `10^4` calls in total.
//!
//! ## Hints
//!
//! 1. Keep disjoint, non-touching intervals in a `BTreeMap<start, end>`.
//! 2. `add_range`: every stored interval with `start <= right` and `end >= left` overlaps or touches
//!    the new one; remove them all and insert their union.
//! 3. `remove_range`: every stored interval with `start < right` and `end > left` is affected; remove
//!    it and put back the parts that stick out on the left (`[start, left)`) and on the right
//!    (`[right, end)`).

use std::collections::BTreeMap;

pub struct RangeModule {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl RangeModule {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn add_range(&mut self, left: i32, right: i32) {
        todo!("Write your solution here")
    }

    pub fn query_range(&self, left: i32, right: i32) -> bool {
        todo!("Write your solution here")
    }

    pub fn remove_range(&mut self, left: i32, right: i32) {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_sequence() {
        let mut m = RangeModule::new();
        m.add_range(10, 20);
        m.remove_range(14, 16);
        assert!(m.query_range(10, 14));
        assert!(!m.query_range(13, 15));
        assert!(m.query_range(16, 17));
    }

    #[test]
    fn touching_ranges_merge() {
        let mut m = RangeModule::new();
        m.add_range(1, 5);
        m.add_range(5, 10);
        assert!(m.query_range(1, 10));
        m.add_range(20, 30);
        m.add_range(8, 25);
        assert!(m.query_range(1, 30));
        assert!(!m.query_range(0, 2));
        assert!(!m.query_range(29, 31));
    }

    #[test]
    fn removal_splits_and_deletes() {
        let mut m = RangeModule::new();
        m.add_range(1, 100);
        m.remove_range(10, 20);
        m.remove_range(30, 40);
        assert!(m.query_range(1, 10));
        assert!(m.query_range(20, 30));
        assert!(!m.query_range(9, 11));
        m.remove_range(1, 100);
        assert!(!m.query_range(50, 51));
        m.remove_range(5, 6);
        assert!(!m.query_range(5, 6));
    }

    #[test]
    fn matches_a_bitmap_model() {
        let mut m = RangeModule::new();
        let mut model = [false; 200];
        for step in 0..600_usize {
            let a = (step * 37) % 190 + 1;
            let b = a + (step * 11) % 9 + 1;
            if step % 3 == 2 {
                m.remove_range(a as i32, b as i32);
                model[a..b].iter_mut().for_each(|x| *x = false);
            } else {
                m.add_range(a as i32, b as i32);
                model[a..b].iter_mut().for_each(|x| *x = true);
            }
            let (qa, qb) = ((step * 7) % 190 + 1, (step * 7) % 190 + 1 + step % 5 + 1);
            assert_eq!(
                m.query_range(qa as i32, qb as i32),
                model[qa..qb].iter().all(|&x| x),
                "step {step}: query [{qa}, {qb})"
            );
        }
    }

    #[test]
    fn huge_coordinates() {
        let mut m = RangeModule::new();
        m.add_range(1, 1_000_000_000);
        m.remove_range(500_000_000, 500_000_001);
        assert!(m.query_range(1, 500_000_000));
        assert!(!m.query_range(1, 1_000_000_000));
        assert!(m.query_range(500_000_001, 1_000_000_000));
    }
}
